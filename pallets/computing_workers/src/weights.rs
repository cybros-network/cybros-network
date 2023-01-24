
//! Autogenerated weights for pallet_computing_workers
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-24, STEPS: `50`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/cybros-node
// benchmark
// pallet
// --pallet=pallet_computing_workers
// --chain=dev
// --steps=50
// --repeat=50
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/computing_workers/src/weights.rs
// --template=./templates/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_computing_workers.
pub trait WeightInfo {
	fn register() -> Weight;
	fn deregister() -> Weight;
	fn deposit() -> Weight;
	fn withdraw() -> Weight;
	fn online() -> Weight;
	fn refresh_attestation() -> Weight;
	fn request_offline() -> Weight;
	fn request_offline_for() -> Weight;
	fn force_offline() -> Weight;
	fn force_offline_for() -> Weight;
	fn heartbeat() -> Weight;
}

/// Weights for pallet_computing_workers using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ComputingWorkers CounterForWorkers (r:1 w:1)
	fn register() -> Weight {
		// Minimum execution time: 45_000 nanoseconds.
		Weight::from_ref_time(46_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ComputingWorkers CounterForWorkers (r:1 w:1)
	fn deregister() -> Weight {
		// Minimum execution time: 46_000 nanoseconds.
		Weight::from_ref_time(47_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: ComputingWorkers Workers (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		// Minimum execution time: 28_000 nanoseconds.
		Weight::from_ref_time(28_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ComputingWorkers Workers (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn withdraw() -> Weight {
		// Minimum execution time: 27_000 nanoseconds.
		Weight::from_ref_time(28_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ComputingWorkers CurrentFlipFlopStartedAt (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: ComputingWorkers FlipOrFlop (r:1 w:0)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn online() -> Weight {
		// Minimum execution time: 70_000 nanoseconds.
		Weight::from_ref_time(72_000_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn refresh_attestation() -> Weight {
		// Minimum execution time: 63_000 nanoseconds.
		Weight::from_ref_time(64_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn request_offline() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn request_offline_for() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(26_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn force_offline() -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn force_offline_for() -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: ComputingWorkers Workers (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: ComputingWorkers CurrentFlipFlopStartedAt (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: ComputingWorkers FlipOrFlop (r:1 w:0)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlipSet (r:1 w:1)
	fn heartbeat() -> Weight {
		// Minimum execution time: 34_000 nanoseconds.
		Weight::from_ref_time(35_000_000)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ComputingWorkers CounterForWorkers (r:1 w:1)
	fn register() -> Weight {
		// Minimum execution time: 45_000 nanoseconds.
		Weight::from_ref_time(46_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ComputingWorkers CounterForWorkers (r:1 w:1)
	fn deregister() -> Weight {
		// Minimum execution time: 46_000 nanoseconds.
		Weight::from_ref_time(47_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: ComputingWorkers Workers (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		// Minimum execution time: 28_000 nanoseconds.
		Weight::from_ref_time(28_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: ComputingWorkers Workers (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn withdraw() -> Weight {
		// Minimum execution time: 27_000 nanoseconds.
		Weight::from_ref_time(28_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ComputingWorkers CurrentFlipFlopStartedAt (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: ComputingWorkers FlipOrFlop (r:1 w:0)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn online() -> Weight {
		// Minimum execution time: 70_000 nanoseconds.
		Weight::from_ref_time(72_000_000)
			.saturating_add(RocksDbWeight::get().reads(8))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn refresh_attestation() -> Weight {
		// Minimum execution time: 63_000 nanoseconds.
		Weight::from_ref_time(64_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn request_offline() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn request_offline_for() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(26_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn force_offline() -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: ComputingWorkers Workers (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	fn force_offline_for() -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: ComputingWorkers Workers (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: ComputingWorkers CurrentFlipFlopStartedAt (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: ComputingWorkers FlipOrFlop (r:1 w:0)
	// Storage: ComputingWorkers FlopSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlopSet (r:1 w:1)
	// Storage: ComputingWorkers FlipSet (r:1 w:1)
	// Storage: ComputingWorkers CounterForFlipSet (r:1 w:1)
	fn heartbeat() -> Weight {
		// Minimum execution time: 34_000 nanoseconds.
		Weight::from_ref_time(35_000_000)
			.saturating_add(RocksDbWeight::get().reads(9))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
}
