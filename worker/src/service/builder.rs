// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::sync::Arc;
use futures::{channel::oneshot, future::ready, FutureExt, StreamExt};
use sc_keystore::LocalKeystore;
use sp_keystore::SyncCryptoStorePtr;
use crate::service::{
	config::{Configuration, KeystoreConfig, PrometheusConfig},
	error::Error, TaskManager
};

pub fn new_worker(
	config: &Configuration
) -> Result<(Arc<LocalKeystore>, TaskManager), Error> {
	let keystore = Arc::new(match &config.keystore {
		KeystoreConfig::Path { path, password } =>
			LocalKeystore::open(path.clone(), password.clone())?,
		KeystoreConfig::InMemory => LocalKeystore::in_memory(),
	});

	let task_manager = {
		let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
		TaskManager::new(config.tokio_handle.clone(), registry)?
	};

	Ok((keystore, task_manager))
}

/// Parameters to pass into `build`.
pub struct SpawnTasksParams<'a> {
	/// The service configuration.
	pub config: Configuration,
	/// A task manager returned by `new_worker`.
	pub task_manager: &'a mut TaskManager,
	/// A shared keystore returned by `new_worker`.
	pub keystore: Arc<LocalKeystore>,
}

/// Spawn the tasks that are required to run a node.
pub fn spawn_tasks(
	params: SpawnTasksParams,
) -> Result<(), Error> {
	let SpawnTasksParams {
		mut config,
		task_manager,
		keystore,
	} = params;

	let sysinfo = sc_sysinfo::gather_sysinfo();
	sc_sysinfo::print_sysinfo(&sysinfo);

	let keystore = keystore as SyncCryptoStorePtr;


	// TODO: generate keys and show public
	// Example: info!("📦 Highest known block at #{}", chain_info.best_number);

	// TODO: Read on-chain state of the worker, if not register, register it

	// TODO: Start services, such as polling latest (finalized?) blocks, Prometheus service, etc.

	let spawn_handle = task_manager.spawn_handle();

	if let Some(PrometheusConfig { port, registry }) = config.prometheus_config.clone() {
		spawn_handle.spawn(
			"prometheus-endpoint",
			None,
			prometheus_endpoint::init_prometheus(port, registry).map(drop),
		);
	}

	Ok(())

	// let chain_info = client.usage_info().chain;
	//
	// sp_session::generate_initial_session_keys(
	// 	client.clone(),
	// 	chain_info.best_hash,
	// 	config.dev_key_seed.clone().map(|s| vec![s]).unwrap_or_default(),
	// )
	// 	.map_err(|e| Error::Application(Box::new(e)))?;
	//
	// let sysinfo = sc_sysinfo::gather_sysinfo();
	// sc_sysinfo::print_sysinfo(&sysinfo);
	//
	// let telemetry = telemetry
	// 	.map(|telemetry| {
	// 		init_telemetry(&mut config, network.clone(), client.clone(), telemetry, Some(sysinfo))
	// 	})
	// 	.transpose()?;
	//
	// info!("📦 Highest known block at #{}", chain_info.best_number);
	//
	// let spawn_handle = task_manager.spawn_handle();
	//
	// // Inform the tx pool about imported and finalized blocks.
	// spawn_handle.spawn(
	// 	"txpool-notifications",
	// 	Some("transaction-pool"),
	// 	sc_transaction_pool::notification_future(client.clone(), transaction_pool.clone()),
	// );
	//
	// spawn_handle.spawn(
	// 	"on-transaction-imported",
	// 	Some("transaction-pool"),
	// 	transaction_notifications(
	// 		transaction_pool.clone(),
	// 		tx_handler_controller,
	// 		telemetry.clone(),
	// 	),
	// );
	//
	// // Prometheus metrics.
	// let metrics_service =
	// 	if let Some(PrometheusConfig { port, registry }) = config.prometheus_config.clone() {
	// 		// Set static metrics.
	// 		let metrics = MetricsService::with_prometheus(telemetry, &registry, &config)?;
	// 		spawn_handle.spawn(
	// 			"prometheus-endpoint",
	// 			None,
	// 			prometheus_endpoint::init_prometheus(port, registry).map(drop),
	// 		);
	//
	// 		metrics
	// 	} else {
	// 		MetricsService::new(telemetry)
	// 	};
	//
	// // Periodically updated metrics and telemetry updates.
	// spawn_handle.spawn(
	// 	"telemetry-periodic-send",
	// 	None,
	// 	metrics_service.run(client.clone(), transaction_pool.clone(), network.clone()),
	// );
	//
	// let rpc_id_provider = config.rpc_id_provider.take();
	//
	// // jsonrpsee RPC
	// let gen_rpc_module = |deny_unsafe: DenyUnsafe| {
	// 	gen_rpc_module(
	// 		deny_unsafe,
	// 		task_manager.spawn_handle(),
	// 		client.clone(),
	// 		transaction_pool.clone(),
	// 		keystore.clone(),
	// 		system_rpc_tx.clone(),
	// 		&config,
	// 		backend.clone(),
	// 		&*rpc_builder,
	// 	)
	// };
	//
	// let rpc = start_rpc_servers(&config, gen_rpc_module, rpc_id_provider)?;
	// let rpc_handlers = RpcHandlers(Arc::new(gen_rpc_module(sc_rpc::DenyUnsafe::No)?.into()));
	//
	// // Spawn informant task
	// spawn_handle.spawn(
	// 	"informant",
	// 	None,
	// 	sc_informant::build(client.clone(), network, config.informant_output_format),
	// );
	//
	// task_manager.keep_alive((config.base_path, rpc));
	//
	// Ok(rpc_handlers)
}

