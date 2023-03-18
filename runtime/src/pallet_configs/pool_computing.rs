use crate::*;
use frame_support::traits::{
	AsEnsureOriginWithArg, ConstU32, ConstU64, ConstU128,
};

impl pallet_pool_computing::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WorkerManageable = ComputingWorkers;
	type Currency = Balances;
	type UnixTime = Timestamp;
	type PoolId = u32;
	type TaskId = u32;
	type PolicyId = u32;
	type CreatePoolOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<Self::AccountId>>;
	type CreatePoolDeposit = ConstU128<{ 1 * UNITS }>;
	type CreateTaskDeposit = ConstU128<{ 1 * UNITS }>;
	type CustomInfoDepositBase = ConstU128<{ 1 * CENTS }>;
	type DepositPerByte = ConstU128<{ 1 * CENTS }>;
	type MaxTakenTasksPerWorker = ConstU32<8>;
	type MaxPoliciesPerPool = ConstU32<8>;
	type MaxTasksPerPool = ConstU32<1000>;
	type MaxWorkersPerPool = ConstU32<100>;
	type MinTaskExpiresIn = ConstU64<600>; // ~ 10 min
	type MaxTaskExpiresIn = ConstU64<86400>; // ~ 1 day
	type DefaultTaskExpiresIn = ConstU64<3600>; // ~ 1 hour
	type MaxTaskScheduledTime = ConstU64<2764800>; // ~ 32 days
	type PoolCustomInfoLimit = ConstU32<2048>; // 2KiB
	type InputLimit = ConstU32<2048>; // 2KiB
	type OutputLimit = ConstU32<2048>; // 2KiB
}
