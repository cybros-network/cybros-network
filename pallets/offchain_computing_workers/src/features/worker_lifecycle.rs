use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
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
	pub fn do_online(
		worker: T::AccountId, payload: OnlinePayload<T::ImplId>,
		attestation: Attestation
	) -> DispatchResult {
		let mut worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::WorkerNotFound)?;
		Self::ensure_worker(&worker, &worker_info)?;
		match worker_info.status {
			WorkerStatus::Registered | WorkerStatus::Offline => {},
			_ => return Err(Error::<T>::WrongStatus.into()),
		}

		if T::DisallowOptOutAttestation::get() {
			ensure!(
				attestation.method() != AttestationMethod::OptOut,
				Error::<T>::OptOutAttestationDisallowed
			);
		}

		ensure!(
			worker_info.impl_id == payload.impl_id.clone(),
			Error::<T>::ImplMismatched
		);

		let mut impl_build_info = ImplBuilds::<T>::get(&worker_info.impl_id, payload.impl_build_version).ok_or(Error::<T>::ImplBuildNotFound)?;
		ensure!(
			impl_build_info.status == ImplBuildStatus::Released,
			Error::<T>::ImplBuildRestricted
		);
		if let Some(magic_bytes) = impl_build_info.magic_bytes.clone() {
			ensure!(
				magic_bytes == payload.impl_build_magic_bytes,
				Error::<T>::ImplBuildMagicBytesMismatched
			);
		}

		// Check reserved money
		let deposit = <T as Config>::Currency::reserved_balance(&worker);
		if deposit < worker_info.deposit {
			// Try add reserved from free
			let free = <T as Config>::Currency::free_balance(&worker);
			let should_add_deposit = worker_info.deposit.saturating_sub(deposit);
			ensure!(free >= should_add_deposit, Error::<T>::InsufficientDeposit);
			<T as Config>::Currency::reserve(&worker, should_add_deposit)?;
		}

		let verified_attestation = Self::verify_attestation(&attestation)?;
		Self::verify_online_payload(&worker, &payload, &verified_attestation)?;
		T::OffchainWorkerLifecycleHooks::can_online(&worker, &payload, &verified_attestation)?;

		worker_info.impl_spec_version = Some(payload.impl_spec_version);
		worker_info.impl_build_version = Some(payload.impl_build_version);
		worker_info.attestation_method = Some(attestation.method());
		worker_info.attestation_expires_at = verified_attestation.expires_at();
		worker_info.attested_at = Some(T::UnixTime::now().as_secs().saturated_into::<u64>());
		worker_info.status = WorkerStatus::Online;
		Workers::<T>::insert(&worker, worker_info);

		impl_build_info.workers_count += 1;
		ImplBuilds::<T>::insert(&payload.impl_id, &payload.impl_build_version, impl_build_info);

		let next_heartbeat = Self::flip_flop_for_online(&worker);

		Self::deposit_event(Event::<T>::WorkerOnline {
			worker: worker.clone(),
			impl_spec_version: payload.impl_spec_version,
			impl_build_version: payload.impl_build_version,
			attestation_method: attestation.method(),
			attestation_expires_at: verified_attestation.expires_at(),
			next_heartbeat,
		});

		T::OffchainWorkerLifecycleHooks::after_online(&worker);

		Ok(())
	}

	pub(crate) fn do_refresh_attestation(
		worker: T::AccountId,
		payload: OnlinePayload<T::ImplId>,
		attestation: Attestation,
	) -> DispatchResult {
		let mut worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::WorkerNotFound)?;
		Self::ensure_worker(&worker, &worker_info)?;

		ensure!(
			worker_info.attestation_expires_at.is_some(),
			Error::<T>::AttestationNeverExpire
		);

		ensure!(
			worker_info.impl_id == payload.impl_id &&
			worker_info.impl_spec_version == Some(payload.impl_spec_version) &&
			worker_info.impl_build_version == Some(payload.impl_build_version),
			Error::<T>::ImplBuildChanged
		);
		// Should we validate the impl here?

		Self::ensure_attestation_method(&attestation, &worker_info)?;
		let verified_attestation = Self::verify_attestation(&attestation)?;
		Self::verify_online_payload(&worker, &payload, &verified_attestation)?;

		worker_info.attestation_expires_at = verified_attestation.expires_at();
		worker_info.attested_at = Some(T::UnixTime::now().as_secs().saturated_into::<u64>());
		Workers::<T>::insert(&worker, worker_info.clone());

		Self::deposit_event(Event::<T>::WorkerAttestationRefreshed { worker: worker.clone(), expires_at: verified_attestation.expires_at() });

		T::OffchainWorkerLifecycleHooks::after_refresh_attestation(&worker, &payload, &verified_attestation);

		Ok(())
	}

	/// Transit worker to `Offline` status
	pub(crate) fn do_request_offline(
		worker: T::AccountId,
		owner: Option<T::AccountId>
	) -> DispatchResult {
		let worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::WorkerNotFound)?;
		Self::ensure_worker(&worker, &worker_info)?;

		if let Some(owner) = owner {
			Self::ensure_owner(&owner, &worker_info)?;
		}

		ensure!(
			matches!(worker_info.status, WorkerStatus::Online | WorkerStatus::RequestingOffline),
			Error::<T>::WorkerNotOnline
		);

		if T::OffchainWorkerLifecycleHooks::can_offline(&worker) {
			let Some(impl_build_version) = worker_info.impl_build_version else {
				return Err(Error::<T>::InternalError.into())
			};

			T::OffchainWorkerLifecycleHooks::before_offline(&worker, OfflineReason::Graceful);
			Self::offline_worker(&worker, &worker_info.impl_id, &impl_build_version, OfflineReason::Graceful);
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

			Self::deposit_event(Event::<T>::WorkerRequestingOffline { worker: worker.clone() });

			T::OffchainWorkerLifecycleHooks::after_requesting_offline(&worker);
		}

		Ok(())
	}

	pub(crate) fn do_force_offline(
		worker: T::AccountId,
		owner: Option<T::AccountId>
	) -> DispatchResult {
		let worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::WorkerNotFound)?;
		Self::ensure_worker(&worker, &worker_info)?;

		if let Some(owner) = owner {
			Self::ensure_owner(&owner, &worker_info)?;
		}

		ensure!(
			matches!(worker_info.status, WorkerStatus::Online | WorkerStatus::RequestingOffline),
			Error::<T>::WorkerNotOnline
		);

		let Some(impl_build_version) = worker_info.impl_build_version else {
			return Err(Error::<T>::InternalError.into())
		};

		T::OffchainWorkerLifecycleHooks::before_offline(&worker, OfflineReason::Forced);
		Self::offline_worker(&worker, &worker_info.impl_id, &impl_build_version, OfflineReason::Forced);

		Ok(())
	}

	pub(crate) fn do_heartbeat(
		worker: T::AccountId
	) -> DispatchResult {
		let worker_info = Workers::<T>::get(&worker).ok_or(Error::<T>::WorkerNotFound)?;
		Self::ensure_worker(&worker, &worker_info)?;
		ensure!(
			matches!(worker_info.status, WorkerStatus::Online | WorkerStatus::RequestingOffline),
			Error::<T>::WorkerNotOnline
		);

		let current_block = frame_system::Pallet::<T>::block_number();
		let now = T::UnixTime::now().as_secs().saturated_into::<u64>();

		let Some(impl_build_version) = worker_info.impl_build_version else {
			return Err(Error::<T>::InternalError.into())
		};

		if let Some(attestation_expires_at) = worker_info.attestation_expires_at {
			if now >= attestation_expires_at {
				T::OffchainWorkerLifecycleHooks::before_offline(&worker, OfflineReason::AttestationExpired);
				Self::offline_worker(&worker, &worker_info.impl_id, &impl_build_version, OfflineReason::AttestationExpired);

				return Ok(())
			}
		}

		// Check whether can offline now, We ignore error here
		if worker_info.status == WorkerStatus::RequestingOffline &&
			T::OffchainWorkerLifecycleHooks::can_offline(&worker)
		{
			T::OffchainWorkerLifecycleHooks::before_offline(&worker, OfflineReason::Graceful);
			Self::offline_worker(&worker, &worker_info.impl_id, &impl_build_version, OfflineReason::Graceful);

			return Ok(())
		}

		// Check the worker's reserved money
		if <T as Config>::Currency::reserved_balance(&worker) < T::RegisterWorkerDeposit::get() {
			T::OffchainWorkerLifecycleHooks::before_offline(&worker, OfflineReason::InsufficientDepositFunds);
			Self::offline_worker(&worker, &worker_info.impl_id, &impl_build_version, OfflineReason::InsufficientDepositFunds);

			return Ok(())
		}

		let impl_build_info = ImplBuilds::<T>::get(&worker_info.impl_id, &impl_build_version).ok_or(Error::<T>::InternalError)?;
		let valid_impl_build = match impl_build_info.status {
			ImplBuildStatus::Released | ImplBuildStatus::Deprecated => true,
			_ => false
		};

		if !valid_impl_build {
			T::OffchainWorkerLifecycleHooks::before_offline(&worker, OfflineReason::ImplBlocked);
			Self::offline_worker(&worker, &worker_info.impl_id, &impl_build_version, OfflineReason::ImplBlocked);

			return Ok(())
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

		Self::deposit_event(Event::<T>::WorkerHeartbeatReceived { worker, next: next_heartbeat });

		Ok(())
	}
}
