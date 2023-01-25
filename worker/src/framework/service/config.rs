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

//! Service configuration.

use prometheus_endpoint::Registry;
use std::{
	io,
	net::SocketAddr,
	path::{Path, PathBuf},
};
use tempfile::TempDir;

/// Service configuration.
#[derive(Debug)]
pub struct Configuration {
	/// Implementation name
	pub impl_name: String,
	/// Implementation version (see sc-cli to see an example of format)
	pub impl_version: String,
	/// Handle to the tokio runtime. Will be used to spawn futures by the task manager.
	pub tokio_handle: tokio::runtime::Handle,
	/// RPC over HTTP binding address. `None` if disabled.
	pub rpc_http: Option<SocketAddr>,
	/// CORS settings for HTTP & WS servers. `None` if all origins are allowed.
	pub rpc_cors: Option<Vec<String>>,
	/// Prometheus endpoint configuration. `None` if disabled.
	pub prometheus_config: Option<PrometheusConfig>,
	/// Substrate node RPC URL
	pub substrate_rpc_url: url::Url,
	/// Development key seed.
	///
	/// When running in development mode, the seed will be used to generate authority keys by the
	/// keystore.
	///
	/// Should only be set when `node` is running development mode.
	pub dev_key_seed: Option<String>,
	/// Tracing targets
	pub tracing_targets: Option<String>,
	/// Tracing receiver
	pub tracing_receiver: crate::framework::tracing::TracingReceiver,
	/// Base path of the configuration
	pub base_path: Option<BasePath>,
}

/// Type for tasks spawned by the executor.
#[derive(PartialEq)]
pub enum TaskType {
	/// Regular non-blocking futures. Polling the task is expected to be a lightweight operation.
	Async,
	/// The task might perform a lot of expensive CPU operations and/or call `thread::sleep`.
	Blocking,
}

/// Configuration of the Prometheus endpoint.
#[derive(Debug, Clone)]
pub struct PrometheusConfig {
	/// Port to use.
	pub port: SocketAddr,
	/// A metrics registry to use. Useful for setting the metric prefix.
	pub registry: Registry,
}

impl PrometheusConfig {
	/// Create a new config using the default registry.
	pub fn new_with_default_registry(port: SocketAddr, labels: Option<std::collections::HashMap<String, String>>) -> Self {
		Self {
			port,
			registry: Registry::new_custom(None, labels)
				.expect("this can only fail if the prefix is empty"),
		}
	}
}

impl Configuration {
	/// Returns the prometheus metrics registry, if available.
	pub fn prometheus_registry(&self) -> Option<&Registry> {
		self.prometheus_config.as_ref().map(|config| &config.registry)
	}
}

#[static_init::dynamic(drop, lazy)]
static mut BASE_PATH_TEMP: Option<TempDir> = None;

/// The base path that is used for everything that needs to be written on disk to run a node.
#[derive(Debug)]
pub struct BasePath {
	path: PathBuf,
}

impl BasePath {
	/// Create a `BasePath` instance using a temporary directory prefixed with "substrate" and use
	/// it as base path.
	///
	/// Note: The temporary directory will be created automatically and deleted when the program
	/// exits. Every call to this function will return the same path for the lifetime of the
	/// program.
	pub fn new_temp_dir() -> io::Result<BasePath> {
		let mut temp = BASE_PATH_TEMP.write();

		match &*temp {
			Some(p) => Ok(Self::new(p.path())),
			None => {
				let temp_dir = tempfile::Builder::new().prefix("cybros_worker").tempdir()?;
				let path = PathBuf::from(temp_dir.path());

				*temp = Some(temp_dir);
				Ok(Self::new(path))
			},
		}
	}

	/// Create a `BasePath` instance based on an existing path on disk.
	///
	/// Note: this function will not ensure that the directory exist nor create the directory. It
	/// will also not delete the directory when the instance is dropped.
	pub fn new<P: Into<PathBuf>>(path: P) -> BasePath {
		Self { path: path.into() }
	}

	/// Create a base path from values describing the project.
	pub fn from_project(qualifier: &str, organization: &str, application: &str) -> BasePath {
		BasePath::new(
			directories::ProjectDirs::from(qualifier, organization, application)
				.expect("app directories exist on all supported platforms; qed")
				.data_local_dir(),
		)
	}

	/// Retrieve the base path.
	pub fn path(&self) -> &Path {
		&self.path
	}

	/// Returns the configuration directory inside this base path.
	///
	/// The path looks like `$base_path/data`
	pub fn config_dir(&self) -> PathBuf {
		self.path().join("config")
	}
}

impl From<PathBuf> for BasePath {
	fn from(path: PathBuf) -> Self {
		BasePath::new(path)
	}
}
