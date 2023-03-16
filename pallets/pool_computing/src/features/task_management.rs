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
		current_block: T::BlockNumber,
		input_data: &Option<BoundedVec<u8, T::InputLimit>>,
	) -> DispatchResult {
		ensure!(!Tasks::<T>::contains_key(&pool_info.id, task_id), Error::<T>::TaskIdTaken);

		let input_deposit = T::DepositPerByte::get()
			.saturating_mul(((input_data.as_ref().map(|x| x.len()).unwrap_or_default()) as u32).into());
		let task_deposit = T::CreateTaskDeposit::get();
		let deposit = input_deposit.saturating_add(task_deposit);
		T::Currency::reserve(owner, deposit)?;

		let task = Task::<T::TaskId, T::AccountId, BalanceOf<T>, T::BlockNumber> {
			id: task_id.clone(),
			creator: owner.clone(),
			owner: owner.clone(),
			owner_deposit: task_deposit,
			status: TaskStatus::Pending,
			result: None,
			created_by: owner.clone(),
			created_at: current_block,
			taken_by: None,
			taking_at: None,
			released_at: None,
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
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Accounts::<T>::insert((owner, &pool_info.id, task_id), ());

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

		Self::remove_task(pool_id, &task)?;

		Ok(())
	}

	pub fn do_reclaim_expired_task(
		pool_id: &T::PoolId,
		task_id: &T::TaskId,
		current_block: T::BlockNumber
	) -> DispatchResult {
		let task = Tasks::<T>::get(&pool_id, task_id).ok_or(Error::<T>::TaskNotFound)?;
		Self::ensure_task_expired(current_block, &task)?;

		Self::remove_task(pool_id, &task)?;

		Ok(())
	}

	fn remove_task(
		pool_id: &T::PoolId,
		task: &Task::<T::TaskId, T::AccountId, BalanceOf<T>, T::BlockNumber>,
	) -> DispatchResult {
		let task_id = task.id;

		T::Currency::unreserve(&task.owner, task.owner_deposit);
		if let Some(task_input) = TaskInputs::<T>::take(pool_id, task_id) {
			let deposit = task_input.actual_deposit.saturating_add(task_input.surplus_deposit);
			T::Currency::unreserve(&task_input.depositor, deposit);
		}
		if let Some(task_output) = TaskOutputs::<T>::take(pool_id, task_id) {
			let deposit = task_output.actual_deposit.saturating_add(task_output.surplus_deposit);
			T::Currency::unreserve(&task_output.depositor, deposit);
		}

		Tasks::<T>::remove(pool_id, task_id);
		Accounts::<T>::remove((&task.owner, pool_id, task_id));

		Pools::<T>::try_mutate_exists(pool_id, |pool_info| -> Result<(), DispatchError> {
			let Some(pool_info) = pool_info else {
				return Err(Error::<T>::PoolNotFound.into())
			};

			pool_info.tasks_count -= 1;

			Ok(())
		})?;

		if task.released_at.is_none() {
			if let Some(worker) = &task.taken_by {
				WorkerTakenTasksCounter::<T>::try_mutate(&worker, |counter| -> Result<(), DispatchError> {
					*counter -= 1;
					Ok(())
				})?;
			}
		}

		Ok(())
	}
}
