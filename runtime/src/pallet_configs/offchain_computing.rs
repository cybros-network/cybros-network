use crate::*;
use frame_system::EnsureSigned;
use frame_support::traits::{
	ConstU32, ConstU64, ConstU128,
};

impl pallet_offchain_computing::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OffchainWorkerManageable = OffchainComputingWorkers;
	type Currency = Balances;
	type UnixTime = Timestamp;
	type PoolId = u32;
	type JobId = u32;
	type PolicyId = u32;
	type CreatePoolOrigin = EnsureSigned<Self::AccountId>;
	type CreatePoolDeposit = ConstU128<{ 1 * UNITS }>;
	type DepositPerJob = ConstU128<{ 1 * UNITS }>;
	type MetadataDepositBase = ConstU128<{ 1 * CENTS }>;
	type DepositPerByte = ConstU128<{ 1 * CENTS }>;
	type MaxAssignedJobsPerWorker = ConstU32<8>;
	type MaxSubscribedPoolsPerWorker = ConstU32<8>;
	type MaxPoliciesPerPool = ConstU32<8>;
	type MaxJobsPerPool = ConstU32<1000>;
	type MaxWorkersPerPool = ConstU32<100>;
	type MinJobExpiresIn = ConstU64<600>; // ~ 10 min
	type MaxJobExpiresIn = ConstU64<86400>; // ~ 1 day
	type DefaultJobExpiresIn = ConstU64<3600>; // ~ 1 hour
	type PoolMetadataLimit = ConstU32<2048>; // 2KiB
	type InputLimit = ConstU32<2048>; // 2KiB
	type OutputLimit = ConstU32<2048>; // 2KiB
	type ProofLimit = ConstU32<2048>; // 2KiB
}
