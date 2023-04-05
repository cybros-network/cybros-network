//! Benchmarking setup for pallet-offchain_computing_workers

// Only enable this module for benchmarking.
#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{v2::*, account, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{Account, RawOrigin};

use frame_support::{
	sp_runtime::{
		app_crypto::{sr25519, KeyTypeId, RuntimePublic},
		SaturatedConversion, Saturating
	},
	assert_ok, fail,
};
use primitives::{
	AttestationMethod, AttestationPayload, ExtraOnlinePayload, FlipFlopStage, OnlinePayload,
	WorkerStatus,
};

use crate::Pallet as OffchainComputingWorkers;
use super::*;

const DOLLARS: u128 = 1_000_000_000_000;
const WORKER_KEY_TYPE: KeyTypeId = KeyTypeId(*b"work");

fn mock_online_payload_and_attestation<T: Config>(
	worker_public: &sr25519::Public,
) -> (OnlinePayload<T::ImplId>, Attestation) {
	let payload = OnlinePayload {
		impl_id: 1,
		impl_spec_version: 0,
		impl_build_version: 1,
		impl_build_magic_bytes: Default::default()
	};

	let attestation = Attestation::OptOut;

	(payload, attestation)
}

fn add_mock_worker<T: Config>(worker_public: &sr25519::Public, owner: &T::AccountId) -> T::AccountId {
	let worker = T::AccountId::decode(&mut worker_public.encode().as_slice()).unwrap();
	let reserved_deposit = T::RegisterWorkerDeposit::get();

	let owner_balance = reserved_deposit.saturating_add((100 * DOLLARS).saturated_into::<BalanceOf<T>>());
	let _ = T::Currency::make_free_balance_be(&owner, owner_balance);

	let initial_deposit = reserved_deposit.saturating_add((11 * DOLLARS).saturated_into::<BalanceOf<T>>());

	assert_ok!(OffchainComputingWorkers::<T>::register(
		RawOrigin::Signed(owner.clone()).into(),
		T::Lookup::unlookup(worker.clone()),
		initial_deposit
	));
	assert_eq!(Workers::<T>::contains_key(&worker), true);

	worker
}

fn add_mock_online_worker<T: Config>(worker_public: &sr25519::Public, owner: &T::AccountId) -> T::AccountId {
	let worker = add_mock_worker::<T>(worker_public, owner);

	let (payload, attestation) = mock_online_payload_and_attestation::<T>(worker_public);
	assert_ok!(
		OffchainComputingWorkers::<T>::online(
			RawOrigin::Signed(worker.clone()).into(),
			payload,
			attestation
		)
	);

	let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
	assert_eq!(worker_info.attestation_method, Some(AttestationMethod::OptOut));

	worker
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn register() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker = account::<T::AccountId>("worker", 0, 0);

		let initial_balance = T::ReservedDeposit::get().saturating_add((1 * DOLLARS).saturated_into::<BalanceOf<T>>());
		let balance = initial_balance.saturating_add((100 * DOLLARS).saturated_into::<BalanceOf<T>>());
		let _ = T::Currency::make_free_balance_be(&owner, balance);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(owner.clone()),
			T::Lookup::unlookup(worker.clone()),
			initial_balance
		);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.owner, owner);
		assert_eq!(T::Currency::reserved_balance(&worker), T::ReservedDeposit::get());
		assert_eq!(worker_info.status, WorkerStatus::Registered);

		Ok(())
	}

	#[benchmark]
	fn deregister() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(owner.clone()),
			T::Lookup::unlookup(worker.clone())
		);

		assert_eq!(Workers::<T>::contains_key(&worker), false);
		assert_eq!(Account::<T>::contains_key(&worker), false);

		Ok(())
	}

	#[benchmark]
	fn deposit() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_worker::<T>(&worker_public, &owner);

		let worker_balance = T::Currency::free_balance(&worker);
		let amount = (10 * DOLLARS).saturated_into::<BalanceOf<T>>();

		#[extrinsic_call]
		_(
			RawOrigin::Signed(owner.clone()),
			T::Lookup::unlookup(worker.clone()),
			amount
		);

		assert_eq!(
			T::Currency::free_balance(&worker),
			worker_balance.saturating_add(amount)
		);

		Ok(())
	}

	#[benchmark]
	fn withdraw() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_worker::<T>(&worker_public, &owner);

		let worker_balance = T::Currency::free_balance(&worker);
		let amount = (10 * DOLLARS).saturated_into::<BalanceOf<T>>();

		#[extrinsic_call]
		_(
			RawOrigin::Signed(owner.clone()),
			T::Lookup::unlookup(worker.clone()),
			amount
		);

		assert_eq!(
			T::Currency::free_balance(&worker),
			worker_balance.saturating_sub(amount)
		);

		Ok(())
	}

	#[benchmark]
	fn online() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_worker::<T>(&worker_public, &owner);
		let (payload, attestation) = mock_online_payload_and_attestation::<T>(&worker_public);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(worker.clone()),
			payload,
			attestation
		);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.attestation_method, Some(AttestationMethod::OptOut));
		assert_eq!(worker_info.status, WorkerStatus::Online);

		Ok(())
	}

	#[benchmark]
	fn refresh_attestation() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);
		let (payload, attestation) = mock_online_payload_and_attestation::<T>(&worker_public);
		let current_block = frame_system::Pallet::<T>::block_number();

		#[extrinsic_call]
		_(
			RawOrigin::Signed(worker.clone()),
			payload,
			attestation
		);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.attestation_method, Some(AttestationMethod::OptOut));
		assert!(worker_info.attested_at > current_block);

		Ok(())
	}

	// This is the slow path,
	// worker shall offline immediately instead of becoming `RequestingOffline`
	#[benchmark]
	fn request_offline() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(worker.clone())
		);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.status, WorkerStatus::Offline);

		Ok(())
	}

	// This is the slow path,
	// worker shall offline immediately instead of becoming `RequestingOffline`
	#[benchmark]
	fn request_offline_for() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(owner.clone()),
			T::Lookup::unlookup(worker.clone())
		);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.status, WorkerStatus::Offline);

		Ok(())
	}

	#[benchmark]
	fn force_offline() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(worker.clone())
		);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.status, WorkerStatus::Offline);

		Ok(())
	}

	#[benchmark]
	fn force_offline_for() -> Result<(), BenchmarkError> {
		let owner: T::AccountId = whitelisted_caller();
		let worker_public = sr25519::Public::generate_pair(WORKER_KEY_TYPE, None);
		let worker = add_mock_online_worker::<T>(&worker_public, &owner);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(owner.clone()),
			T::Lookup::unlookup(worker.clone())
		);

		let worker_info = Workers::<T>::get(&worker).expect("WorkerInfo should has value");
		assert_eq!(worker_info.status, WorkerStatus::Offline);

		Ok(())
	}

	// This is the normal path
	#[benchmark]
	fn heartbeat() -> Result<(), BenchmarkError> {
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

		#[extrinsic_call]
		_(
			RawOrigin::Signed(worker.clone())
		);

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

		Ok(())
	}

	// TODO: benchmark other paths of heartbeat

	impl_benchmark_test_suite! {
		OffchainComputingWorkers,
		crate::mock::new_test_ext(),
		crate::mock::Test
	}
}
