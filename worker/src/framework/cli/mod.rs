#![allow(dead_code)]

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_imports)]

use clap::{CommandFactory, FromArgMatches, Parser};
use crate::framework::service::Configuration;

mod config;
mod params;
mod error;
mod runner;
mod commands;

pub mod arg_enums;

pub use arg_enums::*;
pub use clap;
pub use config::*;
pub use error::*;
pub use params::*;
pub use runner::*;
pub use commands::*;

pub use crate::framework::tracing::logging::LoggerBuilder;

/// Worker CLI
///
/// This trait needs to be implemented on the root CLI struct of the application. It will provide
/// the implementation `name`, `version`, `executable name`, `description`, `author`, `support_url`,
/// `copyright start year` and most importantly: how to load the chain spec.
pub trait WorkerCli: Sized {
	/// Implementation name.
	fn impl_name() -> String;

	/// Implementation version.
	///
	/// By default this will look like this:
	///
	/// `2.0.0-b950f731c`
	///
	/// Where the hash is the short commit hash of the commit of in the Git repository.
	fn impl_version() -> String;

	/// Executable file name.
	///
	/// Extracts the file name from `std::env::current_exe()`.
	/// Resorts to the env var `CARGO_PKG_NAME` in case of Error.
	fn executable_name() -> String {
		std::env::current_exe()
			.ok()
			.and_then(|e| e.file_name().map(|s| s.to_os_string()))
			.and_then(|w| w.into_string().ok())
			.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_owned() )
	}

	/// Executable file description.
	fn description() -> String;

	/// Executable file author.
	fn author() -> String;

	/// Support URL.
	fn support_url() -> String;

	/// Copyright starting year (x-current year)
	fn copyright_start_year() -> i32;

	/// Helper function used to parse the command line arguments. This is the equivalent of
	/// [`clap::Parser::parse()`].
	///
	/// To allow running the node without subcommand, it also sets a few more settings:
	/// [`clap::Command::propagate_version`], [`clap::Command::args_conflicts_with_subcommands`],
	/// [`clap::Command::subcommand_negates_reqs`].
	///
	/// Creates `Self` from the command line arguments. Print the
	/// error message and quit the program in case of failure.
	fn from_args() -> Self
		where
			Self: Parser + Sized,
	{
		<Self as WorkerCli>::from_iter(&mut std::env::args_os())
	}

	/// Helper function used to parse the command line arguments. This is the equivalent of
	/// [`clap::Parser::parse_from`].
	///
	/// To allow running the node without subcommand, it also sets a few more settings:
	/// [`clap::Command::propagate_version`], [`clap::Command::args_conflicts_with_subcommands`],
	/// [`clap::Command::subcommand_negates_reqs`].
	///
	/// Creates `Self` from any iterator over arguments.
	/// Print the error message and quit the program in case of failure.
	fn from_iter<I>(iter: I) -> Self
		where
			Self: Parser + Sized,
			I: IntoIterator,
			I::Item: Into<std::ffi::OsString> + Clone,
	{
		let app = <Self as CommandFactory>::command();

		let mut full_version = Self::impl_version();
		full_version.push('\n');

		let name = Self::executable_name();
		let author = Self::author();
		let about = Self::description();
		let app = app
			.name(name)
			.author(author)
			.about(about)
			.version(full_version)
			.propagate_version(true)
			.args_conflicts_with_subcommands(true)
			.subcommand_negates_reqs(true);

		let matches = app.try_get_matches_from(iter).unwrap_or_else(|e| e.exit());

		<Self as FromArgMatches>::from_arg_matches(&matches).unwrap_or_else(|e| e.exit())
	}

	/// Helper function used to parse the command line arguments. This is the equivalent of
	/// [`clap::Parser::try_parse_from`]
	///
	/// To allow running the node without subcommand, it also sets a few more settings:
	/// [`clap::Command::propagate_version`], [`clap::Command::args_conflicts_with_subcommands`],
	/// [`clap::Command::subcommand_negates_reqs`].
	///
	/// Creates `Self` from any iterator over arguments.
	/// Print the error message and quit the program in case of failure.
	///
	/// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
	/// used. It will return a [`clap::Error`], where the [`clap::Error::kind`] is a
	/// [`clap::error::ErrorKind::DisplayHelp`] or [`clap::error::ErrorKind::DisplayVersion`]
	/// respectively. You must call [`clap::Error::exit`] or perform a [`std::process::exit`].
	fn try_from_iter<I>(iter: I) -> clap::error::Result<Self>
		where
			Self: Parser + Sized,
			I: IntoIterator,
			I::Item: Into<std::ffi::OsString> + Clone,
	{
		let app = <Self as CommandFactory>::command();

		let mut full_version = Self::impl_version();
		full_version.push('\n');

		let name = Self::executable_name();
		let author = Self::author();
		let about = Self::description();
		let app = app.name(name).author(author).about(about).version(full_version);

		let matches = app.try_get_matches_from(iter)?;

		<Self as FromArgMatches>::from_arg_matches(&matches)
	}

	/// Only create a Configuration for the command provided in argument
	fn create_configuration<T: CliConfiguration<DVC>, DVC: DefaultConfigurationValues>(
		&self,
		command: &T,
		tokio_handle: tokio::runtime::Handle,
	) -> error::Result<Configuration> {
		command.create_configuration(self, tokio_handle)
	}

	/// Create a runner for the command provided in argument. This will create a Configuration and
	/// a tokio runtime
	fn create_runner<T: CliConfiguration<DVC>, DVC: DefaultConfigurationValues>(
		&self,
		command: &T,
	) -> error::Result<Runner<Self>> {
		let tokio_runtime = build_runtime()?;
		let config = command.create_configuration(self, tokio_runtime.handle().clone())?;

		command.init(&Self::support_url(), &Self::impl_version(), |_, _| {}, &config)?;
		Runner::new(config, tokio_runtime)
	}

	/// Create a runner for the command provided in argument. The `logger_hook` can be used to setup
	/// a custom profiler or update the logger configuration before it is initialized.
	///
	/// Example:
	/// ```
	/// use crate::framework::tracing::{SpanDatum, TraceEvent};
	/// struct TestProfiler;
	///
	/// impl crate::framework::tracing::TraceHandler for TestProfiler {
	///  	fn handle_span(&self, sd: &SpanDatum) {}
	/// 		fn handle_event(&self, _event: &TraceEvent) {}
	/// };
	///
	/// fn logger_hook() -> impl FnOnce(&mut crate::framework::cli::LoggerBuilder, &sc_service::Configuration) -> () {
	/// 	|logger_builder, config| {
	/// 			logger_builder.with_custom_profiling(Box::new(TestProfiler{}));
	/// 	}
	/// }
	/// ```
	fn create_runner_with_logger_hook<T: CliConfiguration, F>(
		&self,
		command: &T,
		logger_hook: F,
	) -> error::Result<Runner<Self>>
		where
			F: FnOnce(&mut LoggerBuilder, &Configuration),
	{
		let tokio_runtime = build_runtime()?;
		let config = command.create_configuration(self, tokio_runtime.handle().clone())?;

		command.init(&Self::support_url(), &Self::impl_version(), logger_hook, &config)?;
		Runner::new(config, tokio_runtime)
	}
}
