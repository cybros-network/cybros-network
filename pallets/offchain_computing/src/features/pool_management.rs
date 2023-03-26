use crate::*;
use frame_support::pallet_prelude::*;
use sp_runtime::{
	traits::Zero,
	Saturating
};

impl<T: Config> Pallet<T> {
	pub fn do_create_pool(
		owner: &T::AccountId,
		owner_deposit: &BalanceOf<T>,
		pool_id: &T::PoolId
	) -> DispatchResult {
		ensure!(!Pools::<T>::contains_key(pool_id), Error::<T>::PoolIdTaken);

		T::Currency::reserve(owner, *owner_deposit)?;

		let pool_info = PoolInfo::<T::PoolId, T::AccountId, BalanceOf<T>> {
			id: *pool_id,
			owner: owner.clone(),
			owner_deposit: *owner_deposit,
			stash_account: owner.clone(),
			creating_task_ability: true,
			creating_task_policies_count: 0,
			tasks_count: 0,
			workers_count: 0,
		};

		Pools::<T>::insert(pool_id, pool_info);
		AccountOwnedPools::<T>::insert(owner, pool_id, ());

		Self::deposit_event(Event::PoolCreated { owner: owner.clone(), pool_id: pool_id.clone() });
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

		if let Some(metadata_entry) = PoolMetadata::<T>::take(&pool_id) {
			T::Currency::unreserve(&pool_info.owner, metadata_entry.actual_deposit);
		}

		let _ = CreatingTaskPolicies::<T>::clear_prefix(&pool_id, pool_info.creating_task_policies_count, None);

		Pools::<T>::remove(&pool_id);
		AccountOwnedPools::<T>::remove(&pool_info.owner, &pool_id);

		T::Currency::unreserve(&pool_info.owner, pool_info.owner_deposit);

		Self::deposit_event(Event::PoolDestroyed { pool_id });
		Ok(())
	}

	pub fn do_update_pool_metadata(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		new_metadata: &BoundedVec<u8, T::PoolMetadataLimit>
	) -> DispatchResult {
		PoolMetadata::<T>::try_mutate_exists(pool_info.id, |metadata_entry| {
			let deposit = T::DepositPerByte::get()
				.saturating_mul(((new_metadata.len()) as u32).into())
				.saturating_add(T::MetadataDepositBase::get());

			let old_deposit = metadata_entry.take().map_or(Zero::zero(), |m| m.actual_deposit);
			if deposit > old_deposit {
				T::Currency::reserve(&pool_info.owner, deposit - old_deposit)?;
			} else if deposit < old_deposit {
				T::Currency::unreserve(&pool_info.owner, old_deposit - deposit);
			}

			*metadata_entry = Some(ChainStoredData {
				depositor: pool_info.owner.clone(),
				actual_deposit: deposit,
				surplus_deposit: Zero::zero(),
				data: new_metadata.clone()
			});

			Self::deposit_event(Event::PoolMetadataUpdated { pool_id: pool_info.id, new_metadata: new_metadata.clone() });
			Ok(())
		})
	}

	pub fn do_remove_pool_metadata(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>
	) -> DispatchResult {
		let Some(metadata_entry) = PoolMetadata::<T>::get(&pool_info.id) else {
			return Ok(())
		};

		PoolMetadata::<T>::remove(&pool_info.id);
		T::Currency::unreserve(&pool_info.owner, metadata_entry.actual_deposit);

		Self::deposit_event(Event::PoolMetadataRemoved { pool_id: pool_info.id });
		Ok(())
	}

	pub fn do_update_pool_stash_account(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		new_stash_account: &T::AccountId
	) -> DispatchResult {
		let mut new_pool_info = pool_info.clone();
		new_pool_info.stash_account = new_stash_account.clone();

		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::PoolStashAccountUpdated { pool_id: pool_info.id, stash_account: new_stash_account.clone() });
		Ok(())
	}

	pub fn do_toggle_pool_creating_task_ability(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		enabled: bool
	) -> DispatchResult {
		let mut new_pool_info = pool_info.clone();
		new_pool_info.creating_task_ability = enabled;

		Pools::<T>::insert(&pool_info.id, new_pool_info);

		if enabled {
			Self::deposit_event(Event::PoolCreatingTaskAbilityEnabled { pool_id: pool_info.id });
		} else {
			Self::deposit_event(Event::PoolCreatingTaskAbilityDisabled { pool_id: pool_info.id });
		}
		Ok(())
	}
}
