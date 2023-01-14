// This file is part of Substrate.

// Copyright (C) 2020-2022 Parity Technologies (UK) Ltd.
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

//! Configuration trait for a CLI based on substrate

use log::warn;
use std::net::SocketAddr;
use crate::cli::{
	error::Result, SharedParams, WorkerCli,
};
use crate::service::{
	config::{
		BasePath, Configuration, PrometheusConfig,
	},
	TracingReceiver,
};
use crate::tracing::logging::LoggerBuilder;

/// The recommended open file descriptor limit to be configured for the process.
const RECOMMENDED_OPEN_FILE_DESCRIPTOR_LIMIT: u64 = 10_000;

/// Default configuration values used by Substrate
///
/// These values will be used by [`CliConfiguration`] to set
/// default values for e.g. the listen port or the RPC port.
pub trait DefaultConfigurationValues {
	/// The port Substrate should listen on for http connections.
	///
	/// By default this is `8000`.
	fn rpc_http_listen_port() -> u16 {
		8000
	}

	/// The port Substrate should listen on for prometheus connections.
	///
	/// By default this is `8100`.
	fn prometheus_listen_port() -> u16 {
		8100
	}
}

impl DefaultConfigurationValues for () {}

/// A trait that allows converting an object to a Configuration
pub trait CliConfiguration<DCV: DefaultConfigurationValues = ()>: Sized {
	/// Get the SharedParams for this object
	fn shared_params(&self) -> &SharedParams;

	/// Get the base path of the configuration (if any)
	///
	/// By default this is retrieved from `SharedParams`.
	fn base_path(&self) -> Result<Option<BasePath>> {
		self.shared_params().base_path()
	}

	/// Returns `true` if the worker is for development or not
	///
	/// By default this is retrieved from `SharedParams`.
	fn is_dev(&self) -> Result<bool> {
		Ok(self.shared_params().is_dev())
	}

	/// Get the RPC HTTP address (`None` if disabled).
	///
	/// By default this is `None`.
	fn rpc_http(&self, _default_listen_port: u16) -> Result<Option<SocketAddr>> {
		Ok(None)
	}

	/// Get the RPC cors (`None` if disabled)
	///
	/// By default this is `Some(Vec::new())`.
	fn rpc_cors(&self, _is_dev: bool) -> Result<Option<Vec<String>>> {
		Ok(Some(Vec::new()))
	}

	/// Get the prometheus configuration (`None` if disabled)
	///
	/// By default this is `None`.
	fn prometheus_config(
		&self,
		_default_listen_port: u16,
	) -> Result<Option<PrometheusConfig>> {
		Ok(None)
	}

	/// Get the development key seed from the current object
	///
	/// By default this is `None`.
	fn dev_key_seed(&self, _is_dev: bool) -> Result<Option<String>> {
		Ok(Default::default())
	}

	/// Get the tracing targets from the current object (if any)
	///
	/// By default this is retrieved from [`SharedParams`] if it is available. Otherwise its
	/// `None`.
	fn tracing_targets(&self) -> Result<Option<String>> {
		Ok(self.shared_params().tracing_targets())
	}

	/// Get the TracingReceiver value from the current object
	///
	/// By default this is retrieved from [`SharedParams`] if it is available. Otherwise its
	/// `TracingReceiver::default()`.
	fn tracing_receiver(&self) -> Result<TracingReceiver> {
		Ok(self.shared_params().tracing_receiver())
	}

	/// Create a Configuration object from the current object
	fn create_configuration<C: WorkerCli>(
		&self,
		_cli: &C,
		tokio_handle: tokio::runtime::Handle,
	) -> Result<Configuration> {
		let is_dev = self.is_dev()?;
		let base_path = self
			.base_path()?
			.unwrap_or_else(|| BasePath::from_project("", "", &C::executable_name()));

		Ok(Configuration {
			impl_name: C::impl_name(),
			impl_version: C::impl_version(),
			tokio_handle,
			rpc_http: self.rpc_http(DCV::rpc_http_listen_port())?,
			rpc_cors: self.rpc_cors(is_dev)?,
			prometheus_config: self
				.prometheus_config(DCV::prometheus_listen_port())?,
			dev_key_seed: self.dev_key_seed(is_dev)?,
			tracing_targets: self.tracing_targets()?,
			tracing_receiver: self.tracing_receiver()?,
			base_path: Some(base_path),
		})
	}

	/// Get the filters for the logging.
	///
	/// This should be a list of comma-separated values.
	/// Example: `foo=trace,bar=debug,baz=info`
	///
	/// By default this is retrieved from `SharedParams`.
	fn log_filters(&self) -> Result<String> {
		Ok(self.shared_params().log_filters().join(","))
	}

	/// Should the detailed log output be enabled.
	fn detailed_log_output(&self) -> Result<bool> {
		Ok(self.shared_params().detailed_log_output())
	}

	/// Is log reloading enabled?
	fn enable_log_reloading(&self) -> Result<bool> {
		Ok(self.shared_params().enable_log_reloading())
	}

	/// Should the log color output be disabled?
	fn disable_log_color(&self) -> Result<bool> {
		Ok(self.shared_params().disable_log_color())
	}

	/// Initialize substrate. This must be done only once per process.
	///
	/// This method:
	///
	/// 1. Sets the panic handler
	/// 2. Optionally customize logger/profiling
	/// 2. Initializes the logger
	/// 3. Raises the FD limit
	///
	/// The `logger_hook` closure is executed before the logger is constructed
	/// and initialized. It is useful for setting up a custom profiler.
	///
	/// Example:
	/// ```
	/// use crate::tracing::{SpanDatum, TraceEvent};
	/// struct TestProfiler;
	///
	/// impl crate::tracing::TraceHandler for TestProfiler {
	///  	fn handle_span(&self, sd: &SpanDatum) {}
	/// 		fn handle_event(&self, _event: &TraceEvent) {}
	/// };
	///
	/// fn logger_hook() -> impl FnOnce(&mut crate::cli::LoggerBuilder, &sc_service::Configuration) -> () {
	/// 	|logger_builder, config| {
	/// 			logger_builder.with_custom_profiling(Box::new(TestProfiler{}));
	/// 	}
	/// }
	/// ```
	fn init<F>(
		&self,
		support_url: &String,
		impl_version: &String,
		logger_hook: F,
		config: &Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut LoggerBuilder, &Configuration),
	{
		sp_panic_handler::set(support_url, impl_version);

		let mut logger = LoggerBuilder::new(self.log_filters()?);
		logger
			.with_log_reloading(self.enable_log_reloading()?)
			.with_detailed_output(self.detailed_log_output()?);

		if let Some(tracing_targets) = self.tracing_targets()? {
			let tracing_receiver = self.tracing_receiver()?;
			logger.with_profiling(tracing_receiver, tracing_targets);
		}

		if self.disable_log_color()? {
			logger.with_colors(false);
		}

		// Call hook for custom profiling setup.
		logger_hook(&mut logger, config);

		logger.init()?;

		if let Some(new_limit) = fdlimit::raise_fd_limit() {
			if new_limit < RECOMMENDED_OPEN_FILE_DESCRIPTOR_LIMIT {
				warn!(
					"Low open file descriptor limit configured for the process. \
					Current value: {:?}, recommended value: {:?}.",
					new_limit, RECOMMENDED_OPEN_FILE_DESCRIPTOR_LIMIT,
				);
			}
		}

		Ok(())
	}
}
