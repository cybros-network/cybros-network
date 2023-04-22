use crate::*;
use frame_support::pallet_prelude::*;
use sp_runtime::{
	traits::Zero,
	Saturating
};

impl<T: Config> Pallet<T> {
	pub(crate) fn do_assign_task(
		pool_id: T::PoolId,
		task_id: Option<T::TaskId>,
		worker: T::AccountId,
		now: u64,
		processing: bool,
		expires_in: u64,
	) -> DispatchResult {
		Self::ensure_worker_in_pool(&pool_id, &worker)?;
		let worker_info = T::OffchainWorkerManageable::worker_info(&worker).ok_or(Error::<T>::WorkerNotFound)?;
		let worker_impl_spec_version = worker_info.impl_spec_version.ok_or(Error::<T>::InternalError)?;

		let current_assigned_tasks_count = WorkerAssignedTasksCounter::<T>::get(&worker);
		ensure!(
			current_assigned_tasks_count < T::MaxAssignedTasksPerWorker::get(),
			Error::<T>::WorkerAssignedTasksLimitExceeded
		);

		// TODO: the current design has thundering herd problem, but it's OK for now.
		let mut task = 'block: {
			if let Some(task_id) = task_id {
				break 'block Tasks::<T>::get(&pool_id, &task_id).ok_or(Error::<T>::TaskNotFound)
			}

			let task_id = AssignableTasks::<T>::iter_key_prefix(
			(
					pool_id.clone(),
					worker_impl_spec_version.clone()
				)
			).next().ok_or(Error::<T>::NoAssignableTask)?;
			Tasks::<T>::get(&pool_id, &task_id).ok_or(Error::<T>::TaskNotFound)
		}?;
		ensure!(
			worker_impl_spec_version == task.impl_spec_version.clone(),
			Error::<T>::ImplMismatched
		);
		AssignableTasks::<T>::remove((pool_id.clone(), task.impl_spec_version.clone(), task.id.clone()));

		// It is possible to get a expired job, but actually it is a soft expiring
		// Comment this because current `expires_at` actually a soft expiring
		// Self::ensure_task_not_expired(&task, now)?;

		ensure!(
			task.assignee.is_none(),
			Error::<T>::TaskAlreadyAssigned
		);
		task.assignee = Some(worker.clone());
		task.assigned_at = Some(now);

		// task.expires_at = now + expires_in; // Not sure we need to expand expiring time
		if processing {
			task.status = TaskStatus::Processing;
			task.processing_at = Some(now);
			task.expires_at = now + expires_in;
		}

		let task_id = task.id.clone();
		WorkerAssignedTasksCounter::<T>::insert(&worker, current_assigned_tasks_count + 1);
		Tasks::<T>::insert(&pool_id, &task_id, task);

		Self::deposit_event(Event::TaskAssigned { pool_id: pool_id.clone(), task_id: task_id.clone(), assignee: worker });
		if processing {
			Self::deposit_event(Event::TaskStatusUpdated { pool_id, task_id, status: TaskStatus::Processing });
		}
		Ok(())
	}

	pub(crate) fn do_release_task(
		pool_id: T::PoolId,
		task_id: T::TaskId,
		worker: T::AccountId,
	) -> DispatchResult {
		Self::ensure_worker_in_pool(&pool_id, &worker)?;

		Tasks::<T>::try_mutate_exists(&pool_id, &task_id, |task| -> Result<(), DispatchError> {
			let Some(task) = task else {
				return Err(Error::<T>::TaskNotFound.into())
			};
			// Comment this because current `expires_at` actually a soft expiring
			// Self::ensure_task_not_expired(&task, now)?;

			let assignee = task.assignee.clone().ok_or(Error::<T>::NoPermission)?;
			ensure!(worker == assignee, Error::<T>::NoPermission);

			ensure!(
				match task.status {
					TaskStatus::Processing | TaskStatus::Processed => false,
					_ => true
				},
				Error::<T>::TaskAssigneeLocked
			);

			task.assignee = None;
			task.assigned_at = None;

			WorkerAssignedTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
				*counter -= 1;
				Ok(())
			})?;
			AssignableTasks::<T>::insert((pool_id.clone(), task.impl_spec_version.clone(), task_id.clone()), ());

			Ok(())
		})?;

		Self::deposit_event(Event::TaskReleased { pool_id, task_id });
		Ok(())
	}

	pub(crate) fn do_submit_task_result(
		pool_id: T::PoolId,
		task_id: T::TaskId,
		worker: T::AccountId,
		result: TaskResult,
		output_data: Option<BoundedVec<u8, T::OutputLimit>>,
		proof_data: Option<BoundedVec<u8, T::ProofLimit>>,
		now: u64,
		expires_in: u64,
	) -> DispatchResult {
		Tasks::<T>::try_mutate_exists(&pool_id, &task_id, |task| -> Result<(), DispatchError> {
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
			// Comment this because current `expires_at` actually a soft expiring
			// Self::ensure_task_not_expired(&task, now)?;
			Self::ensure_task_assignee(task, &worker)?;

			task.expires_at = now + expires_in;
			task.status = TaskStatus::Processed;
			task.result = Some(result.clone());
			task.ended_at = Some(now);

			if let Some(output_data) = output_data.clone() {
				let deposit = T::DepositPerByte::get()
					.saturating_mul(((output_data.len()) as u32).into());
				let depositor = task.assignee.clone().ok_or(Error::<T>::NoPermission)?;
				T::Currency::reserve(&depositor, deposit)?;

				let output_entry = ChainStoredData::<T::AccountId, BalanceOf<T>, T::OutputLimit> {
					depositor,
					actual_deposit: deposit,
					surplus_deposit: Zero::zero(),
					data: output_data.clone(),
				};
				TaskOutputs::<T>::insert(&pool_id, &task_id, output_entry);
			}
			if let Some(proof_data) = proof_data.clone() {
				let deposit = T::DepositPerByte::get()
					.saturating_mul(((proof_data.len()) as u32).into());
				let depositor = task.assignee.clone().ok_or(Error::<T>::NoPermission)?;
				T::Currency::reserve(&depositor, deposit)?;

				let proof_entry = ChainStoredData::<T::AccountId, BalanceOf<T>, T::ProofLimit> {
					depositor,
					actual_deposit: deposit,
					surplus_deposit: Zero::zero(),
					data: proof_data.clone(),
				};
				TaskProofs::<T>::insert(&pool_id, &task_id, proof_entry);
			}

			Ok(())
		})?;

		WorkerAssignedTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
			*counter -= 1;
			Ok(())
		})?;

		Self::deposit_event(Event::TaskResultUpdated { pool_id: pool_id.clone(), task_id: task_id.clone(), result, output: output_data, proof: proof_data });
		Self::deposit_event(Event::TaskStatusUpdated { pool_id, task_id, status: TaskStatus::Processed });

		Ok(())
	}
}
