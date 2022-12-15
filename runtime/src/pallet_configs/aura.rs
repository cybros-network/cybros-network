use crate::*;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::ConstU32;

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type MaxAuthorities = ConstU32<32>;
	type DisabledValidators = ();
}
