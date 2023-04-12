use crate::*;
use frame_support::pallet_prelude::*;
use sp_runtime::{
	traits::Zero,
	Saturating
};

impl<T: Config> Pallet<T> {
	pub(crate) fn do_create_task(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		task_id: T::TaskId,
		policy_id: T::PolicyId,
		owner: T::AccountId,
		depositor: T::AccountId,
		impl_spec_version: ImplSpecVersion,
		input_data: Option<BoundedVec<u8, T::InputLimit>>,
		now: u64,
		expires_in: Option<u64>,
	) -> DispatchResult {
		let expires_in = expires_in.unwrap_or(T::DefaultTaskExpiresIn::get());
		ensure!(expires_in >= T::MinTaskExpiresIn::get(), Error::<T>::ExpiresInTooSmall);
		ensure!(expires_in <= T::MaxTaskExpiresIn::get(), Error::<T>::ExpiresInTooLarge);

		ensure!(!Tasks::<T>::contains_key(&pool_info.id, &task_id), Error::<T>::TaskIdTaken);

		let input_deposit = T::DepositPerByte::get()
			.saturating_mul(((input_data.as_ref().map(|x| x.len()).unwrap_or_default()) as u32).into());
		let task_deposit = T::CreatingTaskDeposit::get();

		let total_deposit = input_deposit.saturating_add(task_deposit);
		T::Currency::reserve(&depositor, total_deposit)?;

		let expires_at = now + expires_in;
		let task = TaskInfo::<T::TaskId, T::AccountId, BalanceOf<T>, ImplSpecVersion> {
			id: task_id.clone(),
			owner: owner.clone(),
			depositor: depositor.clone(),
			deposit: task_deposit,
			impl_spec_version,
			status: TaskStatus::Pending,
			result: None,
			expires_at,
			created_at: now,
			assignee: None,
			assigned_at: None,
			processing_at: None,
			processed_at: None,
		};
		Tasks::<T>::insert(&pool_info.id, &task_id, task);

		if let Some(input_data) = input_data.clone() {
			let input = ChainStoredData::<T::AccountId, BalanceOf<T>, T::InputLimit> {
				depositor: depositor.clone(),
				actual_deposit: input_deposit,
				surplus_deposit: Zero::zero(),
				data: input_data.clone(),
			};
			TaskInputs::<T>::insert(&pool_info.id, &task_id, input);
		}

		let mut new_pool_info = pool_info.clone();
		new_pool_info.tasks_count += 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		AssignableTasks::<T>::insert((pool_info.id.clone(), impl_spec_version.clone(), task_id.clone()), ());
		AccountOwningTasks::<T>::insert((owner.clone(), pool_info.id.clone(), task_id.clone()), ());

		Self::deposit_event(Event::TaskCreated {
			pool_id: pool_info.id,
			task_id, policy_id,
			owner: owner.clone(),
			impl_spec_version,
			input: input_data,
			expires_in
		});
		Ok(())
	}

	pub(crate) fn do_destroy_task(
		who: T::AccountId,
		pool_id: T::PoolId,
		task_id: T::TaskId
	) -> DispatchResult {
		let task = Tasks::<T>::get(&pool_id, &task_id).ok_or(Error::<T>::TaskNotFound)?;
		Self::ensure_task_owner(&who, &task)?;
		ensure!(
			match task.status {
				TaskStatus::Pending | TaskStatus::Processed => true,
				_ => false
			},
			Error::<T>::TaskIsProcessing
		);

		Self::do_actual_destroy_task(pool_id, task, who)
	}

	pub(crate) fn do_destroy_expired_task(
		pool_id: T::PoolId,
		task_id: T::TaskId,
		destroyer: T::AccountId,
		now: u64
	) -> DispatchResult {
		let task = Tasks::<T>::get(&pool_id, &task_id).ok_or(Error::<T>::TaskNotFound)?;
		Self::ensure_task_expired(&task, now)?;

		Self::do_actual_destroy_task(pool_id, task, destroyer)?;

		Ok(())
	}

	fn do_actual_destroy_task(
		pool_id: T::PoolId,
		task: TaskInfo<T::TaskId, T::AccountId, BalanceOf<T>, ImplSpecVersion>,
		destroyer: T::AccountId
	) -> DispatchResult {
		let task_id = task.id;

		T::Currency::unreserve(&task.depositor, task.deposit);
		if let Some(input_entry) = TaskInputs::<T>::take(&pool_id, &task_id) {
			let deposit = input_entry.actual_deposit.saturating_add(input_entry.surplus_deposit);
			T::Currency::unreserve(&input_entry.depositor, deposit);
		}
		if let Some(output_entry) = TaskOutputs::<T>::take(&pool_id, &task_id) {
			let deposit = output_entry.actual_deposit.saturating_add(output_entry.surplus_deposit);
			T::Currency::unreserve(&output_entry.depositor, deposit);
		}
		if let Some(proof_entry) = TaskProofs::<T>::take(&pool_id, &task_id) {
			let deposit = proof_entry.actual_deposit.saturating_add(proof_entry.surplus_deposit);
			T::Currency::unreserve(&proof_entry.depositor, deposit);
		}

		Tasks::<T>::remove(&pool_id, &task_id);

		Pools::<T>::try_mutate_exists(&pool_id, |pool_info| -> Result<(), DispatchError> {
			let Some(pool_info) = pool_info else {
				return Err(Error::<T>::PoolNotFound.into())
			};

			pool_info.tasks_count -= 1;

			Ok(())
		})?;

		if task.status == TaskStatus::Pending {
			AssignableTasks::<T>::remove((pool_id.clone(), task.impl_spec_version.clone(), task_id.clone()));
		} else if task.status == TaskStatus::Processing {
			if let Some(worker) = &task.assignee {
				WorkerAssignedTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
					*counter -= 1;
					Ok(())
				})?;
			}
		}
		AccountOwningTasks::<T>::remove((task.owner.clone(), pool_id.clone(), task_id.clone()));

		Self::deposit_event(Event::TaskDestroyed { pool_id, task_id, destroyer });
		Ok(())
	}
}
