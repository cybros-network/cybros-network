use std::sync::Arc;
use futures::StreamExt;
use log::{error, info};
use scale_codec::Decode;
use sp_core::{
	traits::SpawnEssentialNamed,
	sr25519::{Pair, Public, Signature},
	Pair as PairT,
	crypto::{SecretUri, ExposeSecret},
};
use subxt::{
	dynamic::Value,
	tx::PairSigner,
	OnlineClient,
};

use crate::chain::CybrosConfig;

use runtime_primitives::types::{AccountId, Balance, BlockNumber};
type WorkerInfo = pallet_computing_workers_primitives::WorkerInfo<AccountId, Balance, BlockNumber>;

const LOG_TARGET: &str = "chain-sync-service";

/// Error type used in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Network error")]
	Network,
}

/// Chain sync service: checks the available space for the filesystem for fiven path.
pub struct ChainSyncService {
	/// The keyring of the worker
	keyring: Pair,
	/// The Substrate node API client
	substrate_api: Arc<OnlineClient<CybrosConfig>>
}

impl ChainSyncService {
	/// Creates new StorageMonitorService for given client config
	pub fn try_spawn(
		keyring: Pair,
		substrate_api: Arc<OnlineClient<CybrosConfig>>,
		spawner: &impl SpawnEssentialNamed,
	) -> Result<(), Error> {
		let service = ChainSyncService {
			keyring,
			substrate_api
		};

		spawner.spawn_essential(
			"chain-sync",
			None,
			Box::pin(service.run()),
		);

		Ok(())
	}

	/// Main monitoring loop, intended to be spawned as essential task. Quits if free space drop
	/// below threshold.
	async fn run(self) {
		let storage_address = subxt::dynamic::storage(
			"ComputingWorkers",
			"Workers",
			vec![
				// Something that encodes to an AccountId32 is what we need for the map key here:
				Value::from_bytes(&self.keyring.public()),
			],
		);

		let mut block_sub = self.substrate_api.blocks().subscribe_finalized().await.unwrap();
		// Get each finalized block as it arrives.
		while let Some(block) = block_sub.next().await {
			let block = match block {
				Ok(b) => b,
				Err(e) => {
					error!(target: LOG_TARGET, "Couldn't fetch new block {:?}", e);
					continue;
				}
			};
			let block_number = block.number();
			let block_hash = block.hash();
			let worker_info: WorkerInfo = {
				let raw_worker_info =
					match self.substrate_api
						.storage()
						.at(Some(block_hash))
						.await.unwrap()
						.fetch(&storage_address)
						.await {
						Ok(r) => r,
						Err(e) => {
							error!("[Finalized #{}] Couldn't fetch the worker info: {:?}", block_number, e);
							break;
						}
					};
				let Some(raw_worker_info) = raw_worker_info else {
					error!("[Finalized #{}] Couldn't found the worker info, you need to register it first", block_number);
					continue;
				};
				match WorkerInfo::decode::<&[u8]>(&mut raw_worker_info.encoded()) {
					Ok(decoded) => decoded,
					Err(e) => {
						error!("[Finalized #{}] Couldn't decode the worker info: {:?}", block_number, e);
						break;
					}
				}
			};
			let status = worker_info.status;
			info!("[Finalized #{}] Worker status: {}", block_number, worker_info.status);

			// TODO: make online if offline or registered

			// Ask for the events for this block.
			let events = block.events().await.unwrap();
			// We can dynamically decode events:
			info!("  Dynamic event details: {block_hash:?}:");
			for event in events.iter() {
				let event = event.unwrap();
				let pallet = event.pallet_name();
				let variant = event.variant_name();
				info!(
					"    {pallet}::{variant}"
				);
			}
		}
	}
}
