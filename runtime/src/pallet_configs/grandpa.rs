use crate::*;
use frame_support::traits::KeyOwnerProofSystem;
use pallet_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::KeyTypeId, ConstU32, ConstU64};

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
	type KeyOwnerIdentification =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::IdentificationTuple;
	type KeyOwnerProofSystem = ();
	type HandleEquivocation = ();
	type WeightInfo = ();
	type MaxAuthorities = ConstU32<32>;
	type MaxSetIdSessionEntries = ConstU64<0>;
}
