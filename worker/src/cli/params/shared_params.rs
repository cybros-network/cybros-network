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

use crate::cli::arg_enums::TracingReceiver;
use clap::Args;
use crate::service::config::BasePath;
use std::path::PathBuf;

/// Shared parameters used by all `CoreParams`.
#[derive(Debug, Clone, PartialEq, Args)]
pub struct SharedParams {
	/// Specify the development mode.
	///
	#[arg(long)]
	pub dev: bool,

	/// Specify custom base path.
	#[arg(long, short = 'd', value_name = "PATH")]
	pub base_path: Option<PathBuf>,

	/// Sets a custom logging filter. Syntax is `<target>=<level>`, e.g. -lsync=debug.
	///
	/// Log levels (least to most verbose) are error, warn, info, debug, and trace.
	/// By default, all targets log `info`. The global log level can be set with `-l<level>`.
	#[arg(short = 'l', long, value_name = "LOG_PATTERN", num_args = 1..)]
	pub log: Vec<String>,

	/// Enable detailed log output.
	///
	/// This includes displaying the log target, log level and thread name.
	///
	/// This is automatically enabled when something is logged with any higher level than `info`.
	#[arg(long)]
	pub detailed_log_output: bool,

	/// Disable log color output.
	#[arg(long)]
	pub disable_log_color: bool,

	/// Enable feature to dynamically update and reload the log filter.
	///
	/// Be aware that enabling this feature can lead to a performance decrease up to factor six or
	/// more. Depending on the global logging level the performance decrease changes.
	///
	/// The `system_addLogFilter` and `system_resetLogFilter` RPCs will have no effect with this
	/// option not being set.
	#[arg(long)]
	pub enable_log_reloading: bool,

	/// Sets a custom profiling filter. Syntax is the same as for logging: `<target>=<level>`.
	#[arg(long, value_name = "TARGETS")]
	pub tracing_targets: Option<String>,

	/// Receiver to process tracing messages.
	#[arg(long, value_name = "RECEIVER", value_enum, ignore_case = true, default_value_t = TracingReceiver::Log)]
	pub tracing_receiver: TracingReceiver,
}

impl SharedParams {
	/// Specify custom base path.
	pub fn base_path(&self) -> Result<Option<BasePath>, crate::cli::Error> {
		match &self.base_path {
			Some(r) => Ok(Some(r.clone().into())),
			// If `dev` is enabled, we use the temp base path.
			None if self.is_dev() => Ok(Some(BasePath::new_temp_dir()?)),
			None => Ok(None),
		}
	}

	/// Specify the development mode.
	pub fn is_dev(&self) -> bool {
		self.dev
	}

	/// Get the filters for the logging
	pub fn log_filters(&self) -> &[String] {
		&self.log
	}

	/// Should the detailed log output be enabled.
	pub fn detailed_log_output(&self) -> bool {
		self.detailed_log_output
	}

	/// Should the log color output be disabled?
	pub fn disable_log_color(&self) -> bool {
		self.disable_log_color
	}

	/// Is log reloading enabled
	pub fn enable_log_reloading(&self) -> bool {
		self.enable_log_reloading
	}

	/// Receiver to process tracing messages.
	pub fn tracing_receiver(&self) -> crate::service::TracingReceiver {
		self.tracing_receiver.into()
	}

	/// Comma separated list of targets for tracing.
	pub fn tracing_targets(&self) -> Option<String> {
		self.tracing_targets.clone()
	}
}
