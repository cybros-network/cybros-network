use crate::*;
use frame_support::parameter_types;

parameter_types! {
	pub const JobDepositBase: Balance = deposit(1, MaxJobOutputLen::get());
	pub const JobInputDepositPerByte: Balance = deposit(0, 1);
	pub const MinJobRunningDurationLen: u32 = 20;
	pub const MaxJobCommandLen: u32 = 32;
	pub const MaxJobInputLen: u32 = 2 * 1024; // 2 KiB
	pub const MaxJobOutputLen: u32 = 2 * 1024; // 2 KiB
}

impl pallet_simple_computing::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WorkerManageable = ComputingWorkers;
	type JobId = u32;
	type JobDepositBase = JobDepositBase;
	type JobInputDepositPerByte = JobInputDepositPerByte;
	type MinJobRunningDurationLen = MinJobRunningDurationLen;
	type MaxJobCommandLen = MaxJobCommandLen;
	type MaxJobInputLen = MaxJobInputLen;
	type MaxJobOutputLen = MaxJobOutputLen;
}
