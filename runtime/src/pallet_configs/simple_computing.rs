use crate::*;
use frame_support::parameter_types;

parameter_types! {
	pub const MaxJobPayloadLen: u32 = 128 * 1000;
	pub const SlashingCardinal: Balance = UNITS;
}

impl pallet_simple_computing::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WorkerManageable = ComputingWorkers;
	type MaxJobPayloadLen = MaxJobPayloadLen;
	type SlashingCardinal = SlashingCardinal;
}
