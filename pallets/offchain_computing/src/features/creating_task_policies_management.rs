use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub(crate) fn do_create_creating_task_policy(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		policy_id: T::PolicyId,
		policy: CreatingTaskPolicy<T::BlockNumber>,
	) -> DispatchResult {
		ensure!(
			!CreatingTaskPolicies::<T>::contains_key(&pool_info.id, &policy_id),
			Error::<T>::PolicyIdTaken
		);

		CreatingTaskPolicies::<T>::insert(&pool_info.id, &policy_id, policy.clone());

		let mut new_pool_info = pool_info.clone();
		new_pool_info.creating_task_policies_count += 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::CreatingTaskPolicyCreated { pool_id: pool_info.id, policy_id, policy });
		Ok(())
	}

	pub(crate) fn do_destroy_creating_task_policy(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		policy_id: T::PolicyId,
	) -> DispatchResult {
		ensure!(
			CreatingTaskPolicies::<T>::contains_key(&pool_info.id, &policy_id),
			Error::<T>::PolicyNotFound
		);

		CreatingTaskPolicies::<T>::remove(&pool_info.id, &policy_id);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.creating_task_policies_count -= 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::CreatingTaskPolicyDestroyed { pool_id: pool_info.id, policy_id });
		Ok(())
	}
}
