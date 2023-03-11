use crate::*;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use frame_support::traits::ConstU32;

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type MaxAuthorities = ConstU32<32>;
	type DisabledValidators = ();
}
