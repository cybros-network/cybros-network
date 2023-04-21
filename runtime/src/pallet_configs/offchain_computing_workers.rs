use crate::*;
use frame_system::EnsureSigned;
use frame_support::traits::{ConstBool, ConstU128, ConstU32};

impl pallet_offchain_computing_workers::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UnixTime = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type ImplId = u32;
	type RegisterImplOrigin = EnsureSigned<Self::AccountId>;
	type RegisterWorkerDeposit = ConstU128<{ 100 * UNITS }>;
	type RegisterImplDeposit = ConstU128<{ 100 * UNITS }>;
	type ImplMetadataDepositBase = ConstU128<{ 1 * UNITS }>;
	type DepositPerByte = ConstU128<{ 1 * CENTS }>;
	type ImplMetadataLimit = ConstU32<2048>; // 2KiB
	type MaxImplBuilds = ConstU32<8>;
	type HandleUnresponsivePerBlockLimit = ConstU32<100>;
	type CollectingHeartbeatsDurationInBlocks = ConstU32<300>; // 30min * 60 / 6
	type MaxWorkerUnresponsiveProtectionInBlocks = ConstU32<300>; // 30min * 60 / 6
	type DisallowOptOutAttestation = ConstBool<false>;
	type WeightInfo = pallet_offchain_computing_workers::weights::SubstrateWeight<Runtime>;
	type OffchainWorkerLifecycleHooks = OffchainComputing;
}
