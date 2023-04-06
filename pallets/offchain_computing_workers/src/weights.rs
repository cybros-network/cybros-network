
//! Autogenerated weights for pallet_offchain_computing_workers
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-06, STEPS: `50`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
//    ./target/production/node
//    benchmark
//    pallet
//    --pallet=pallet_offchain_computing_workers
//    --extrinsic=*
//    --chain=dev
//    --steps=50
//    --repeat=50
//    --no-storage-info
//    --no-median-slopes
//    --no-min-squares
//    --execution=wasm
//    --wasm-execution=compiled
//    --heap-pages=4096
//    --output=./pallets/offchain_computing_workers/src/weights.rs
//    --template=./pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_offchain_computing_workers.
pub trait WeightInfo {
    fn register_worker() -> Weight;
    fn deregister_worker() -> Weight;
    fn transfer_to_worker() -> Weight;
    fn withdraw_from_worker() -> Weight;
    fn online() -> Weight;
    fn refresh_attestation() -> Weight;
    fn request_offline() -> Weight;
    fn request_offline_for() -> Weight;
    fn force_offline() -> Weight;
    fn force_offline_for() -> Weight;
    fn heartbeat() -> Weight;
}

/// Weights for pallet_offchain_computing_workers using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForWorkers (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForWorkers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers AccountOwningWorkers (r:0 w:1)
    /// Proof: OffchainComputingWorkers AccountOwningWorkers (max_values: None, max_size: Some(96), added: 2571, mode: MaxEncodedLen)
    fn register_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `6`
        //   Estimated: `8711`
        // Minimum execution time: 52_000_000 picoseconds.
        Weight::from_parts(53_000_000, 8711)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputing WorkerServingPools (r:1 w:0)
    /// Proof: OffchainComputing WorkerServingPools (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForWorkers (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForWorkers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers AccountOwningWorkers (r:0 w:1)
    /// Proof: OffchainComputingWorkers AccountOwningWorkers (max_values: None, max_size: Some(96), added: 2571, mode: MaxEncodedLen)
    fn deregister_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `371`
        //   Estimated: `12244`
        // Minimum execution time: 57_000_000 picoseconds.
        Weight::from_parts(58_000_000, 12244)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:0)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn transfer_to_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `329`
        //   Estimated: `7222`
        // Minimum execution time: 32_000_000 picoseconds.
        Weight::from_parts(33_000_000, 7222)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:0)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn withdraw_from_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `329`
        //   Estimated: `7222`
        // Minimum execution time: 32_000_000 picoseconds.
        Weight::from_parts(33_000_000, 7222)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CurrentFlipFlopStartedAt (r:1 w:0)
    /// Proof: OffchainComputingWorkers CurrentFlipFlopStartedAt (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
    /// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipOrFlop (r:1 w:0)
    /// Proof: OffchainComputingWorkers FlipOrFlop (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    fn online() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `511`
        //   Estimated: `24359`
        // Minimum execution time: 28_000_000 picoseconds.
        Weight::from_parts(29_000_000, 24359)
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    fn refresh_attestation() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `383`
        //   Estimated: `5122`
        // Minimum execution time: 15_000_000 picoseconds.
        Weight::from_parts(15_000_000, 5122)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputing WorkerAssignedTasksCounter (r:1 w:0)
    /// Proof: OffchainComputing WorkerAssignedTasksCounter (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    fn request_offline() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `556`
        //   Estimated: `19253`
        // Minimum execution time: 25_000_000 picoseconds.
        Weight::from_parts(26_000_000, 19253)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputing WorkerAssignedTasksCounter (r:1 w:0)
    /// Proof: OffchainComputing WorkerAssignedTasksCounter (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    fn request_offline_for() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `556`
        //   Estimated: `19253`
        // Minimum execution time: 25_000_000 picoseconds.
        Weight::from_parts(26_000_000, 19253)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    fn force_offline() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `514`
        //   Estimated: `15736`
        // Minimum execution time: 23_000_000 picoseconds.
        Weight::from_parts(23_000_000, 15736)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    fn force_offline_for() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `514`
        //   Estimated: `15736`
        // Minimum execution time: 23_000_000 picoseconds.
        Weight::from_parts(24_000_000, 15736)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:0)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CurrentFlipFlopStartedAt (r:1 w:0)
    /// Proof: OffchainComputingWorkers CurrentFlipFlopStartedAt (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
    /// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipOrFlop (r:1 w:0)
    /// Proof: OffchainComputingWorkers FlipOrFlop (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlipSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    fn heartbeat() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `599`
        //   Estimated: `25781`
        // Minimum execution time: 28_000_000 picoseconds.
        Weight::from_parts(30_000_000, 25781)
            .saturating_add(T::DbWeight::get().reads(10_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForWorkers (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForWorkers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers AccountOwningWorkers (r:0 w:1)
    /// Proof: OffchainComputingWorkers AccountOwningWorkers (max_values: None, max_size: Some(96), added: 2571, mode: MaxEncodedLen)
    fn register_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `6`
        //   Estimated: `8711`
        // Minimum execution time: 52_000_000 picoseconds.
        Weight::from_parts(53_000_000, 8711)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(4_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputing WorkerServingPools (r:1 w:0)
    /// Proof: OffchainComputing WorkerServingPools (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForWorkers (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForWorkers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers AccountOwningWorkers (r:0 w:1)
    /// Proof: OffchainComputingWorkers AccountOwningWorkers (max_values: None, max_size: Some(96), added: 2571, mode: MaxEncodedLen)
    fn deregister_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `371`
        //   Estimated: `12244`
        // Minimum execution time: 57_000_000 picoseconds.
        Weight::from_parts(58_000_000, 12244)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(4_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:0)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn transfer_to_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `329`
        //   Estimated: `7222`
        // Minimum execution time: 32_000_000 picoseconds.
        Weight::from_parts(33_000_000, 7222)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:0)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn withdraw_from_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `329`
        //   Estimated: `7222`
        // Minimum execution time: 32_000_000 picoseconds.
        Weight::from_parts(33_000_000, 7222)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CurrentFlipFlopStartedAt (r:1 w:0)
    /// Proof: OffchainComputingWorkers CurrentFlipFlopStartedAt (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
    /// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipOrFlop (r:1 w:0)
    /// Proof: OffchainComputingWorkers FlipOrFlop (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    fn online() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `511`
        //   Estimated: `24359`
        // Minimum execution time: 28_000_000 picoseconds.
        Weight::from_parts(29_000_000, 24359)
            .saturating_add(RocksDbWeight::get().reads(9_u64))
            .saturating_add(RocksDbWeight::get().writes(4_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    fn refresh_attestation() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `383`
        //   Estimated: `5122`
        // Minimum execution time: 15_000_000 picoseconds.
        Weight::from_parts(15_000_000, 5122)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputing WorkerAssignedTasksCounter (r:1 w:0)
    /// Proof: OffchainComputing WorkerAssignedTasksCounter (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    fn request_offline() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `556`
        //   Estimated: `19253`
        // Minimum execution time: 25_000_000 picoseconds.
        Weight::from_parts(26_000_000, 19253)
            .saturating_add(RocksDbWeight::get().reads(6_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputing WorkerAssignedTasksCounter (r:1 w:0)
    /// Proof: OffchainComputing WorkerAssignedTasksCounter (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    fn request_offline_for() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `556`
        //   Estimated: `19253`
        // Minimum execution time: 25_000_000 picoseconds.
        Weight::from_parts(26_000_000, 19253)
            .saturating_add(RocksDbWeight::get().reads(6_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    fn force_offline() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `514`
        //   Estimated: `15736`
        // Minimum execution time: 23_000_000 picoseconds.
        Weight::from_parts(23_000_000, 15736)
            .saturating_add(RocksDbWeight::get().reads(5_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:1)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers Impls (r:1 w:1)
    /// Proof: OffchainComputingWorkers Impls (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
    fn force_offline_for() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `514`
        //   Estimated: `15736`
        // Minimum execution time: 23_000_000 picoseconds.
        Weight::from_parts(24_000_000, 15736)
            .saturating_add(RocksDbWeight::get().reads(5_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
    /// Storage: OffchainComputingWorkers Workers (r:1 w:0)
    /// Proof: OffchainComputingWorkers Workers (max_values: None, max_size: Some(164), added: 2639, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CurrentFlipFlopStartedAt (r:1 w:0)
    /// Proof: OffchainComputingWorkers CurrentFlipFlopStartedAt (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
    /// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipOrFlop (r:1 w:0)
    /// Proof: OffchainComputingWorkers FlipOrFlop (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlopSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlopSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlopSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers FlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers FlipSet (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: OffchainComputingWorkers CounterForFlipSet (r:1 w:1)
    /// Proof: OffchainComputingWorkers CounterForFlipSet (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    fn heartbeat() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `599`
        //   Estimated: `25781`
        // Minimum execution time: 28_000_000 picoseconds.
        Weight::from_parts(30_000_000, 25781)
            .saturating_add(RocksDbWeight::get().reads(10_u64))
            .saturating_add(RocksDbWeight::get().writes(4_u64))
    }
}
