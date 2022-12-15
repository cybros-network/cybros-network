use crate::*;
use frame_system::EnsureRoot;
use sp_core::{ConstBool, ConstU128, ConstU32};

impl pallet_computing_workers::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UnixTime = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type HandleUnresponsivePerBlockLimit = ConstU32<100>;
	type ReservedDeposit = ConstU128<{ 100 * UNITS }>;
	type CollectingHeartbeatsDuration = ConstU32<10>; // TODO: ConstU32<240>; // 240 block * 6 sec / 60 sec = 24 min
	type AttestationValidityDuration = ConstU32<432000>; // 10 block/min * 60 min * 24 hour * 30 days = 432000 block
	type DisallowOptOutAttestation = ConstBool<true>;
	type DisallowNonTEEAttestation = ConstBool<false>;
	type ValidateWorkerImpl = ConstBool<false>;
	type ValidateWorkerImplHash = ConstBool<false>;
	type GovernanceOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = pallet_computing_workers::weights::SubstrateWeight<Runtime>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type WorkerLifecycleHooks = SimpleComputing;
	#[cfg(feature = "runtime-benchmarks")]
	type WorkerLifecycleHooks = ();
}
