//! Benchmarking setup for pallet-computing_workers

// Only enable this module for benchmarking.
#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{account, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::benchmarking::*;
use frame_system::{Account, RawOrigin};

use frame_support::{
	sp_runtime::{
		app_crypto::{sr25519, KeyTypeId, RuntimePublic},
		SaturatedConversion, Saturating
	},
	assert_ok, fail,
};
use primitives::{
	AttestationMethod, AttestationPayload, ExtraOnlinePayload, FlipFlopStage, NonTEEAttestation, OnlinePayload,
	WorkerStatus,
};

use crate::Pallet as ComputingWorkers;
use super::*;

const DOLLARS: u128 = 1_000_000_000_000;
const WORKER_KEY_TYPE: KeyTypeId = KeyTypeId(*b"work");

fn mock_online_payload_and_attestation<T: Config>(
	worker_public: &sr25519::Public,
) -> (OnlinePayload, Option<Attestation>) {
	let payload = OnlinePayload { impl_name: *b"mock", impl_version: 1, extra: ExtraOnlinePayload::default() };

	let encoded_payload = Encode::encode(&payload);
	let signature = worker_public.sign(WORKER_KEY_TYPE, &encoded_payload).unwrap();

	let attestation = Attestation::NonTEE(NonTEEAttestation {
		issued_at: T::UnixTime::now().as_millis().saturated_into::<u64>() - 1000,
		payload: AttestationPayload::truncate_from(signature.0.to_vec()),
	});

	(payload, Some(attestation))
}

fn add_mock_worker<T: Config>(worker_public: &sr25519::Public, owner: &T::AccountId) -> T::AccountId {
	let worker = T::AccountId::decode(&mut worker_public.encode().as_slice()).unwrap();
	let reserved_deposit = T::ReservedDeposit::get();

	let owner_balance = reserved_deposit.saturating_add((50 * DOLLARS).saturated_into::<BalanceOf<T>>());
	let _ = T::Currency::make_free_balance_be(&owner, owner_balance);

	let initial_deposit = reserved_deposit.saturating_add((10 * DOLLARS).saturated_into::<BalanceOf<T>>());

	assert_ok!(ComputingWorkers::<T>::register(
		RawOrigin::Signed(owner.clone()).into(),
		worker.clone(),
		initial_deposit
	));
	assert_eq!(Workers::<T>::contains_key(&worker), true);

	worker
}

fn add_mock_online_worker<T: Config>(worker_public: &sr25519::Public, owner: &T::AccountId) -> T::AccountId {
	let worker = add_mock_worker::<T>(worker_public, owner);

	let (payload, attestation) = mock_online_payload_and_attestation::<T>(worker_public);
	assert_ok!(ComputingWorkers::<T>::online(RawOrigin::Signed(worker.clone()).into(), payload, attestation));

	let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
	assert_eq!(worker_info.attestation_method, Some(AttestationMethod::NonTEE));

	worker
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn register() {
		let owner: T::AccountId = whitelisted_caller();
		let worker = account::<T::AccountId>("worker", 0, 0);

		let reserved_deposit = T::ReservedDeposit::get();
		let balance = reserved_deposit.saturating_add((1 * DOLLARS).saturated_into::<BalanceOf<T>>());
		let _ = T::Currency::make_free_balance_be(&owner, balance);

		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), worker.clone(), reserved_deposit);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.owner, owner);
		assert_eq!(T::Currency::reserved_balance(&worker), reserved_deposit);
		assert_eq!(worker_info.status, WorkerStatus::Registered);
	}

	#[benchmark]
	fn deregister() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), worker.clone());

		assert_eq!(Workers::<T>::contains_key(&worker), false);
		assert_eq!(Account::<T>::contains_key(&worker), false);
	}

	#[benchmark]
	fn deposit() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_worker::<T>(&worker_public, &owner);

		let worker_balance = T::Currency::free_balance(&worker);
		let amount = (10 * DOLLARS).saturated_into::<BalanceOf<T>>();

		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), worker.clone(), amount);

		assert_eq!(
			T::Currency::free_balance(&worker),
			worker_balance.saturating_add(amount)
		);
	}

	#[benchmark]
	fn withdraw() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_worker::<T>(&worker_public, &owner);

		let worker_balance = T::Currency::free_balance(&worker);
		let amount = (10 * DOLLARS).saturated_into::<BalanceOf<T>>();

		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), worker.clone(), amount);

		assert_eq!(
			T::Currency::free_balance(&worker),
			worker_balance.saturating_sub(amount)
		);
	}

	#[benchmark]
	fn online() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_worker::<T>(&worker_public, &owner);
		let (payload, attestation) = mock_online_payload_and_attestation::<T>(&worker_public);

		#[extrinsic_call]
		_(RawOrigin::Signed(worker.clone()), payload, attestation);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.attestation_method, Some(AttestationMethod::NonTEE));
		assert_eq!(worker_info.status, WorkerStatus::Online);
	}

	#[benchmark]
	fn refresh_attestation() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);
		let (payload, attestation) = mock_online_payload_and_attestation::<T>(&worker_public);
		let current_block = frame_system::Pallet::<T>::block_number();

		#[extrinsic_call]
		_(RawOrigin::Signed(worker.clone()), payload, attestation);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.attestation_method, Some(AttestationMethod::NonTEE));
		assert!(worker_info.attested_at > current_block)
	}

	// This is the slow path,
	// worker shall offline immediately instead of becoming `RequestingOffline`
	#[benchmark]
	fn request_offline() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(RawOrigin::Signed(worker.clone()));

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.status, WorkerStatus::Offline);
	}

	// This is the slow path,
	// worker shall offline immediately instead of becoming `RequestingOffline`
	#[benchmark]
	fn request_offline_for() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), worker.clone());

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.status, WorkerStatus::Offline);
	}

	#[benchmark]
	fn force_offline() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(RawOrigin::Signed(worker.clone()));

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.status, WorkerStatus::Offline);
	}

	#[benchmark]
	fn force_offline_for() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), worker.clone());

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.status, WorkerStatus::Offline);
	}

	// This is the normal path
	#[benchmark]
	fn heartbeat() {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);

		let stage = FlipOrFlop::<T>::get();
		// Simulate to the next stage
		match stage {
			FlipFlopStage::Flip => {
				assert_eq!(FlopSet::<T>::contains_key(&worker), true);
				FlopSet::<T>::insert(&worker, T::BlockNumber::zero());
				FlipOrFlop::<T>::set(FlipFlopStage::Flop);
			},
			FlipFlopStage::Flop => {
				assert_eq!(FlipSet::<T>::contains_key(&worker), true);
				FlipSet::<T>::insert(&worker, T::BlockNumber::zero());
				FlipOrFlop::<T>::set(FlipFlopStage::Flip);
			},
			_ => fail!("Other stages is unexpected")
		};

		# [extrinsic_call]
		_(RawOrigin::Signed(worker.clone()));

		let stage = FlipOrFlop::<T>::get();
		match stage {
			FlipFlopStage::Flip => {
				assert_eq!(FlipSet::<T>::contains_key(&worker), false);
				assert_eq!(FlopSet::<T>::contains_key(&worker), true);
			},
			FlipFlopStage::Flop => {
				assert_eq!(FlipSet::<T>::contains_key(&worker), true);
				assert_eq!(FlopSet::<T>::contains_key(&worker), false);
			},
			_ => fail!("Other stages is unexpected")
		};
	}

	// TODO: benchmark other paths of heartbeat

	impl_benchmark_test_suite! {
		ComputingWorkers,
		crate::mock::new_test_ext(),
		crate::mock::Test
	}
}
