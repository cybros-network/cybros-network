use std::sync::Arc;
use futures::StreamExt;
use log::{error, info, debug, warn};
use scale_codec::Decode;
use sp_core::{
	traits::SpawnEssentialNamed,
	sr25519::Pair,
	Pair as PairT,
};
use subxt::{
	dynamic::Value,
	config::substrate::H256,
	OnlineClient,
};

use crate::chain::RuntimeConfig;

use runtime_primitives::types::{AccountId, Balance, BlockNumber};
type WorkerInfo = pallet_computing_workers_primitives::WorkerInfo<AccountId, Balance, BlockNumber>;

const LOG_TARGET: &str = "chain-sync-service";

/// Error type used in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Can't fetch on-chain entry")]
	FetchFailed(subxt::Error),
	#[error("Can't decode SCALE data")]
	ScaleDecodeFailed(scale_codec::Error),
	#[error("Can't found on-chain entry")]
	EntryNotFound,
}

/// Chain sync service: checks the available space for the filesystem for fiven path.
pub struct ChainSyncService {
	/// The keyring of the worker
	keyring: Pair,
	/// The Substrate node API client
	substrate_api: Arc<OnlineClient<RuntimeConfig>>,
}

impl ChainSyncService {
	/// Creates new StorageMonitorService for given client config
	pub fn try_spawn(
		keyring: Pair,
		substrate_api: Arc<OnlineClient<RuntimeConfig>>,
		spawner: &impl SpawnEssentialNamed,
	) -> Result<(), Error> {
		let service = ChainSyncService {
			keyring,
			substrate_api,
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
			let worker_info =
				match self.fetch_worker_info(Some(block_hash)).await {
					Ok(w) => w,
					Err(Error::EntryNotFound) => {
						warn!("[Finalized #{}] Couldn't found the worker info, you need to register it first", block_number);
						continue;
					},
					Err(Error::FetchFailed(e)) => {
						error!("[Finalized #{}] Couldn't fetch the worker info: {:?}", block_number, e);
						break;
					},
					Err(Error::ScaleDecodeFailed(e)) => {
						error!("[Finalized #{}] Couldn't decode the worker info: {:?}", block_number, e);
						break;
					},
				};
			let status = worker_info.status;
			debug!("[Finalized #{}] Worker status: {}", block_number, status);

			// TODO: make online if offline or registered
			// Perhaps we need a child-task-manager

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

	async fn fetch_worker_info(&self, block_hash: Option<H256>) -> Result<WorkerInfo, Error> {
		let storage_address = subxt::dynamic::storage(
			"ComputingWorkers",
			"Workers",
			vec![
				// Something that encodes to an AccountId32 is what we need for the map key here:
				Value::from_bytes(&self.keyring.public()),
			],
		);

		let raw_worker_info =
			match self.substrate_api
				.storage()
				.at(block_hash)
				.await.unwrap()
				.fetch(&storage_address)
				.await {
				Ok(r) => r,
				Err(e) => {
					return Err(Error::FetchFailed(e))
				}
			};
		let Some(raw_worker_info) = raw_worker_info else {
			return Err(Error::EntryNotFound)
		};
		match WorkerInfo::decode::<&[u8]>(&mut raw_worker_info.encoded()) {
			Ok(decoded) => Ok(decoded),
			Err(e) => {
				Err(Error::ScaleDecodeFailed(e))
			}
		}
	}
}
