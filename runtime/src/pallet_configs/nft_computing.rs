use crate::*;
use frame_support::parameter_types;

parameter_types! {
	pub const MetadataLimit: u32 = 256;
	pub const OutputLimit: u32 = 256;
}

impl pallet_nft_computing::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WorkerManageable = ComputingWorkers;
	type Currency = Balances;
	type NftCollectionId = u32;
	type NftItemId = u32;
	type Nfts = Nfts;
	type MetadataLimit = MetadataLimit;
	type OutputLimit = OutputLimit;
}
