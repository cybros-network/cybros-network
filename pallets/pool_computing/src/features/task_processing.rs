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
		current_block: T::BlockNumber,
	) -> DispatchResult {
		Self::ensure_worker_in_pool(pool_id, worker)?;
		WorkerTakenTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
			ensure!(
					*counter < T::MaxTakenTasksPerWorker::get(),
					Error::<T>::WorkerTakenItemsLimitExceeded
				);

			*counter += 1;
			Ok(())
		})?;

		Tasks::<T>::try_mutate_exists(&pool_id, task_id, |task| -> Result<(), DispatchError> {
			let Some(task) = task else {
				return Err(Error::<T>::TaskNotFound.into())
			};
			ensure!(task.taken_by.is_none(), Error::<T>::TaskTakenByOtherWorker);
			Self::ensure_task_not_expired(current_block, &task)?;

			task.taken_by = Some(worker.clone());
			task.taken_at = Some(current_block);
			task.status = TaskStatus::Processing;
			task.start_processing_at = Some(current_block);

			Ok(())
		})?;

		Ok(())
	}

	pub fn do_submit_task_result(
		pool_id: &T::PoolId,
		task_id: &T::TaskId,
		worker: &T::AccountId,
		current_block: T::BlockNumber,
		output_data: &Option<BoundedVec<u8, T::OutputLimit>>,
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
			Self::ensure_task_not_expired(current_block, &task)?;
			Self::ensure_task_taker(task, worker)?;

			task.status = TaskStatus::Processed;
			task.processed_at = Some(current_block);

			if let Some(output_data) = output_data {
				let output_deposit = T::DepositPerByte::get()
					.saturating_mul(((output_data.len()) as u32).into());
				let output_depositor = task.taken_by.clone().ok_or(Error::<T>::NoPermission)?;
				T::Currency::reserve(&output_depositor, output_deposit)?;

				let output = ChainStoredData::<T::AccountId, BalanceOf<T>, T::OutputLimit> {
					depositor: output_depositor,
					actual_deposit: output_deposit,
					surplus_deposit: Zero::zero(),
					data: output_data.clone(),
				};
				TaskOutputs::<T>::insert(pool_id, task_id, output);
			}

			Ok(())
		})?;

		WorkerTakenTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
			*counter -= 1;
			Ok(())
		})?;

		Ok(())
	}
}
