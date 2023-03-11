use crate::*;
use frame_support::traits::ConstU32;

impl pallet_nft_computing::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WorkerManageable = ComputingWorkers;
	type Currency = Balances;
	type NftCollectionId = u32;
	type NftItemId = u32;
	type Nfts = Nfts;
	type MaxAcquiredItemsPerWorker = ConstU32<8>;
	type MetadataLimit = ConstU32<256>;
	type OutputLimit = ConstU32<256>;
}
