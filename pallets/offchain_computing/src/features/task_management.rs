use crate::*;
use frame_support::pallet_prelude::*;
use sp_runtime::{
	traits::Zero,
	Saturating
};

impl<T: Config> Pallet<T> {
	pub fn do_create_task(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		task_id: &T::TaskId,
		owner: &T::AccountId,
		input_data: &Option<BoundedVec<u8, T::InputLimit>>,
		now: u64,
		expires_in: Option<u64>,
	) -> DispatchResult {
		let expires_in = expires_in.unwrap_or(T::DefaultTaskExpiresIn::get());
		ensure!(expires_in >= T::MinTaskExpiresIn::get(), Error::<T>::ExpiresInTooSmall);
		ensure!(expires_in <= T::MaxTaskExpiresIn::get(), Error::<T>::ExpiresInTooLarge);

		ensure!(!Tasks::<T>::contains_key(&pool_info.id, task_id), Error::<T>::TaskIdTaken);

		let input_deposit = T::DepositPerByte::get()
			.saturating_mul(((input_data.as_ref().map(|x| x.len()).unwrap_or_default()) as u32).into());
		let task_deposit = T::CreateTaskDeposit::get();
		let deposit = input_deposit.saturating_add(task_deposit);
		T::Currency::reserve(owner, deposit)?;

		let expires_at = now + expires_in;
		let task = TaskInfo::<T::TaskId, T::AccountId, BalanceOf<T>> {
			id: task_id.clone(),
			creator: owner.clone(),
			owner: owner.clone(),
			owner_deposit: task_deposit,
			status: TaskStatus::Pending,
			result: None,
			expires_at,
			created_by: owner.clone(),
			created_at: now,
			assignee: None,
			assigned_at: None,
			processing_at: None,
			processed_at: None,
		};
		Tasks::<T>::insert(&pool_info.id, task_id, task);

		if let Some(input_data) = input_data {
			let input = ChainStoredData::<T::AccountId, BalanceOf<T>, T::InputLimit> {
				depositor: owner.clone(),
				actual_deposit: input_deposit,
				surplus_deposit: Zero::zero(),
				data: input_data.clone(),
			};
			TaskInputs::<T>::insert(&pool_info.id, task_id, input);
		}

		let mut new_pool_info = pool_info.clone();
		new_pool_info.tasks_count += 1;

		AssignableTasks::<T>::insert(&pool_info.id, task_id, ());
		AccountOwnedTasks::<T>::insert((owner, pool_info.id, task_id), ());

		Self::deposit_event(Event::TaskCreated { owner: owner.clone(), pool_id: pool_info.id, task_id: *task_id, input: input_data.clone() });
		Ok(())
	}

	pub fn do_destroy_task(
		who: &T::AccountId,
		pool_id: &T::PoolId,
		task_id: &T::TaskId
	) -> DispatchResult {
		let task = Tasks::<T>::get(&pool_id, task_id).ok_or(Error::<T>::TaskNotFound)?;
		Self::ensure_task_owner(who, &task)?;
		ensure!(
			match task.status {
				TaskStatus::Pending | TaskStatus::Processed => true,
				_ => false
			},
			Error::<T>::TaskIsProcessing
		);

		Self::do_actual_destroy_task(pool_id, &task, who)?;

		Ok(())
	}

	pub fn do_destroy_expired_task(
		pool_id: &T::PoolId,
		task_id: &T::TaskId,
		destroyer: &T::AccountId,
		now: u64
	) -> DispatchResult {
		let task = Tasks::<T>::get(&pool_id, task_id).ok_or(Error::<T>::TaskNotFound)?;
		Self::ensure_task_expired(&task, now)?;

		Self::do_actual_destroy_task(pool_id, &task, destroyer)?;

		Ok(())
	}

	fn do_actual_destroy_task(
		pool_id: &T::PoolId,
		task: &TaskInfo::<T::TaskId, T::AccountId, BalanceOf<T>>,
		destroyer: &T::AccountId
	) -> DispatchResult {
		let task_id = task.id;

		T::Currency::unreserve(&task.owner, task.owner_deposit);
		if let Some(input_entry) = TaskInputs::<T>::take(pool_id, task_id) {
			let deposit = input_entry.actual_deposit.saturating_add(input_entry.surplus_deposit);
			T::Currency::unreserve(&input_entry.depositor, deposit);
		}
		if let Some(output_entry) = TaskOutputs::<T>::take(pool_id, task_id) {
			let deposit = output_entry.actual_deposit.saturating_add(output_entry.surplus_deposit);
			T::Currency::unreserve(&output_entry.depositor, deposit);
		}
		if let Some(proof_entry) = TaskProofs::<T>::take(pool_id, task_id) {
			let deposit = proof_entry.actual_deposit.saturating_add(proof_entry.surplus_deposit);
			T::Currency::unreserve(&proof_entry.depositor, deposit);
		}

		Tasks::<T>::remove(pool_id, task_id);

		Pools::<T>::try_mutate_exists(pool_id, |pool_info| -> Result<(), DispatchError> {
			let Some(pool_info) = pool_info else {
				return Err(Error::<T>::PoolNotFound.into())
			};

			pool_info.tasks_count -= 1;

			Ok(())
		})?;

		if let Some(worker) = &task.assignee {
			WorkerAssignedTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
				*counter -= 1;
				Ok(())
			})?;
		}
		AccountOwnedTasks::<T>::remove((&task.owner, pool_id, task_id));
		if task.status == TaskStatus::Pending {
			AssignableTasks::<T>::remove(pool_id, task_id);
		}

		Self::deposit_event(Event::TaskDestroyed { pool_id: *pool_id, task_id, destroyer: destroyer.clone() });
		Ok(())
	}
}
