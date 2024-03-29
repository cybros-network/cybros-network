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

//! Autogenerated weights for pallet_offchain_computing_infra
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-29, STEPS: `50`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! CPU: `<UNKNOWN>`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
//    ./target/production/cybros-node
//    benchmark
//    pallet
//    --pallet=pallet_offchain_computing_infra
//    --extrinsic=*
//    --chain=dev
//    --steps=50
//    --repeat=50
//    --no-storage-info
//    --no-median-slopes
//    --no-min-squares
//    --wasm-execution=compiled
//    --heap-pages=4096
//    --output=./pallets/offchain_computing_infra/src/weights.rs
//    --template=./pallet-weight-template.hbs
//    --header
//    ./AGPL3-HEADER

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_offchain_computing_infra.
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

/// Weights for pallet_offchain_computing_infra using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::Impls` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Impls` (`max_values`: None, `max_size`: Some(77), added: 2552, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForWorkers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForWorkers` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::AccountOwningWorkers` (r:0 w:1)
    /// Proof: `OffchainComputingInfra::AccountOwningWorkers` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
    fn register_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `286`
        //   Estimated: `3655`
        // Minimum execution time: 44_000_000 picoseconds.
        Weight::from_parts(46_000_000, 3655)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::Impls` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Impls` (`max_values`: None, `max_size`: Some(77), added: 2552, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForWorkers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForWorkers` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::AccountOwningWorkers` (r:0 w:1)
    /// Proof: `OffchainComputingInfra::AccountOwningWorkers` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
    fn deregister_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `652`
        //   Estimated: `3655`
        // Minimum execution time: 44_000_000 picoseconds.
        Weight::from_parts(48_000_000, 3655)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn transfer_to_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `466`
        //   Estimated: `3655`
        // Minimum execution time: 20_000_000 picoseconds.
        Weight::from_parts(21_000_000, 3655)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn withdraw_from_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `466`
        //   Estimated: `3655`
        // Minimum execution time: 20_000_000 picoseconds.
        Weight::from_parts(22_000_000, 3655)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:0)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CurrentFlipFlopStartedAt` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::CurrentFlipFlopStartedAt` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `RandomnessCollectiveFlip::RandomMaterial` (r:1 w:0)
    /// Proof: `RandomnessCollectiveFlip::RandomMaterial` (`max_values`: Some(1), `max_size`: Some(2594), added: 3089, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipOrFlop` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::FlipOrFlop` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    fn online() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `543`
        //   Estimated: `4079`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(18_000_000, 4079)
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn refresh_attestation() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `510`
        //   Estimated: `3655`
        // Minimum execution time: 8_000_000 picoseconds.
        Weight::from_parts(9_000_000, 3655)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingPool::CounterForWorkerAssignedJobs` (r:1 w:0)
    /// Proof: `OffchainComputingPool::CounterForWorkerAssignedJobs` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn request_offline() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `573`
        //   Estimated: `3655`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(17_000_000, 3655)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingPool::CounterForWorkerAssignedJobs` (r:1 w:0)
    /// Proof: `OffchainComputingPool::CounterForWorkerAssignedJobs` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn request_offline_for() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `573`
        //   Estimated: `3655`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(17_000_000, 3655)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingPool::CounterForWorkerAssignedJobs` (r:1 w:0)
    /// Proof: `OffchainComputingPool::CounterForWorkerAssignedJobs` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn force_offline() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `573`
        //   Estimated: `3655`
        // Minimum execution time: 15_000_000 picoseconds.
        Weight::from_parts(16_000_000, 3655)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingPool::CounterForWorkerAssignedJobs` (r:1 w:0)
    /// Proof: `OffchainComputingPool::CounterForWorkerAssignedJobs` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn force_offline_for() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `573`
        //   Estimated: `3655`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(17_000_000, 3655)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:0)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CurrentFlipFlopStartedAt` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::CurrentFlipFlopStartedAt` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `RandomnessCollectiveFlip::RandomMaterial` (r:1 w:0)
    /// Proof: `RandomnessCollectiveFlip::RandomMaterial` (`max_values`: Some(1), `max_size`: Some(2594), added: 3089, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipOrFlop` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::FlipOrFlop` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlipSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    fn heartbeat() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `755`
        //   Estimated: `4079`
        // Minimum execution time: 21_000_000 picoseconds.
        Weight::from_parts(23_000_000, 4079)
            .saturating_add(T::DbWeight::get().reads(11_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::Impls` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Impls` (`max_values`: None, `max_size`: Some(77), added: 2552, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForWorkers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForWorkers` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::AccountOwningWorkers` (r:0 w:1)
    /// Proof: `OffchainComputingInfra::AccountOwningWorkers` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
    fn register_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `286`
        //   Estimated: `3655`
        // Minimum execution time: 44_000_000 picoseconds.
        Weight::from_parts(46_000_000, 3655)
            .saturating_add(RocksDbWeight::get().reads(5_u64))
            .saturating_add(RocksDbWeight::get().writes(6_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::Impls` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Impls` (`max_values`: None, `max_size`: Some(77), added: 2552, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForWorkers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForWorkers` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::AccountOwningWorkers` (r:0 w:1)
    /// Proof: `OffchainComputingInfra::AccountOwningWorkers` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
    fn deregister_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `652`
        //   Estimated: `3655`
        // Minimum execution time: 44_000_000 picoseconds.
        Weight::from_parts(48_000_000, 3655)
            .saturating_add(RocksDbWeight::get().reads(5_u64))
            .saturating_add(RocksDbWeight::get().writes(6_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn transfer_to_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `466`
        //   Estimated: `3655`
        // Minimum execution time: 20_000_000 picoseconds.
        Weight::from_parts(21_000_000, 3655)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn withdraw_from_worker() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `466`
        //   Estimated: `3655`
        // Minimum execution time: 20_000_000 picoseconds.
        Weight::from_parts(22_000_000, 3655)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:0)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CurrentFlipFlopStartedAt` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::CurrentFlipFlopStartedAt` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `RandomnessCollectiveFlip::RandomMaterial` (r:1 w:0)
    /// Proof: `RandomnessCollectiveFlip::RandomMaterial` (`max_values`: Some(1), `max_size`: Some(2594), added: 3089, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipOrFlop` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::FlipOrFlop` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    fn online() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `543`
        //   Estimated: `4079`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(18_000_000, 4079)
            .saturating_add(RocksDbWeight::get().reads(9_u64))
            .saturating_add(RocksDbWeight::get().writes(4_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn refresh_attestation() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `510`
        //   Estimated: `3655`
        // Minimum execution time: 8_000_000 picoseconds.
        Weight::from_parts(9_000_000, 3655)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingPool::CounterForWorkerAssignedJobs` (r:1 w:0)
    /// Proof: `OffchainComputingPool::CounterForWorkerAssignedJobs` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn request_offline() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `573`
        //   Estimated: `3655`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(17_000_000, 3655)
            .saturating_add(RocksDbWeight::get().reads(6_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingPool::CounterForWorkerAssignedJobs` (r:1 w:0)
    /// Proof: `OffchainComputingPool::CounterForWorkerAssignedJobs` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn request_offline_for() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `573`
        //   Estimated: `3655`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(17_000_000, 3655)
            .saturating_add(RocksDbWeight::get().reads(6_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingPool::CounterForWorkerAssignedJobs` (r:1 w:0)
    /// Proof: `OffchainComputingPool::CounterForWorkerAssignedJobs` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn force_offline() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `573`
        //   Estimated: `3655`
        // Minimum execution time: 15_000_000 picoseconds.
        Weight::from_parts(16_000_000, 3655)
            .saturating_add(RocksDbWeight::get().reads(6_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingPool::CounterForWorkerAssignedJobs` (r:1 w:0)
    /// Proof: `OffchainComputingPool::CounterForWorkerAssignedJobs` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn force_offline_for() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `573`
        //   Estimated: `3655`
        // Minimum execution time: 16_000_000 picoseconds.
        Weight::from_parts(17_000_000, 3655)
            .saturating_add(RocksDbWeight::get().reads(6_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
    /// Storage: `OffchainComputingInfra::Workers` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::Workers` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:0)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::ImplBuilds` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::ImplBuilds` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CurrentFlipFlopStartedAt` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::CurrentFlipFlopStartedAt` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `RandomnessCollectiveFlip::RandomMaterial` (r:1 w:0)
    /// Proof: `RandomnessCollectiveFlip::RandomMaterial` (`max_values`: Some(1), `max_size`: Some(2594), added: 3089, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipOrFlop` (r:1 w:0)
    /// Proof: `OffchainComputingInfra::FlipOrFlop` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlopSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlopSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlopSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::FlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::FlipSet` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `OffchainComputingInfra::CounterForFlipSet` (r:1 w:1)
    /// Proof: `OffchainComputingInfra::CounterForFlipSet` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    fn heartbeat() -> Weight {
        // Proof Size summary in bytes:
        //   Measured:  `755`
        //   Estimated: `4079`
        // Minimum execution time: 21_000_000 picoseconds.
        Weight::from_parts(23_000_000, 4079)
            .saturating_add(RocksDbWeight::get().reads(11_u64))
            .saturating_add(RocksDbWeight::get().writes(5_u64))
    }
}
