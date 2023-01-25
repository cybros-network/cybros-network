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

//! Initialization errors.

/// Result type alias for the CLI.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the CLI.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Cli(#[from] clap::Error),

	#[error(transparent)]
	Service(#[from] crate::framework::service::error::Error),

	#[error("Invalid input: {0}")]
	Input(String),

	#[error("Invalid hexadecimal string data, {0:?}")]
	HexDataConversion(array_bytes::Error),

	/// Application specific error chain sequence forwarder.
	#[error(transparent)]
	Application(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),

	#[error(transparent)]
	GlobalLoggerError(#[from] crate::framework::tracing::logging::Error),
}

impl From<&str> for Error {
	fn from(s: &str) -> Error {
		Error::Input(s.to_string())
	}
}

impl From<String> for Error {
	fn from(s: String) -> Error {
		Error::Input(s)
	}
}

impl From<array_bytes::Error> for Error {
	fn from(e: array_bytes::Error) -> Error {
		Error::HexDataConversion(e)
	}
}
