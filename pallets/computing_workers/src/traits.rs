use crate::{
	types::{BalanceOf, NegativeImbalanceOf, OfflineReason, OnlinePayload, WorkerInfo},
	Config,
};
use frame_support::dispatch::DispatchResult;
use sp_std::prelude::*;

/// Trait describing something that implements a hook for any operations to perform when a staker is
/// slashed.
pub trait WorkerLifecycleHooks<AccountId, Balance> {
	/// A hook for checking the worker whether can online,
	/// can use for add extra conditions check, if returns error, the worker will not be online
	fn can_online(worker: &AccountId, payload: &OnlinePayload, verified_attestation: &Option<VerifiedAttestation>) -> DispatchResult;

	/// A hook after the worker transited to online status,
	/// can use for add additional business logic, e.g. assign job, reserve more money
	fn after_online(worker: &AccountId);

	/// A hook for checking the worker whether can offline,
	/// can use for add extra conditions check,
	/// if returns error (e.g. still have job running), the worker will not be offline
	fn can_offline(worker: &AccountId) -> bool;

	/// A hook before the worker transited to offline status,
	/// can use for add additional business logic, e.g. un-reserve money
	fn before_offline(worker: &AccountId, reason: OfflineReason);

	/// A hook after the worker update its attestation,
	/// Can use for if interest in payload's custom field
	fn after_refresh_attestation(worker: &AccountId, payload: &OnlinePayload, verified_attestation: &Option<VerifiedAttestation>);

	/// A hook after the worker transited to requesting offline status,
	/// can use for add additional business logic, e.g. stop assigning job
	fn after_requesting_offline(worker: &AccountId);

	/// A hook before the worker deregister
	fn before_deregister(worker: &AccountId);
}

impl<AccountId, Balance> WorkerLifecycleHooks<AccountId, Balance> for () {
	fn can_online(_: &AccountId, _: &OnlinePayload, _: &Option<VerifiedAttestation>) -> DispatchResult {
		Ok(())
	}

	fn after_online(_: &AccountId) {
		// Do nothing
	}

	fn can_offline(_: &AccountId) -> bool {
		true
	}

	fn before_offline(_: &AccountId, _: OfflineReason) {
		// Do nothing
	}

	fn after_refresh_attestation(_: &AccountId, _: &OnlinePayload, _: &Option<VerifiedAttestation>) {
		// Do nothing
	}

	fn after_requesting_offline(_: &AccountId) {
		// Do nothing
	}

	fn before_deregister(_: &AccountId) {
		// Do nothing
	}
}

pub trait WorkerManageable<T: Config> {
	fn worker_info(worker: &T::AccountId) -> Option<WorkerInfo<T>>;

	fn worker_exists(worker: &T::AccountId) -> bool;

	fn reward(worker: &T::AccountId, source: &T::AccountId, value: BalanceOf<T>) -> DispatchResult;

	fn slash(worker: &T::AccountId, value: BalanceOf<T>) -> (NegativeImbalanceOf<T>, BalanceOf<T>);

	fn offline(worker: &T::AccountId, reason: Option<Vec<u8>>) -> DispatchResult;
}

#[cfg(feature = "std")]
use frame_support::traits::Imbalance;
#[cfg(feature = "std")]
use sp_runtime::traits::Zero;
use crate::types::VerifiedAttestation;

#[cfg(feature = "std")]
impl<T: Config> WorkerManageable<T> for () {
	fn worker_info(_: &T::AccountId) -> Option<WorkerInfo<T>> {
		None
	}

	fn worker_exists(_: &T::AccountId) -> bool {
		false
	}

	fn reward(_: &T::AccountId, _: &T::AccountId, _: BalanceOf<T>) -> DispatchResult {
		Ok(())
	}

	fn slash(_: &T::AccountId, _: BalanceOf<T>) -> (NegativeImbalanceOf<T>, BalanceOf<T>) {
		(NegativeImbalanceOf::<T>::zero(), BalanceOf::<T>::zero())
	}

	fn offline(_: &T::AccountId, _: Option<Vec<u8>>) -> DispatchResult {
		Ok(())
	}
}
