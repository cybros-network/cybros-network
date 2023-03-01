use frame_support::{
	dispatch::DispatchResult,
	traits::{
		tokens::Balance,
		Imbalance,
		ReservableCurrency
	},
};
use sp_std::prelude::*;
use primitives::{OfflineReason, OnlinePayload, WorkerInfo};

/// Trait describing something that implements a hook for any operations to perform when a staker is
/// slashed.
pub trait WorkerLifecycleHooks<AccountId> {
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

impl<AccountId> WorkerLifecycleHooks<AccountId> for () {
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

pub trait WorkerManageable<AccountId, BlockNumber> {
	type Currency: ReservableCurrency<AccountId>;
	type Balance: Balance;
	type PositiveImbalance: Imbalance<Self::Balance, Opposite = Self::NegativeImbalance>;
	type NegativeImbalance: Imbalance<Self::Balance, Opposite = Self::PositiveImbalance>;

	fn worker_info(worker: &AccountId) -> Option<WorkerInfo<AccountId, Self::Balance, BlockNumber>>;

	fn worker_exists(worker: &AccountId) -> bool;

	fn reward(worker: &AccountId, source: &AccountId, value: Self::Balance) -> DispatchResult;

	fn slash(worker: &AccountId, value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance);

	fn offline(worker: &AccountId, reason: Option<Vec<u8>>) -> DispatchResult;
}

#[cfg(feature = "std")]
use sp_runtime::traits::Zero;

use primitives::VerifiedAttestation;

#[cfg(feature = "std")]
impl<AccountId, BlockNumber> WorkerManageable<AccountId, BlockNumber> for () {
	type Currency = ();
	type Balance = u32;
	type PositiveImbalance = ();
	type NegativeImbalance = ();

	fn worker_info(_: &AccountId) -> Option<WorkerInfo<AccountId, Self::Balance, BlockNumber>> {
		None
	}

	fn worker_exists(_: &AccountId) -> bool {
		false
	}

	fn reward(_: &AccountId, _: &AccountId, _: Self::Balance) -> DispatchResult {
		Ok(())
	}

	fn slash(_: &AccountId, _: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
		((), Self::Balance::zero())
	}

	fn offline(_: &AccountId, _: Option<Vec<u8>>) -> DispatchResult {
		Ok(())
	}
}
