use crate::*;
use frame_support::{
	parameter_types,
	traits::AsEnsureOriginWithArg,
};
use frame_system::EnsureSigned;
use sp_runtime::traits::Verify;
use pallet_nfts::{PalletFeature, PalletFeatures};

parameter_types! {
	pub const CollectionDeposit: Balance = 100 * UNITS;
	pub const ItemDeposit: Balance = 1 * UNITS;
	pub const MetadataDepositBase: Balance = 10 * UNITS;
	pub const MetadataDepositPerByte: Balance = 1 * UNITS;
	pub const KeyLimit: u32 = 32;
	pub const ValueLimit: u32 = 256;
	pub const StringLimit: u32 = 50;
	pub const ApprovalsLimit: u32 = 20;
	pub const ItemAttributesApprovalsLimit: u32 = 20;
	pub const MaxTips: u32 = 10;
	pub const MaxDeadlineDuration: BlockNumber = 12 * 30 * DAYS;
	pub const MaxAttributesPerCall: u32 = 10;
	pub Features: PalletFeatures = PalletFeatures::from_disabled(
		PalletFeature::Approvals | PalletFeature::Trading | PalletFeature::Swaps
	);
}

impl pallet_nfts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = ();
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type ApprovalsLimit = ApprovalsLimit;
	type ItemAttributesApprovalsLimit = ItemAttributesApprovalsLimit;
	type MaxTips = MaxTips;
	type MaxDeadlineDuration = MaxDeadlineDuration;
	type MaxAttributesPerCall = MaxAttributesPerCall;
	type Features = Features;
	type OffchainSignature = Signature;
	type OffchainPublic = <Signature as Verify>::Signer;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type WeightInfo = pallet_nfts::weights::SubstrateWeight<Runtime>;
}
