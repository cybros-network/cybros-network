// This file is part of Cybros.

// Copyright (C) Jun Jiang.
// SPDX-License-Identifier: AGPL-3.0-only

// Cybros is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cybros is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with Cybros.  If not, see <http://www.gnu.org/licenses/>.

use crate::*;
use frame_support::pallet_prelude::*;
use sp_runtime::{
	traits::Zero,
	Saturating
};
use sp_std::cmp::Ordering;
use pallet_offchain_computing_workers::ApplicableScope;

impl<T: Config> Pallet<T> {
	pub(crate) fn do_create_pool(
		owner: T::AccountId,
		pool_id: T::PoolId,
		impl_id: ImplIdOf<T>,
		create_job_enabled: bool,
		auto_destroy_processed_job_enabled: bool
	) -> DispatchResult {
		ensure!(!Pools::<T>::contains_key(&pool_id), Error::<T>::PoolIdTaken);

		let impl_info = T::OffchainWorkerManageable::impl_info(&impl_id).ok_or(Error::<T>::ImplNotFound)?;
		ensure!(
			match impl_info.deployment_scope {
				ApplicableScope::Owner => {
					impl_info.owner == owner
				},
				ApplicableScope::Public => {
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
			create_job_enabled,
			auto_destroy_processed_job_enabled,
			min_impl_spec_version: 1,
			max_impl_spec_version: 1,
			job_policies_count: 0,
			jobs_count: 0,
			workers_count: 0,
		};

		Pools::<T>::insert(&pool_id, pool_info);
		AccountOwningPools::<T>::insert(&owner, &pool_id, ());

		Self::deposit_event(Event::PoolCreated { owner, pool_id, impl_id, create_job_enabled, auto_destroy_processed_job_enabled });
		Ok(())
	}

	pub(crate) fn do_destroy_pool(
		who: T::AccountId,
		pool_id: T::PoolId,
	) -> DispatchResult {
		let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
		Self::ensure_pool_owner(&who, &pool_info)?;
		ensure!(pool_info.jobs_count == 0, Error::<T>::PoolNotEmpty);
		ensure!(pool_info.workers_count == 0, Error::<T>::PoolNotEmpty);

		if let Some(metadata_entry) = PoolMetadata::<T>::take(&pool_id) {
			T::Currency::unreserve(&pool_info.owner, metadata_entry.actual_deposit);
		}

		let _ = JobPolicies::<T>::clear_prefix(&pool_id, pool_info.job_policies_count, None);

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
			match deposit.cmp(&old_deposit) {
				Ordering::Greater => {
					T::Currency::reserve(&pool_info.owner, deposit - old_deposit)?;
				},
				Ordering::Less => {
					T::Currency::unreserve(&pool_info.owner, old_deposit - deposit);
				},
				_ => {}
			};

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

	pub(crate) fn do_update_pool_settings(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		min_impl_spec_version: ImplSpecVersion,
		max_impl_spec_version: ImplSpecVersion,
		create_job_enabled: bool,
		auto_destroy_processed_job_enabled: bool,
	) -> DispatchResult {
		ensure!(
			max_impl_spec_version >= min_impl_spec_version,
			Error::<T>::InvalidImplSpecVersionRange
		);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.min_impl_spec_version = min_impl_spec_version;
		new_pool_info.max_impl_spec_version = max_impl_spec_version;
		new_pool_info.create_job_enabled = create_job_enabled;
		new_pool_info.auto_destroy_processed_job_enabled = auto_destroy_processed_job_enabled;

		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(
			Event::PoolSettingsUpdated {
				pool_id: pool_info.id,
				min_impl_spec_version,
				max_impl_spec_version,
				create_job_enabled,
				auto_destroy_processed_job_enabled
			});
		Ok(())
	}
}
