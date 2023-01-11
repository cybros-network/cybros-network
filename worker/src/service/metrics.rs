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

use std::time::SystemTime;

use prometheus_endpoint::{register, Gauge, GaugeVec, Opts, PrometheusError, Registry, U64};
use sc_utils::metrics::register_globals;

struct PrometheusMetrics {
	// generic info
	block_height: GaugeVec<U64>,
}

impl PrometheusMetrics {
	fn setup(
		registry: &Registry,
		name: &str,
		version: &str,
	) -> Result<Self, PrometheusError> {
		register(
			Gauge::<U64>::with_opts(
				Opts::new(
					"worker_build_info",
					"A metric with a constant '1' value labeled by name, version",
				)
				.const_label("name", name)
				.const_label("version", version),
			)?,
			registry,
		)?
		.set(1);

		register_globals(registry)?;

		let start_time_since_epoch =
			SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
		register(
			Gauge::<U64>::new(
				"worker_process_start_time_seconds",
				"Number of seconds between the UNIX epoch and the moment the process started",
			)?,
			registry,
		)?
		.set(start_time_since_epoch.as_secs());

		Ok(Self {
			// generic internals
			block_height: register(
				GaugeVec::new(
					Opts::new("substrate_block_height", "Block height info of the chain"),
					&["status"],
				)?,
				registry,
			)?,
		})
	}
}
