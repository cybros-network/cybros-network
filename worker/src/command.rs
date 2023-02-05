use crate::framework::cli::{WorkerCli, RunCmd, Result};
use crate::framework::service::{Configuration, TaskManager};

use std::{str::FromStr, sync::Arc};
use subxt::OnlineClient;
use crate::chain::RuntimeConfig;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[clap(flatten)]
	pub run: RunCmd,
}

impl WorkerCli for Cli {
	fn impl_name() -> String {
		"Worker prototype".into()
	}

	fn impl_version() -> String {
		env!("WORKER_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2022
	}
}

const DB_FILE_NAME: &str = "storage.redb";
const TABLE_SECRETS: redb::TableDefinition<&str, &[u8]> = redb::TableDefinition::new("secrets");

/// Parse and run command line arguments
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	let runner = cli.create_runner(&cli.run)?;
	runner.run_node_until_exit(|config| async move {
		let task_manger = init_worker(&config).await?;

		Ok(task_manger)
	})
}

async fn init_worker(config: &Configuration) -> crate::framework::service::Result<TaskManager, crate::framework::service::Error> {
	use futures::FutureExt;
	use redb::ReadableTable;
	use log::info;
	use sp_core::{
		sr25519::Pair,
		Pair as PairT,
		crypto::{SecretUri, ExposeSecret},
	};

	use runtime_primitives::types::{AccountId, Balance, BlockNumber};
	use crate::framework::service::config::PrometheusConfig;

	type WorkerInfo = pallet_computing_workers_primitives::WorkerInfo<AccountId, Balance, BlockNumber>;

	let work_path = config.base_path.as_ref().expect("Must provide a valid `--base-path`");
	let work_path = work_path.path();

	let sysinfo = sc_sysinfo::gather_sysinfo();
	sc_sysinfo::print_sysinfo(&sysinfo);

	let task_manager = init_task_manager(&config)?;
	let spawn_handle = task_manager.spawn_handle();

	// Start Prometheus service earlier so monitor can get the worker is started (but may not initiated)
	if let Some(PrometheusConfig { port, registry }) = config.prometheus_config.clone() {
		spawn_handle.spawn(
			"prometheus-endpoint",
			None,
			prometheus_endpoint::init_prometheus(port, registry).map(drop),
		);
	}

	// Load or create DB
	let db = init_db(work_path)?;

	// Read the worker identity

	// If not found, generate one.
	let read_txn = db.begin_read()?;
	if let Err(_table) = read_txn.open_table(TABLE_SECRETS) {
		let secret =
			if let Some(dev_seed) = &config.dev_key_seed {
				let suri = SecretUri::from_str(&dev_seed).unwrap();
				let pair = Pair::from_string_with_seed(
					suri.phrase.expose_secret().as_str(),
					None
				).unwrap().0;
				let keyring = Pair::from(pair);
				keyring.to_raw_vec()
			} else {
				let keyring = Pair::generate().0;

				keyring.to_raw_vec()
			};

		let write_txn = db.begin_write()?;
		{
			let mut table = write_txn.open_table(TABLE_SECRETS)?;
			table.insert("worker_identity", secret.as_slice())?;
		}
		write_txn.commit()?;
	}

	// Read the worker identity, display it
	let read_txn = db.begin_read()?;
	let table = read_txn.open_table(TABLE_SECRETS)?;
	let worker_identity_from_db = table.get("worker_identity")?.unwrap();

	let secret = schnorrkel::SecretKey::from_bytes(worker_identity_from_db.value()).unwrap();
	let keyring = Pair::from(secret);
	info!("👤 Worker identity: {}", keyring.public());

	// TODO: Read on-chain state of the worker, if not register, notice user and quit
	let substrate_url = config.substrate_rpc_url.as_str();
	let Ok(substrate_api) = OnlineClient::<RuntimeConfig>::from_url(substrate_url).await else {
		return Err(crate::framework::service::Error::Other("Can't connect to Substrate node".to_owned()));
	};
	let substrate_api = Arc::new(substrate_api);
	info!("Connected to: {}", substrate_url);

	crate::services::ChainSyncService::try_spawn(
		keyring,
		substrate_api,
		&task_manager.spawn_essential_handle(),
	).unwrap();

	Ok(task_manager)
}

fn init_task_manager(config: &Configuration) -> crate::framework::service::Result<TaskManager, crate::framework::service::Error> {
	let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
	TaskManager::new(config.tokio_handle.clone(), registry).map_err(|e| e.into())
}

fn init_db(work_path: &std::path::Path) -> std::result::Result<redb::Database, crate::framework::service::Error> {
	let db_path = work_path.join(DB_FILE_NAME);
	redb::Database::create(db_path).map_err(|e| e.into())
}
