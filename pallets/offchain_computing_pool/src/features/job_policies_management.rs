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
use frame_system::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub(crate) fn do_create_job_policy(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, T::ImplId>,
		policy_id: T::PolicyId,
		applicable_scope: ApplicableScope,
		start_block: Option<BlockNumberFor<T>>,
		end_block: Option<BlockNumberFor<T>>
	) -> DispatchResult {
		ensure!(
			!JobPolicies::<T>::contains_key(&pool_info.id, &policy_id),
			Error::<T>::PolicyIdTaken
		);

		let policy = JobPolicy::<T::PolicyId, BlockNumberFor<T>> {
			id: policy_id.clone(),
			enabled: true,
			applicable_scope: applicable_scope.clone(),
			start_block,
			end_block,
			jobs_count: 0,
		};
		JobPolicies::<T>::insert(&pool_info.id, &policy_id, policy);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.job_policies_count += 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::JobPolicyCreated { pool_id: pool_info.id, policy_id, applicable_scope, start_block, end_block });
		Ok(())
	}

	pub(crate) fn do_destroy_job_policy(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, T::ImplId>,
		policy_id: T::PolicyId,
	) -> DispatchResult {
		ensure!(
			JobPolicies::<T>::contains_key(&pool_info.id, &policy_id),
			Error::<T>::JobPolicyNotFound
		);

		let policy = JobPolicies::<T>::get(&pool_info.id, &policy_id).ok_or(Error::<T>::JobPolicyNotFound)?;
		ensure!(
			policy.jobs_count == 0,
			Error::<T>::JobPolicyStillInUse
		);

		JobPolicies::<T>::remove(&pool_info.id, &policy_id);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.job_policies_count -= 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::JobPolicyDestroyed { pool_id: pool_info.id, policy_id });
		Ok(())
	}

	pub(crate) fn do_update_job_policy_enablement(
		pool_id: T::PoolId,
		policy_id: T::PolicyId,
		enabled: bool
	) -> DispatchResult {
		ensure!(
			JobPolicies::<T>::contains_key(&pool_id, &policy_id),
			Error::<T>::JobPolicyNotFound
		);

		let mut policy = JobPolicies::<T>::get(&pool_id, &policy_id).ok_or(Error::<T>::JobPolicyNotFound)?;
		policy.enabled = enabled;
		JobPolicies::<T>::insert(&pool_id, &policy_id, policy.clone());

		Self::deposit_event(Event::JobPolicyEnablementUpdated { pool_id, policy_id, enabled });
		Ok(())
	}
}
