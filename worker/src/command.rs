use crate::cli::{WorkerCli, RunCmd, Result};

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

/// Parse and run command line arguments
pub fn run() -> Result<()> {
	use futures::FutureExt;
	use crate::service::TaskManager;
	use crate::service::config::PrometheusConfig;
	use crate::service::Error;

	let cli = Cli::from_args();

	let runner = cli.create_runner(&cli.run)?;
	runner.run_node_until_exit(|config| async move {
		let sysinfo = sc_sysinfo::gather_sysinfo();
		sc_sysinfo::print_sysinfo(&sysinfo);

		let task_manager = {
			let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
			TaskManager::new(config.tokio_handle.clone(), registry)
		}.map_err(|e| <prometheus_endpoint::PrometheusError as Into<Error>>::into(e))?;

		// TODO: Initialize DB



		// TODO: generate keys (if needed) and show public
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

		Ok(task_manager)
	})
}
