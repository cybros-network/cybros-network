use crate::*;
use frame_support::pallet_prelude::*;
use sp_runtime::{
	traits::Zero,
	Saturating
};

impl<T: Config> Pallet<T> {
	pub fn do_take_task(
		pool_id: &T::PoolId,
		task_id: &T::TaskId,
		worker: &T::AccountId,
		processing: bool,
		now: u64,
		expires_in: u64,
	) -> DispatchResult {
		Self::ensure_worker_in_pool(pool_id, worker)?;

		Tasks::<T>::try_mutate_exists(&pool_id, task_id, |task| -> Result<(), DispatchError> {
			let Some(task) = task else {
				return Err(Error::<T>::TaskNotFound.into())
			};
			Self::ensure_task_not_expired(&task, now)?;
			ensure!(
				task.taken_by.is_none() || (task.taken_by.is_some() && task.released_at.is_some()),
				Error::<T>::TaskAlreadyTaken
			);

			if let Some(scheduled_at) = task.scheduled_at {
				ensure!(
					now >= scheduled_at,
					Error::<T>::TaskTooEarlyToTake
				);
			}

			WorkerTakenTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
				ensure!(
					*counter <= T::MaxTakenTasksPerWorker::get(),
					Error::<T>::WorkerTakenTasksLimitExceeded
				);

				*counter += 1;
				Ok(())
			})?;

			task.expires_at = now + expires_in;
			task.taken_by = Some(worker.clone());
			task.taking_at = Some(now);
			task.released_at = None;
			if processing {
				task.status = TaskStatus::Processing;
				task.processing_at = Some(now);
			}

			Ok(())
		})?;

		Ok(())
	}

	pub fn do_release_task(
		pool_id: &T::PoolId,
		task_id: &T::TaskId,
		worker: &T::AccountId,
		now: u64,
		expires_in: u64,
	) -> DispatchResult {
		Self::ensure_worker_in_pool(pool_id, worker)?;

		Tasks::<T>::try_mutate_exists(&pool_id, task_id, |task| -> Result<(), DispatchError> {
			let Some(task) = task else {
				return Err(Error::<T>::TaskNotFound.into())
			};
			Self::ensure_task_not_expired(&task, now)?;
			ensure!(task.released_at.is_none(), Error::<T>::TaskAlreadyReleased);

			let taker = task.taken_by.as_ref().ok_or(Error::<T>::NoPermission)?;
			ensure!(worker == taker, Error::<T>::NoPermission);

			ensure!(
				match task.status {
					TaskStatus::Processing => false,
					_ => true
				},
				Error::<T>::TaskIsProcessing
			);

			task.expires_at = now + expires_in;
			task.released_at = Some(now);

			WorkerTakenTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
				*counter -= 1;
				Ok(())
			})?;

			Ok(())
		})?;

		Ok(())
	}

	pub fn do_submit_task_result(
		pool_id: &T::PoolId,
		task_id: &T::TaskId,
		worker: &T::AccountId,
		output_data: &Option<BoundedVec<u8, T::OutputLimit>>,
		proof_data: &Option<BoundedVec<u8, T::ProofLimit>>,
		release: bool,
		now: u64,
		expires_in: u64,
	) -> DispatchResult {
		Tasks::<T>::try_mutate_exists(&pool_id, task_id, |task| -> Result<(), DispatchError> {
			let Some(task) = task else {
				return Err(Error::<T>::TaskNotFound.into())
			};
			ensure!(
				match task.status {
					TaskStatus::Pending | TaskStatus::Processing => true,
					_ => false
				},
				Error::<T>::TaskIsProcessed
			);
			Self::ensure_task_not_expired(&task, now)?;
			Self::ensure_task_taker(task, worker)?;

			task.expires_at = now + expires_in;
			task.status = TaskStatus::Processed;
			task.processed_at = Some(now);
			if release {
				task.released_at = Some(now);
			}

			if let Some(output_data) = output_data {
				let deposit = T::DepositPerByte::get()
					.saturating_mul(((output_data.len()) as u32).into());
				let depositor = task.taken_by.clone().ok_or(Error::<T>::NoPermission)?;
				T::Currency::reserve(&depositor, deposit)?;

				let output_entry = ChainStoredData::<T::AccountId, BalanceOf<T>, T::OutputLimit> {
					depositor,
					actual_deposit: deposit,
					surplus_deposit: Zero::zero(),
					data: output_data.clone(),
				};
				TaskOutputs::<T>::insert(pool_id, task_id, output_entry);
			}
			if let Some(proof_data) = proof_data {
				let deposit = T::DepositPerByte::get()
					.saturating_mul(((proof_data.len()) as u32).into());
				let depositor = task.taken_by.clone().ok_or(Error::<T>::NoPermission)?;
				T::Currency::reserve(&depositor, deposit)?;

				let proof_entry = ChainStoredData::<T::AccountId, BalanceOf<T>, T::ProofLimit> {
					depositor,
					actual_deposit: deposit,
					surplus_deposit: Zero::zero(),
					data: proof_data.clone(),
				};
				TaskProofs::<T>::insert(pool_id, task_id, proof_entry);
			}

			Ok(())
		})?;

		if release {
			WorkerTakenTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
				*counter -= 1;
				Ok(())
			})?;
		}

		Ok(())
	}
}
