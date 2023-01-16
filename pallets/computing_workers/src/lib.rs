#![cfg_attr(not(feature = "std"), no_std)]

pub mod traits;
pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// The log target of this pallet.
pub const LOG_TARGET: &str = "runtime::computing_workers";

// Syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: $crate::LOG_TARGET,
			concat!("[{:?}] ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}

pub use primitives;
pub use pallet::*;

use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::{Currency, ExistenceRequirement, Get, Randomness, ReservableCurrency, UnixTime},
	transactional,
};
use scale_codec::{Decode, Encode};
use sp_core::{sr25519, H256};
use sp_io::crypto::sr25519_verify;
use sp_runtime::{traits::Zero, SaturatedConversion, Saturating};
use sp_std::prelude::*;

use primitives::*;
use crate::{
	traits::*,
	weights::WeightInfo,
};

pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;
pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

#[frame_support::pallet]
mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The system's currency for payment.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Time used for verify attestation
		type UnixTime: UnixTime;

		/// Something that provides randomness in the runtime.
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		/// Max number of moving unresponsive workers to pending offline workers queue
		#[pallet::constant]
		type HandleUnresponsivePerBlockLimit: Get<u32>;

		/// The minimum amount required to keep a worker registration.
		#[pallet::constant]
		type ReservedDeposit: Get<BalanceOf<Self>>;

		/// The duration (blocks) of collecting workers' heartbeats
		#[pallet::constant]
		type CollectingHeartbeatsDuration: Get<u32>;

		/// The duration (blocks) of collecting workers' heartbeats
		#[pallet::constant]
		type AttestationValidityDuration: Get<u32>;

		/// Allow Opt out attestation
		///
		/// SHOULD NOT SET TO FALSE ON PRODUCTION!!!
		#[pallet::constant]
		type DisallowOptOutAttestation: Get<bool>;

		/// Allow Opt out attestation
		///
		/// SHOULD NOT SET TO FALSE ON PRODUCTION!!!
		#[pallet::constant]
		type DisallowNonTEEAttestation: Get<bool>;

		/// Validate worker's implementation
		#[pallet::constant]
		type ValidateWorkerImpl: Get<bool>;

		/// Validate worker's implementation's hash
		#[pallet::constant]
		type ValidateWorkerImplHash: Get<bool>;

		/// Origin used to govern the pallet
		type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// A handler for manging worker slashing
		type WorkerLifecycleHooks: WorkerLifecycleHooks<Self::AccountId, BalanceOf<Self>>;
	}

	/// Storage for worker's implementations permission.
	#[pallet::storage]
	#[pallet::getter(fn worker_impl_permissions)]
	pub(crate) type WorkerImplPermissions<T: Config> = StorageMap<_, Identity, WorkerImplName, WorkerImplPermission>;

	/// Storage for worker's implementations' hashes.
	#[pallet::storage]
	#[pallet::getter(fn worker_impl_hashes)]
	pub(crate) type WorkerImplHashes<T: Config> =
		StorageDoubleMap<_, Identity, WorkerImplName, Identity, WorkerImplVersion, WorkerImplHash>;

	/// Storage for computing_workers.
	#[pallet::storage]
	#[pallet::getter(fn workers)]
	pub(crate) type Workers<T: Config> = CountedStorageMap<_, Identity, T::AccountId, WorkerInfo<T::AccountId, BalanceOf<T>, T::BlockNumber>>;

	/// Storage for flip set, this is for online checking
	#[pallet::storage]
	#[pallet::getter(fn flip_set)]
	pub(crate) type FlipSet<T: Config> = CountedStorageMap<_, Identity, T::AccountId, T::BlockNumber>;

	/// Storage for flop set, this is for online checking
	#[pallet::storage]
	#[pallet::getter(fn flop_set)]
	pub(crate) type FlopSet<T: Config> = CountedStorageMap<_, Identity, T::AccountId, T::BlockNumber>;

	/// Storage for stage of flip-flop, this is used for online checking
	#[pallet::storage]
	#[pallet::getter(fn flip_flop_stage)]
	pub(crate) type FlipOrFlop<T: Config> = StorageValue<_, FlipFlopStage, ValueQuery>;

	/// Storage for stage of flip-flop, this is used for online checking
	#[pallet::storage]
	#[pallet::getter(fn current_flip_flop_started_at)]
	pub(crate) type CurrentFlipFlopStartedAt<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The worker registered successfully
		Registered { worker: T::AccountId },
		/// The worker registered successfully
		Deregistered { worker: T::AccountId, force: bool },
		/// The worker is online
		Online {
			worker: T::AccountId,
			impl_name: WorkerImplName,
			impl_version: WorkerImplVersion,
			attestation_method: Option<AttestationMethod>,
			next_heartbeat: T::BlockNumber,
		},
		/// The worker is requesting offline
		RequestingOffline { worker: T::AccountId },
		/// The worker is offline
		Offline { worker: T::AccountId, reason: OfflineReason },
		/// The worker send heartbeat successfully
		HeartbeatReceived { worker: T::AccountId, next_heartbeat: T::BlockNumber },
		/// The worker refresh its attestation successfully
		AttestationRefreshed { worker: T::AccountId },
		/// Update worker's implementation permission successfully
		WorkerImplPermissionUpdated { impl_name: WorkerImplName },
		/// Remove worker's implementation permission successfully
		WorkerImplPermissionRemoved { impl_name: WorkerImplName },
		/// Update worker's implementation permission successfully
		WorkerImplHashUpdated { impl_name: WorkerImplName, impl_version: WorkerImplVersion },
		/// Remove worker's implementation permission successfully
		WorkerImplHashRemoved { impl_name: WorkerImplName, impl_version: WorkerImplVersion },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The own must not the worker it self
		InvalidOwner,
		/// Initial deposit for register a worker must equal or above `ExistentialDeposit`
		InitialDepositTooLow,
		/// Worker already registered
		AlreadyRegistered,
		/// Worker's wallet reserved money smaller than should be reserved
		InsufficientReserved,
		/// The extrinsic origin isn't the worker's owner
		NotTheOwner,
		/// The extrinsic origin isn't the worker
		NotTheWorker,
		/// The worker not exists
		NotExists,
		/// The worker is not online
		NotOnline,
		/// The worker must offline before do deregister
		NotOffline,
		/// The worker's status doesn't allow the operation
		WrongStatus,
		/// Attestation required
		MustProvideAttestation,
		/// Attestation expired,
		ExpiredAttestation,
		/// Attestation invalid,
		InvalidAttestation,
		/// Attestation payload invalid
		CanNotVerifyPayload,
		/// Can't verify payload
		PayloadSignatureMismatched,
		/// Can not downgrade
		WorkerImplCanNotDowngrade,
		/// Worker's software changed, it must offline first
		WorkerImplChanged,
		/// Worker's software blocked
		WorkerImplBlocked,
		/// Worker's software unsupported.
		WorkerImplUnsupported,
		/// Unknown worker implementation's hash
		UnknownWorkerImplHash,
		/// worker implementation's hash mismatch
		WorkerImplHashMismatched,
		/// The runtime disallowed NonTEE worker
		DisallowNonTEEAttestation,
		/// Unsupported attestation
		UnsupportedAttestation,
		/// The attestation method must not change
		AttestationMethodChanged,
		/// AlreadySentHeartbeat
		HeartbeatAlreadySent,
		/// Too early to send heartbeat
		TooEarly,
		/// Already requested offline
		AlreadyRequestedOffline,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			let mut reads: u64 = 2; // Read FlipOrFlop and CurrentFlipFlopStartedAt
			let mut writes: u64 = 0;

			let mut flip_or_flop = FlipOrFlop::<T>::get();
			let current_flip_flop_started_at = CurrentFlipFlopStartedAt::<T>::get();
			if n == current_flip_flop_started_at + T::CollectingHeartbeatsDuration::get().into() {
				match flip_or_flop {
					FlipFlopStage::Flip => {
						flip_or_flop = FlipFlopStage::FlipToFlop;
						FlipOrFlop::<T>::set(flip_or_flop);
						writes += 1;
					},
					FlipFlopStage::Flop => {
						flip_or_flop = FlipFlopStage::FlopToFlip;
						FlipOrFlop::<T>::set(flip_or_flop);
						writes += 1;
					},
					_ => {},
				}
			}
			match flip_or_flop {
				FlipFlopStage::FlipToFlop => {
					let iter = FlipSet::<T>::iter_keys().take(T::HandleUnresponsivePerBlockLimit::get() as usize);
					let total_count = FlipSet::<T>::count();

					let mut i: u64 = 0;
					for worker in iter {
						FlipSet::<T>::remove(&worker);
						Self::handle_worker_unresponsive(&worker);
						i += 1;
					}

					reads += i;
					writes += i.saturating_mul(3);

					if i >= total_count as u64 {
						FlipOrFlop::<T>::set(FlipFlopStage::Flop);
						CurrentFlipFlopStartedAt::<T>::set(n);
						writes += 2;
					}
				},
				FlipFlopStage::FlopToFlip => {
					let iter = FlopSet::<T>::iter_keys().take(T::HandleUnresponsivePerBlockLimit::get() as usize);
					let total_count = FlopSet::<T>::count();

					let mut i: u64 = 0;
					for worker in iter {
						FlopSet::<T>::remove(&worker);
						Self::handle_worker_unresponsive(&worker);
						i += 1;
					}

					reads += i;
					writes += i.saturating_mul(3);

					if i >= total_count as u64 {
						FlipOrFlop::<T>::set(FlipFlopStage::Flip);
						CurrentFlipFlopStartedAt::<T>::set(n);
						writes += 2;
					}
				},
				_ => {},
			}

			T::DbWeight::get().reads_writes(reads, writes)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a computing workers.
		///
		/// ## Arguments
		/// - `origin`: Must be called by a `Signed` origin, it will become the worker's owner.
		/// - `worker`: The worker.
		/// - `initial_deposit`: Initial deposit amount.
		///
		/// ## Deposits/Fees
		/// The origin signed account will transfer `initial_deposit` to worker's current account
		/// that will use for slashing.
		/// If the balance below `ReservedDeposit`, the worker will be removed
		///
		/// ## Events
		/// The `Registered` event is emitted in case of success.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::register())]
		#[transactional]
		pub fn register(origin: OriginFor<T>, worker: T::AccountId, initial_deposit: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_register(who, worker, initial_deposit)
		}

		/// The worker claim for online
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::online())]
		#[transactional]
		pub fn online(
			origin: OriginFor<T>,
			payload: OnlinePayload,
			attestation: Option<Attestation>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_online(who, payload, attestation)
		}

		/// The worker requesting offline
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::request_offline())]
		#[transactional]
		pub fn request_offline(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_request_offline(who, None)
		}

		/// The worker force offline, slashing will apply
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::force_offline())]
		#[transactional]
		pub fn force_offline(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_force_offline(who, None)
		}

		/// Deregister a computing workers.
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::deregister())]
		#[transactional]
		pub fn deregister(origin: OriginFor<T>, worker: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_deregister(who, worker)
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::refresh_attestation())]
		#[transactional]
		pub fn refresh_attestation(
			origin: OriginFor<T>,
			payload: OnlinePayload,
			attestation: Option<Attestation>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_refresh_attestation(who, payload, attestation)
		}

		/// Worker report it is still online, must called by the worker
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::heartbeat())]
		#[transactional]
		pub fn heartbeat(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_heartbeat(who)
		}

		/// The same with balances.transfer_keep_alive(owner, worker, balance)
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::deposit())]
		#[transactional]
		pub fn deposit(origin: OriginFor<T>, worker: T::AccountId, value: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::NotExists)?;
			Self::ensure_owner(&who, &worker_info)?;

			<T as Config>::Currency::transfer(&who, &worker, value, ExistenceRequirement::KeepAlive)?;
			Ok(())
		}

		/// The same with balances.transfer_keep_alive(worker, owner, balance)
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::withdraw())]
		#[transactional]
		pub fn withdraw(origin: OriginFor<T>, worker: T::AccountId, value: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::NotExists)?;
			Self::ensure_owner(&who, &worker_info)?;

			<T as Config>::Currency::transfer(&worker, &who, value, ExistenceRequirement::KeepAlive)?;
			Ok(())
		}

		/// The owner (or his proxy) requesting a worker to offline
		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::request_offline_for())]
		#[transactional]
		pub fn request_offline_for(origin: OriginFor<T>, worker: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_request_offline(worker, Some(who))
		}

		/// The owner (or his proxy) force a worker to offline, will apply slash
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::force_offline_for())]
		#[transactional]
		pub fn force_offline_for(origin: OriginFor<T>, worker: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_force_offline(worker, Some(who))
		}


		/// Set worker's implementations' permissions
		#[pallet::call_index(11)]
		#[pallet::weight(0)]
		#[transactional]
		pub fn set_worker_impl_permission(
			origin: OriginFor<T>,
			impl_name: WorkerImplName,
			impl_permission: Option<WorkerImplPermission>,
		) -> DispatchResult {
			T::GovernanceOrigin::ensure_origin(origin)?;
			Self::do_set_worker_impl_permission(impl_name, impl_permission)
		}

		/// Set worker's implementations' hashes
		#[pallet::call_index(12)]
		#[pallet::weight(0)]
		#[transactional]
		pub fn set_worker_impl_hashes(
			origin: OriginFor<T>,
			impl_name: WorkerImplName,
			impl_version: WorkerImplVersion,
			impl_hash: Option<WorkerImplHash>,
		) -> DispatchResult {
			T::GovernanceOrigin::ensure_origin(origin)?;
			Self::do_set_worker_impl_hashes(impl_name, impl_version, impl_hash)
		}
	}
}

impl<T: Config> Pallet<T> {
	fn do_register(owner: T::AccountId, worker: T::AccountId, initial_deposit: BalanceOf<T>) -> DispatchResult {
		ensure!(owner != worker, Error::<T>::InvalidOwner);

		let initial_reserved_deposit = T::ReservedDeposit::get();
		ensure!(initial_deposit >= initial_reserved_deposit, Error::<T>::InitialDepositTooLow);

		ensure!(!Workers::<T>::contains_key(&worker), Error::<T>::AlreadyRegistered);

		let worker_info = WorkerInfo {
			account: worker.clone(),
			owner: owner.clone(),
			reserved: initial_reserved_deposit,
			status: WorkerStatus::Registered,
			impl_name: [0, 0, 0, 0],
			impl_version: 0,
			attestation_method: None,
			attested_at: T::BlockNumber::default(),
		};

		<T as Config>::Currency::transfer(&owner, &worker, initial_deposit, ExistenceRequirement::KeepAlive)?;
		if !initial_reserved_deposit.is_zero() {
			<T as Config>::Currency::reserve(&worker, initial_reserved_deposit)?;
		}

		Workers::<T>::insert(&worker, worker_info);

		Self::deposit_event(Event::<T>::Registered { worker });
		Ok(())
	}

	fn do_deregister(owner: T::AccountId, worker: T::AccountId) -> DispatchResult {
		let worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::NotExists)?;
		Self::ensure_owner(&owner, &worker_info)?;
		ensure!(
			worker_info.status == WorkerStatus::Offline || worker_info.status == WorkerStatus::Registered,
			Error::<T>::NotOffline
		);

		let reserved = worker_info.reserved;
		if !reserved.is_zero() {
			// The upper limit is the actual reserved, so it is OK
			<T as Config>::Currency::unreserve(&worker, reserved);
		}
		<T as Config>::Currency::transfer(
			&worker,
			&owner,
			<T as Config>::Currency::free_balance(&worker),
			ExistenceRequirement::AllowDeath,
		)?;

		Workers::<T>::remove(&worker);

		Self::deposit_event(Event::<T>::Deregistered { worker, force: false });
		Ok(())
	}

	/// Transit a worker to `Online` status
	/// Check following things
	/// 1 Get the worker info by the caller
	/// 2 Check the worker's status (Must be `Registered`, and `Offline`)
	/// 3 Check the payload
	/// 4 Check the reserved (will try complement from free)
	/// 5 Check the attestation (the payload's signature is inside as payload)
	/// 6 Do `can_online` hook, will pass the payload
	/// Then
	/// 2 Update worker's info, persists to storage
	/// 3 Set flip-flop
	pub fn do_online(worker: T::AccountId, payload: OnlinePayload, attestation: Option<Attestation>) -> DispatchResult {
		let mut worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::NotExists)?;
		Self::ensure_worker(&worker, &worker_info)?;
		match worker_info.status {
			WorkerStatus::Registered | WorkerStatus::Offline => {},
			_ => return Err(Error::<T>::WrongStatus.into()),
		}

		if worker_info.impl_name == payload.impl_name {
			ensure!(worker_info.impl_version <= payload.impl_version, Error::<T>::WorkerImplCanNotDowngrade);
		}

		if T::ValidateWorkerImpl::get() {
			let Some(impl_permission) = WorkerImplPermissions::<T>::get(payload.impl_name) else {
				return Err(Error::<T>::WorkerImplUnsupported.into())
			};

			ensure!(
				payload.impl_version >= impl_permission.oldest_version &&
					payload.impl_version <= impl_permission.latest_version &&
					!impl_permission.blocked_versions.contains(&payload.impl_version),
				Error::<T>::WorkerImplBlocked
			)
		}

		// Check reserved money
		let reserved = <T as Config>::Currency::reserved_balance(&worker);
		if reserved < worker_info.reserved {
			// Try add reserved from free
			let free = <T as Config>::Currency::free_balance(&worker);
			let should_add_reserve = worker_info.reserved.saturating_sub(reserved);
			ensure!(free >= should_add_reserve, Error::<T>::InsufficientReserved);
			<T as Config>::Currency::reserve(&worker, should_add_reserve)?;
		}

		Self::ensure_attestation_provided(&attestation)?;
		let verified_attestation = Self::verify_attestation(&attestation)?;
		Self::verify_online_payload(&worker, &payload, &verified_attestation)?;
		T::WorkerLifecycleHooks::can_online(&worker, &payload, &verified_attestation)?;

		let attestation_method: Option<AttestationMethod> = attestation.map(|a| a.method());

		worker_info.impl_name = payload.impl_name;
		worker_info.impl_version = payload.impl_version;
		worker_info.attestation_method = attestation_method.clone();
		worker_info.attested_at = frame_system::Pallet::<T>::block_number();
		worker_info.status = WorkerStatus::Online;

		Workers::<T>::insert(&worker, worker_info);

		let next_heartbeat = Self::flip_flop_for_online(&worker);

		Self::deposit_event(Event::<T>::Online {
			worker: worker.clone(),
			impl_name: payload.impl_name,
			impl_version: payload.impl_version,
			attestation_method,
			next_heartbeat,
		});

		T::WorkerLifecycleHooks::after_online(&worker);

		Ok(())
	}

	fn do_refresh_attestation(
		worker: T::AccountId,
		payload: OnlinePayload,
		attestation: Option<Attestation>,
	) -> DispatchResult {
		let mut worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::NotExists)?;
		Self::ensure_worker(&worker, &worker_info)?;

		if worker_info.attestation_method.is_none() {
			return Ok(())
		}

		ensure!(worker_info.impl_name == payload.impl_name, Error::<T>::WorkerImplChanged);
		ensure!(worker_info.impl_version == payload.impl_version, Error::<T>::WorkerImplChanged);

		// Should we validate the impl here?

		Self::ensure_attestation_method(&attestation, &worker_info)?;
		let verified_attestation = Self::verify_attestation(&attestation)?;
		Self::verify_online_payload(&worker, &payload, &verified_attestation)?;

		worker_info.attested_at = frame_system::Pallet::<T>::block_number();
		Workers::<T>::insert(&worker, worker_info);

		Self::deposit_event(Event::<T>::AttestationRefreshed { worker: worker.clone() });

		T::WorkerLifecycleHooks::after_refresh_attestation(&worker, &payload, &verified_attestation);

		Ok(())
	}

	/// Transit worker to `Offline` status
	pub fn do_request_offline(worker: T::AccountId, owner: Option<T::AccountId>) -> DispatchResult {
		let worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::NotExists)?;
		Self::ensure_worker(&worker, &worker_info)?;

		if let Some(owner) = owner {
			Self::ensure_owner(&owner, &worker_info)?;
		}

		ensure!(
			matches!(worker_info.status, WorkerStatus::Online | WorkerStatus::RequestingOffline),
			Error::<T>::NotOnline
		);

		if T::WorkerLifecycleHooks::can_offline(&worker) {
			T::WorkerLifecycleHooks::before_offline(&worker, OfflineReason::Graceful);
			Self::offline_worker(&worker);

			Self::deposit_event(Event::<T>::Offline { worker, reason: OfflineReason::Graceful });
		} else {
			ensure!(
				worker_info.status == WorkerStatus::Online,
				Error::<T>::AlreadyRequestedOffline
			);

			// the worker should keep sending heartbeat until get permission to offline
			Workers::<T>::mutate(&worker, |worker_info| {
				if let Some(mut info) = worker_info.as_mut() {
					info.status = WorkerStatus::RequestingOffline;
				}
			});

			Self::deposit_event(Event::<T>::RequestingOffline { worker: worker.clone() });

			T::WorkerLifecycleHooks::after_requesting_offline(&worker);
		}

		Ok(())
	}

	pub fn do_force_offline(worker: T::AccountId, owner: Option<T::AccountId>) -> DispatchResult {
		let worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::NotExists)?;
		Self::ensure_worker(&worker, &worker_info)?;

		if let Some(owner) = owner {
			Self::ensure_owner(&owner, &worker_info)?;
		}

		ensure!(
			matches!(worker_info.status, WorkerStatus::Online | WorkerStatus::RequestingOffline),
			Error::<T>::NotOnline
		);

		T::WorkerLifecycleHooks::before_offline(&worker, OfflineReason::Forced);
		Self::offline_worker(&worker);

		Self::deposit_event(Event::<T>::Offline { worker, reason: OfflineReason::Forced });
		Ok(())
	}

	pub fn do_heartbeat(worker: T::AccountId) -> DispatchResult {
		let worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::NotExists)?;
		Self::ensure_worker(&worker, &worker_info)?;
		ensure!(
			matches!(worker_info.status, WorkerStatus::Online | WorkerStatus::RequestingOffline),
			Error::<T>::NotOnline
		);

		let current_block = frame_system::Pallet::<T>::block_number();

		// Check whether attestation expired, if yes, treat as force offline
		if current_block - worker_info.attested_at > T::AttestationValidityDuration::get().into() {
			T::WorkerLifecycleHooks::before_offline(&worker, OfflineReason::AttestationExpired);
			Self::offline_worker(&worker);

			Self::deposit_event(Event::<T>::Offline { worker, reason: OfflineReason::AttestationExpired });
			return Ok(())
		}

		// Check whether can offline now, We ignore error here
		if worker_info.status == WorkerStatus::RequestingOffline &&
			T::WorkerLifecycleHooks::can_offline(&worker)
		{
			T::WorkerLifecycleHooks::before_offline(&worker, OfflineReason::Graceful);
			Self::offline_worker(&worker);

			Self::deposit_event(Event::<T>::Offline { worker, reason: OfflineReason::Graceful });
			return Ok(())
		}

		// Check the worker's reserved money
		if <T as Config>::Currency::reserved_balance(&worker) < T::ReservedDeposit::get() {
			T::WorkerLifecycleHooks::before_offline(&worker, OfflineReason::InsufficientReservedFunds);
			Self::offline_worker(&worker);

			Self::deposit_event(Event::<T>::Offline { worker, reason: OfflineReason::InsufficientReservedFunds });
			return Ok(())
		}

		if T::ValidateWorkerImpl::get() {
			let valid_impl = if let Some(impl_permission) = WorkerImplPermissions::<T>::get(worker_info.impl_name) {
				worker_info.impl_version >= impl_permission.oldest_version &&
					worker_info.impl_version <= impl_permission.latest_version &&
					!impl_permission.blocked_versions.contains(&worker_info.impl_version)
			} else {
				false
			};

			if !valid_impl {
				T::WorkerLifecycleHooks::before_offline(&worker, OfflineReason::WorkerImplBlocked);
				Self::offline_worker(&worker);

				Self::deposit_event(Event::<T>::Offline { worker, reason: OfflineReason::WorkerImplBlocked });
				return Ok(())
			}
		}

		let next_heartbeat = Self::generate_next_heartbeat_block();
		let stage = FlipOrFlop::<T>::get();
		match stage {
			FlipFlopStage::Flip => {
				let Some(flip) = FlipSet::<T>::get(&worker) else {
					return Err(Error::<T>::HeartbeatAlreadySent.into())
				};
				ensure!(flip <= current_block, Error::<T>::TooEarly);

				FlipSet::<T>::remove(&worker);
				FlopSet::<T>::insert(&worker, next_heartbeat);
			},
			FlipFlopStage::Flop => {
				let Some(flop) = FlopSet::<T>::get(&worker) else {
					return Err(Error::<T>::HeartbeatAlreadySent.into())
				};
				ensure!(flop <= current_block, Error::<T>::TooEarly);

				FlopSet::<T>::remove(&worker);
				FlipSet::<T>::insert(&worker, next_heartbeat);
			},
			_ => return Err(Error::<T>::TooEarly.into()),
		}

		Self::deposit_event(Event::<T>::HeartbeatReceived { worker, next_heartbeat });

		Ok(())
	}

	fn do_set_worker_impl_permission(
		impl_name: WorkerImplName,
		impl_permission: Option<WorkerImplPermission>,
	) -> DispatchResult {
		let Some(impl_permission) = impl_permission else {
			WorkerImplPermissions::<T>::remove(impl_name);
			Self::deposit_event(
				Event::<T>::WorkerImplPermissionRemoved { impl_name }
			);
			return Ok(())
		};

		WorkerImplPermissions::<T>::insert(impl_name, impl_permission);
		Self::deposit_event(Event::<T>::WorkerImplPermissionUpdated { impl_name });

		Ok(())
	}

	fn do_set_worker_impl_hashes(
		impl_name: WorkerImplName,
		impl_version: WorkerImplVersion,
		impl_hash: Option<WorkerImplHash>,
	) -> DispatchResult {
		let Some(impl_hash) = impl_hash else {
			WorkerImplHashes::<T>::remove(impl_name, impl_version);
			Self::deposit_event(
				Event::<T>::WorkerImplHashRemoved { impl_name, impl_version }
			);
			return Ok(())
		};

		WorkerImplHashes::<T>::insert(impl_name, impl_version, impl_hash);
		Self::deposit_event(Event::<T>::WorkerImplHashUpdated { impl_name, impl_version });

		Ok(())
	}

	fn offline_worker(worker: &T::AccountId) {
		FlipSet::<T>::remove(worker);
		FlopSet::<T>::remove(worker);
		Workers::<T>::mutate(worker, |worker_info| {
			if let Some(mut info) = worker_info.as_mut() {
				info.status = WorkerStatus::Offline;
			}
		});
	}

	fn flip_flop_for_online(worker: &T::AccountId) -> T::BlockNumber {
		let next_heartbeat = Self::generate_next_heartbeat_block();
		let stage = FlipOrFlop::<T>::get();
		match stage {
			FlipFlopStage::Flip | FlipFlopStage::FlopToFlip => {
				FlopSet::<T>::insert(worker, next_heartbeat);
			},
			FlipFlopStage::Flop | FlipFlopStage::FlipToFlop => {
				FlipSet::<T>::insert(worker, next_heartbeat);
			},
		}

		next_heartbeat
	}

	fn handle_worker_unresponsive(worker: &T::AccountId) {
		T::WorkerLifecycleHooks::before_offline(worker, OfflineReason::Unresponsive);

		Workers::<T>::mutate(worker, |worker_info| {
			if let Some(mut info) = worker_info.as_mut() {
				info.status = WorkerStatus::Offline;
			}
		});

		Self::deposit_event(Event::<T>::Offline { worker: worker.clone(), reason: OfflineReason::Unresponsive });
	}

	fn verify_attestation(attestation: &Option<Attestation>) -> Result<Option<VerifiedAttestation>, DispatchError> {
		let Some(attestation) = attestation else {
			return Ok(None)
		};

		let now = T::UnixTime::now().as_millis().saturated_into::<u64>();
		let verified = attestation.verify(now);
		match verified {
			Ok(verified) => Ok(Some(verified)),
			Err(AttestationError::Expired) => Err(Error::<T>::ExpiredAttestation.into()),
			Err(AttestationError::Invalid) => Err(Error::<T>::InvalidAttestation.into()),
		}
	}

	fn verify_online_payload(
		worker: &T::AccountId,
		payload: &OnlinePayload,
		verified_attestation: &Option<VerifiedAttestation>,
	) -> DispatchResult {
		let Some(verified_attestation) = verified_attestation else {
			return Ok(())
		};

		let encode_worker = T::AccountId::encode(worker);
		let h256_worker = H256::from_slice(&encode_worker);
		let worker_public_key = sr25519::Public::from_h256(h256_worker);

		let encoded_message = Encode::encode(payload);

		let Some(signature) = sr25519::Signature::from_slice(verified_attestation.payload()) else {
			return Err(Error::<T>::CanNotVerifyPayload.into())
		};

		ensure!(
			sr25519_verify(&signature, &encoded_message, &worker_public_key),
			Error::<T>::PayloadSignatureMismatched
		);

		Ok(())
	}

	fn ensure_owner(who: &T::AccountId, worker_info: &WorkerInfo<T::AccountId, BalanceOf<T>, T::BlockNumber>) -> DispatchResult {
		ensure!(*who == worker_info.owner, Error::<T>::NotTheOwner);
		Ok(())
	}

	fn ensure_worker(who: &T::AccountId, worker_info: &WorkerInfo<T::AccountId, BalanceOf<T>, T::BlockNumber>) -> DispatchResult {
		ensure!(*who == worker_info.account, Error::<T>::NotTheWorker);
		Ok(())
	}

	fn ensure_attestation_provided(attestation: &Option<Attestation>) -> DispatchResult {
		let Some(attestation) = attestation else {
			ensure!(
				!T::DisallowOptOutAttestation::get() || attestation.is_some(),
				Error::<T>::MustProvideAttestation
			);
			return Ok(())
		};

		if attestation.method() == AttestationMethod::NonTEE {
			ensure!(!T::DisallowNonTEEAttestation::get(), Error::<T>::DisallowNonTEEAttestation);
		}

		Ok(())
	}

	fn ensure_attestation_method(attestation: &Option<Attestation>, worker_info: &WorkerInfo<T::AccountId, BalanceOf<T>, T::BlockNumber>) -> DispatchResult {
		let Some(worker_attestation_method) = worker_info.attestation_method.clone() else {
			ensure!(attestation.is_none(), Error::<T>::AttestationMethodChanged);
			return Ok(())
		};

		let Some(attestation) = attestation else {
			return Err(Error::<T>::AttestationMethodChanged.into())
		};

		ensure!(attestation.method() == worker_attestation_method, Error::<T>::AttestationMethodChanged);

		Ok(())
	}

	/// This function copied from pallet_lottery
	///
	/// Generate a random number from a given seed.
	/// Note that there is potential bias introduced by using modulus operator.
	/// You should call this function with different seed values until the random
	/// number lies within `u32::MAX - u32::MAX % n`.
	/// TODO: deal with randomness freshness
	/// https://github.com/paritytech/substrate/issues/8311
	fn generate_random_number(seed: u32) -> u32 {
		let (random_seed, _) = T::Randomness::random(&(b"computing_workers", seed).encode());
		let random_number =
			<u32>::decode(&mut random_seed.as_ref()).expect("secure hashes should always be bigger than u32; qed");
		// log!(info, "Random number: {}", random_number);

		random_number
	}

	fn generate_next_heartbeat_block() -> T::BlockNumber {
		let current_flip_flop_started_at = CurrentFlipFlopStartedAt::<T>::get();
		let duration = T::CollectingHeartbeatsDuration::get();
		let random_delay = Self::generate_random_number(0) % (duration * 4 / 5); // Give ~20% room

		current_flip_flop_started_at + (duration + random_delay).into()
	}
}

impl<T: Config> WorkerManageable<T> for Pallet<T> {
	fn worker_info(worker: &T::AccountId) -> Option<WorkerInfo<T::AccountId, BalanceOf<T>, T::BlockNumber>> {
		Workers::<T>::get(worker)
	}

	fn worker_exists(worker: &T::AccountId) -> bool {
		Workers::<T>::contains_key(worker)
	}

	fn reward(worker: &T::AccountId, source: &T::AccountId, value: BalanceOf<T>) -> DispatchResult {
		<T as Config>::Currency::transfer(source, worker, value, ExistenceRequirement::KeepAlive)
	}

	fn slash(worker: &T::AccountId, value: BalanceOf<T>) -> (NegativeImbalanceOf<T>, BalanceOf<T>) {
		<T as Config>::Currency::slash(worker, value)
	}

	fn offline(worker: &T::AccountId, reason: Option<Vec<u8>>) -> DispatchResult {
		let mut worker_info = Workers::<T>::get(worker).ok_or(Error::<T>::NotExists)?;
		ensure!(
			matches!(worker_info.status, WorkerStatus::Online | WorkerStatus::RequestingOffline),
			Error::<T>::NotOnline
		);

		worker_info.status = WorkerStatus::Offline;
		Workers::<T>::insert(worker, worker_info);

		FlipSet::<T>::remove(worker);
		FlopSet::<T>::remove(worker);

		Self::deposit_event(Event::<T>::Offline { worker: worker.clone(), reason: OfflineReason::Other(reason) });

		Ok(())
	}
}
