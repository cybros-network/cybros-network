use std::sync::Arc;
use futures::StreamExt;
use log::info;
use subxt::OnlineClient;
use sp_core::traits::SpawnEssentialNamed;

use crate::chain::CybrosConfig;

const LOG_TARGET: &str = "chain-sync-service";

/// Error type used in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Network error")]
	Network,
}

/// Chain sync service: checks the available space for the filesystem for fiven path.
pub struct ChainSyncService {
	/// the Substrate node API client
	substrate_api: Arc<OnlineClient<CybrosConfig>>
}

impl ChainSyncService {
	/// Creates new StorageMonitorService for given client config
	pub fn try_spawn(
		substrate_api: Arc<OnlineClient<CybrosConfig>>,
		spawner: &impl SpawnEssentialNamed,
	) -> Result<(), Error> {
		let service = ChainSyncService {
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
		let mut block_sub = self.substrate_api.blocks().subscribe_finalized().await.unwrap();
		// Get each finalized block as it arrives.
		while let Some(block) = block_sub.next().await {
			let block = block.unwrap();

			// Ask for the events for this block.
			let events = block.events().await.unwrap();

			let block_hash = block.hash();

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
