// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
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

use crate::cli::{
	error::Result,
	params::{
		KeystoreParams, SharedParams,
	},
	CliConfiguration,
};
use clap::Parser;
use crate::service::config::{BasePath, PrometheusConfig};
use std::net::{Ipv4Addr, SocketAddr};

/// The `run` command used to run a node.
#[derive(Debug, Clone, Parser)]
pub struct RunCmd {
	/// Listen to all RPC interfaces.
	///
	/// Default is local.
	#[arg(long)]
	pub rpc_external: bool,

	/// Expose Prometheus exporter on all interfaces.
	///
	/// Default is local.
	#[arg(long)]
	pub prometheus_external: bool,

	/// Specify HTTP RPC server TCP port.
	#[arg(long, value_name = "PORT")]
	pub rpc_port: Option<u16>,

	/// Specify browser Origins allowed to access the HTTP & WS RPC servers.
	///
	/// A comma-separated list of origins (protocol://domain or special `null`
	/// value). Value of `all` will disable origin validation. Default is to
	/// allow localhost and <https://polkadot.js.org> origins. When running in
	/// --dev mode the default is to allow all origins.
	#[arg(long, value_name = "ORIGINS", value_parser = parse_cors)]
	pub rpc_cors: Option<Cors>,

	/// Specify Prometheus exporter TCP Port.
	#[arg(long, value_name = "PORT")]
	pub prometheus_port: Option<u16>,

	/// Do not expose a Prometheus exporter endpoint.
	///
	/// Prometheus metric endpoint is enabled by default.
	#[arg(long)]
	pub no_prometheus: bool,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub keystore_params: KeystoreParams,

	/// Run a temporary node.
	///
	/// A temporary directory will be created to store the configuration and will be deleted
	/// at the end of the process.
	///
	/// Note: the directory is random per process execution. This directory is used as base path
	/// which includes: database, node key and keystore.
	///
	/// When `--dev` is given and no explicit `--base-path`, this option is implied.
	#[arg(long, conflicts_with = "base_path")]
	pub tmp: bool,
}

impl CliConfiguration for RunCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.shared_params
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		Some(&self.keystore_params)
	}

	fn dev_key_seed(&self, is_dev: bool) -> Result<Option<String>> {
		Ok(
			if is_dev {
				Some("//Alice".into())
			} else {
				None
			}
		)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
	) -> Result<Option<PrometheusConfig>> {
		Ok(if self.no_prometheus {
			None
		} else {
			let interface =
				if self.prometheus_external { Ipv4Addr::UNSPECIFIED } else { Ipv4Addr::LOCALHOST };

			Some(PrometheusConfig::new_with_default_registry(
				SocketAddr::new(
					interface.into(),
					self.prometheus_port.unwrap_or(default_listen_port),
				),
				None
			))
		})
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		Ok(self
			.rpc_cors
			.clone()
			.unwrap_or_else(|| {
				if is_dev {
					log::warn!("Running in --dev mode, RPC CORS has been disabled.");
					Cors::All
				} else {
					Cors::List(vec![
						"http://localhost:*".into(),
						"http://127.0.0.1:*".into(),
						"https://localhost:*".into(),
						"https://127.0.0.1:*".into(),
						"https://polkadot.js.org".into(),
					])
				}
			})
			.into())
	}

	fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		let interface =
			if self.rpc_external { Ipv4Addr::UNSPECIFIED } else { Ipv4Addr::LOCALHOST };

		Ok(Some(SocketAddr::new(interface.into(), self.rpc_port.unwrap_or(default_listen_port))))
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(if self.tmp {
			Some(BasePath::new_temp_dir()?)
		} else {
			match self.shared_params().base_path()? {
				Some(r) => Some(r),
				// If `dev` is enabled, we use the temp base path.
				None if self.shared_params().is_dev() => Some(BasePath::new_temp_dir()?),
				None => None,
			}
		})
	}
}

#[derive(Debug)]
enum TelemetryParsingError {
	MissingVerbosity,
	VerbosityParsingError(std::num::ParseIntError),
}

impl std::error::Error for TelemetryParsingError {}

impl std::fmt::Display for TelemetryParsingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TelemetryParsingError::MissingVerbosity => write!(f, "Verbosity level missing"),
			TelemetryParsingError::VerbosityParsingError(e) => write!(f, "{}", e),
		}
	}
}

fn parse_telemetry_endpoints(s: &str) -> std::result::Result<(String, u8), TelemetryParsingError> {
	let pos = s.find(' ');
	match pos {
		None => Err(TelemetryParsingError::MissingVerbosity),
		Some(pos_) => {
			let url = s[..pos_].to_string();
			let verbosity =
				s[pos_ + 1..].parse().map_err(TelemetryParsingError::VerbosityParsingError)?;
			Ok((url, verbosity))
		},
	}
}

/// CORS setting
///
/// The type is introduced to overcome `Option<Option<T>>` handling of `clap`.
#[derive(Clone, Debug)]
pub enum Cors {
	/// All hosts allowed.
	All,
	/// Only hosts on the list are allowed.
	List(Vec<String>),
}

impl From<Cors> for Option<Vec<String>> {
	fn from(cors: Cors) -> Self {
		match cors {
			Cors::All => None,
			Cors::List(list) => Some(list),
		}
	}
}

/// Parse cors origins.
fn parse_cors(s: &str) -> Result<Cors> {
	let mut is_all = false;
	let mut origins = Vec::new();
	for part in s.split(',') {
		match part {
			"all" | "*" => {
				is_all = true;
				break
			},
			other => origins.push(other.to_owned()),
		}
	}

	if is_all {
		Ok(Cors::All)
	} else {
		Ok(Cors::List(origins))
	}
}
