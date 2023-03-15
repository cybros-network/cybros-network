use crate::*;
use frame_support::pallet_prelude::*;
use sp_runtime::{
	traits::Zero,
	Saturating
};

impl<T: Config> Pallet<T> {
	pub fn do_create_pool(
		owner: &T::AccountId,
		owner_deposit: BalanceOf<T>,
		pool_id: T::PoolId
	) -> DispatchResult {
		ensure!(!Pools::<T>::contains_key(&pool_id), Error::<T>::PoolIdTaken);

		T::Currency::reserve(owner, owner_deposit)?;

		let pool_info = PoolInfo::<T::PoolId, T::AccountId, BalanceOf<T>> {
			id: pool_id,
			owner: owner.clone(),
			owner_deposit,
			stash_account: owner.clone(),
			create_task_ability: true,
			create_task_policies_count: 0,
			tasks_count: 0,
			workers_count: 0,
		};

		Pools::<T>::insert(pool_id, pool_info);
		PoolsAccounts::<T>::insert(owner, &pool_id, ());

		Ok(())
	}

	pub fn do_destroy_pool(
		who: &T::AccountId,
		pool_id: T::PoolId,
	) -> DispatchResult {
		let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
		Self::ensure_pool_owner(&who, &pool_info)?;
		ensure!(pool_info.tasks_count == 0, Error::<T>::PoolNotEmpty);
		ensure!(pool_info.workers_count == 0, Error::<T>::PoolNotEmpty);

		if let Some(custom_info) = PoolCustomInfos::<T>::take(&pool_id) {
			T::Currency::unreserve(&pool_info.owner, custom_info.actual_deposit);
		}

		let _ = CreateTaskPolicies::<T>::clear_prefix(&pool_id, pool_info.create_task_policies_count, None);

		Pools::<T>::remove(&pool_id);
		PoolsAccounts::<T>::remove(&pool_info.owner, &pool_id);

		T::Currency::unreserve(&pool_info.owner, pool_info.owner_deposit);

		Ok(())
	}

	pub fn do_update_pool_custom_info(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		data: &BoundedVec<u8, T::PoolCustomInfoLimit>
	) -> DispatchResult {
		PoolCustomInfos::<T>::try_mutate_exists(pool_info.id, |custom_info| {
			let deposit = T::DepositPerByte::get()
				.saturating_mul(((data.len()) as u32).into())
				.saturating_add(T::CustomInfoDepositBase::get());

			let old_deposit = custom_info.take().map_or(Zero::zero(), |m| m.actual_deposit);
			if deposit > old_deposit {
				T::Currency::reserve(&pool_info.owner, deposit - old_deposit)?;
			} else if deposit < old_deposit {
				T::Currency::unreserve(&pool_info.owner, old_deposit - deposit);
			}

			*custom_info = Some(ChainStoredData {
				depositor: pool_info.owner.clone(),
				actual_deposit: deposit,
				surplus_deposit: Zero::zero(),
				data: data.clone()
			});
			Ok(())
		})
	}

	pub fn do_remove_pool_custom_info(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>
	) -> DispatchResult {
		let Some(custom_info) = PoolCustomInfos::<T>::get(&pool_info.id) else {
			return Ok(())
		};

		PoolCustomInfos::<T>::remove(&pool_info.id);
		T::Currency::unreserve(&pool_info.owner, custom_info.actual_deposit);

		Ok(())
	}

	pub fn do_update_pool_stash_account(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		new_stash_account: T::AccountId
	) -> DispatchResult {
		let mut new_pool_info = pool_info.clone();
		new_pool_info.stash_account = new_stash_account;

		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Ok(())
	}

	pub fn do_toggle_pool_create_task_ability(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		enabled: bool
	) -> DispatchResult {
		let mut new_pool_info = pool_info.clone();
		new_pool_info.create_task_ability = enabled;

		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Ok(())
	}
}
