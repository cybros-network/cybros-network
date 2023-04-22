use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub(crate) fn do_create_task_policy(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		policy_id: T::PolicyId,
		creating_task_scope: ApplicableScope,
		start_block: Option<T::BlockNumber>,
		end_block: Option<T::BlockNumber>
	) -> DispatchResult {
		ensure!(
			!TaskPolicies::<T>::contains_key(&pool_info.id, &policy_id),
			Error::<T>::PolicyIdTaken
		);

		let policy = TaskPolicy::<T::PolicyId, T::BlockNumber> {
			id: policy_id.clone(),
			availability: true,
			creating_task_scope: creating_task_scope.clone(),
			start_block: start_block.clone(),
			end_block: end_block.clone(),
			tasks_count: 0,
		};
		TaskPolicies::<T>::insert(&pool_info.id, &policy_id, policy);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.task_policies_count += 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::TaskPolicyCreated { pool_id: pool_info.id, policy_id, creating_task_scope, start_block, end_block });
		Ok(())
	}

	pub(crate) fn do_destroy_task_policy(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		policy_id: T::PolicyId,
	) -> DispatchResult {
		ensure!(
			TaskPolicies::<T>::contains_key(&pool_info.id, &policy_id),
			Error::<T>::PolicyNotFound
		);

		let policy = TaskPolicies::<T>::get(&pool_info.id, &policy_id).ok_or(Error::<T>::PolicyNotFound)?;
		ensure!(
			policy.tasks_count == 0,
			Error::<T>::PolicyStillInUse
		);

		TaskPolicies::<T>::remove(&pool_info.id, &policy_id);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.task_policies_count -= 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::TaskPolicyDestroyed { pool_id: pool_info.id, policy_id });
		Ok(())
	}

	pub(crate) fn do_update_task_policy_availability(
		pool_id: T::PoolId,
		policy_id: T::PolicyId,
		availability: bool
	) -> DispatchResult {
		ensure!(
			TaskPolicies::<T>::contains_key(&pool_id, &policy_id),
			Error::<T>::PolicyNotFound
		);

		let mut policy = TaskPolicies::<T>::get(&pool_id, &policy_id).ok_or(Error::<T>::PolicyNotFound)?;
		policy.availability = availability;
		TaskPolicies::<T>::insert(&pool_id, &policy_id, policy.clone());

		Self::deposit_event(Event::TaskPolicyAvailabilityUpdated { pool_id, policy_id, availability });
		Ok(())
	}
}
