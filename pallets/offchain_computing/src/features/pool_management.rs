use crate::*;
use frame_support::pallet_prelude::*;
use sp_runtime::{
	traits::Zero,
	Saturating
};
use pallet_offchain_computing_workers::ImplDeploymentPermission;

impl<T: Config> Pallet<T> {
	pub(crate) fn do_create_pool(
		owner: T::AccountId,
		pool_id: T::PoolId,
		impl_id: ImplIdOf<T>
	) -> DispatchResult {
		ensure!(!Pools::<T>::contains_key(&pool_id), Error::<T>::PoolIdTaken);

		let impl_info = T::OffchainWorkerManageable::impl_info(&impl_id).ok_or(Error::<T>::ImplNotFound)?;
		ensure!(
			match impl_info.deployment_permission {
				ImplDeploymentPermission::Owner => {
					impl_info.owner == owner
				},
				ImplDeploymentPermission::Public => {
					true
				}
			},
			Error::<T>::NoPermission
		);

		T::Currency::reserve(&owner, T::CreatePoolDeposit::get())?;

		let pool_info = PoolInfo::<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>> {
			id: pool_id.clone(),
			owner: owner.clone(),
			owner_deposit: T::CreatePoolDeposit::get(),
			impl_id: impl_id.clone(),
			creating_task_availability: true,
			task_policies_count: 0,
			tasks_count: 0,
			workers_count: 0,
		};

		Pools::<T>::insert(&pool_id, pool_info);
		AccountOwningPools::<T>::insert(&owner, &pool_id, ());

		Self::deposit_event(Event::PoolCreated { owner, pool_id, impl_id });
		Ok(())
	}

	pub(crate) fn do_destroy_pool(
		who: T::AccountId,
		pool_id: T::PoolId,
	) -> DispatchResult {
		let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
		Self::ensure_pool_owner(&who, &pool_info)?;
		ensure!(pool_info.tasks_count == 0, Error::<T>::PoolNotEmpty);
		ensure!(pool_info.workers_count == 0, Error::<T>::PoolNotEmpty);

		if let Some(metadata_entry) = PoolMetadata::<T>::take(&pool_id) {
			T::Currency::unreserve(&pool_info.owner, metadata_entry.actual_deposit);
		}

		let _ = TaskPolicies::<T>::clear_prefix(&pool_id, pool_info.task_policies_count, None);

		Pools::<T>::remove(&pool_id);
		AccountOwningPools::<T>::remove(&pool_info.owner, &pool_id);

		T::Currency::unreserve(&pool_info.owner, pool_info.owner_deposit);

		Self::deposit_event(Event::PoolDestroyed { pool_id });
		Ok(())
	}

	pub(crate) fn do_update_pool_metadata(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		new_metadata: BoundedVec<u8, T::PoolMetadataLimit>
	) -> DispatchResult {
		let pool_id = pool_info.id;
		PoolMetadata::<T>::try_mutate_exists(&pool_id.clone(), |metadata_entry| {
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

			Self::deposit_event(Event::PoolMetadataUpdated { pool_id, metadata: new_metadata.clone() });
			Ok(())
		})
	}

	pub(crate) fn do_remove_pool_metadata(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>
	) -> DispatchResult {
		let Some(metadata_entry) = PoolMetadata::<T>::get(&pool_info.id) else {
			return Ok(())
		};

		PoolMetadata::<T>::remove(&pool_info.id);
		T::Currency::unreserve(&pool_info.owner, metadata_entry.actual_deposit);

		Self::deposit_event(Event::PoolMetadataRemoved { pool_id: pool_info.id });
		Ok(())
	}

	pub(crate) fn do_toggle_pool_task_creatable(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		creatable: bool
	) -> DispatchResult {
		let mut new_pool_info = pool_info.clone();
		new_pool_info.creating_task_availability = creatable;

		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::PoolCreatingTaskAvailabilityUpdated { pool_id: pool_info.id, availability: creatable });
		Ok(())
	}
}
