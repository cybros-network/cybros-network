use crate::*;
use frame_support::traits::{ConstU32, ConstU64};

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxAuthorities = ConstU32<32>;
	type MaxSetIdSessionEntries = ConstU64<0>;
	type KeyOwnerProof = sp_core::Void;
	type EquivocationReportSystem = ();
}
