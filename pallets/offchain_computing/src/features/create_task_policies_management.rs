use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub fn do_create_create_task_policy(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		policy_id: T::PolicyId,
		policy: CreateTaskPolicy<T::BlockNumber>,
	) -> DispatchResult {
		ensure!(
			!CreateTaskPolicies::<T>::contains_key(&pool_info.id, &policy_id),
			Error::<T>::PolicyIdTaken
		);

		CreateTaskPolicies::<T>::insert(&pool_info.id, &policy_id, policy.clone());

		let mut new_pool_info = pool_info.clone();
		new_pool_info.create_task_policies_count += 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::CreateTaskPolicyCreated { pool_id: pool_info.id, policy_id, policy: policy });
		Ok(())
	}

	pub fn do_destroy_create_task_policy(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		policy_id: T::PolicyId,
	) -> DispatchResult {
		ensure!(
			CreateTaskPolicies::<T>::contains_key(&pool_info.id, &policy_id),
			Error::<T>::CreateTaskPolicyNotFound
		);

		CreateTaskPolicies::<T>::remove(&pool_info.id, &policy_id);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.create_task_policies_count -= 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::CreateTaskPolicyDestroyed { pool_id: pool_info.id, policy_id });
		Ok(())
	}
}
