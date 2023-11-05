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

use crate as pallet_offchain_computing_infra;

use frame_support::traits::{fungible::Mutate, OnFinalize, OnInitialize};
use frame_system::EnsureSigned;
use sp_core::{ConstBool, ConstU128, ConstU16, ConstU32, ConstU64, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

pub(crate) type Balance = u128;
pub(crate) type AccountId = u64;

pub(crate) const INIT_TIMESTAMP: u64 = 30_000;
pub(crate) const BLOCK_TIME: u64 = 1000;

pub(crate) const MILLI_CENTS: Balance = 1_000_000;
pub(crate) const CENTS: Balance = 1_000 * MILLI_CENTS;
pub(crate) const DOLLARS: Balance = 100 * CENTS;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
		Balances: pallet_balances,
		Timestamp: pallet_timestamp,
		RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,
		OffchainComputingInfra: pallet_offchain_computing_infra,
	}
);

impl frame_system::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = ();
	type WeightInfo = ();
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<{ CENTS }>;
	type AccountStore = System;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type MaxHolds = ConstU32<2>;
	type MaxFreezes = ();
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

impl pallet_offchain_computing_infra::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type UnixTime = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type ImplId = u32;
	type RegisterImplOrigin = EnsureSigned<Self::AccountId>;
	type RegisterWorkerDeposit = ConstU128<{ 100 * DOLLARS }>;
	type RegisterImplDeposit = ConstU128<{ DOLLARS }>;
	type ImplMetadataDepositBase = ConstU128<{ DOLLARS }>;
	type ImplMetadataDepositPerByte = ConstU128<{ CENTS }>;
	type ImplMetadataLimit = ConstU32<50>;
	type MaxImplBuilds = ConstU32<4>;
	type HandleUnresponsivePerBlockLimit = ConstU32<3>;
	type CollectingHeartbeatsDurationInBlocks = ConstU32<6>;
	type DisallowOptOutAttestation = ConstBool<false>;
	type WeightInfo = ();
	type OffchainWorkerLifecycleHooks = ();
}

// Build genesis storage according to the mock runtime.
#[allow(unused)]
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	// Customize genesis config here
	t.into()
}

#[allow(unused)]
pub(crate) fn run_to_block(n: u64) {
	// NOTE that this function only simulates modules of interest. Depending on new pallet may
	// require adding it here.
	assert!(System::block_number() < n);

	for b in (System::block_number() + 1)..=n {
		System::set_block_number(b);
		System::on_initialize(System::block_number());
		Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
		if b != n {
			System::on_finalize(System::block_number());
		}
	}
}

#[allow(unused)]
pub(crate) fn take_events() -> Vec<RuntimeEvent> {
	let events = System::events().into_iter().map(|i| i.event).collect::<Vec<_>>();
	System::reset_events();
	events
}

#[allow(unused)]
pub(crate) fn set_balance(who: AccountId, new_free: Balance) {
	<Test as crate::Config>::Currency::set_balance(&who, new_free);
	assert_eq!(<Test as crate::Config>::Currency::free_balance(who), new_free);
}
