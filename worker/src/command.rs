use crate::cli::{WorkerCli, RunCmd, Result};
use crate::service::new_worker;

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
	let cli = Cli::from_args();

	let runner = cli.create_runner(&cli.run)?;
	runner.run_node_until_exit(|config| async move {
		let (keystore, mut task_manager) = new_worker(&config)?;

		crate::service::spawn_tasks(
			crate::service::SpawnTasksParams {
				config,
				task_manager: &mut task_manager,
				keystore: keystore.clone()
			}
		)?;

		Ok(task_manager)
	})
}
