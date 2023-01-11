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

mod keystore_params;
mod shared_params;

use crate::cli::arg_enums::{CryptoScheme, OutputType};
use clap::Args;
use sp_core::crypto::{Ss58AddressFormat, Ss58AddressFormatRegistry};
use std::fmt::Debug;

pub use crate::cli::params::{
	keystore_params::*, shared_params::*,
};

/// Parse Ss58AddressFormat
pub fn parse_ss58_address_format(x: &str) -> Result<Ss58AddressFormat, String> {
	match Ss58AddressFormatRegistry::try_from(x) {
		Ok(format_registry) => Ok(format_registry.into()),
		Err(_) => Err(format!(
			"Unable to parse variant. Known variants: {:?}",
			Ss58AddressFormat::all_names()
		)),
	}
}

/// Optional flag for specifying crypto algorithm
#[derive(Debug, Clone, Args)]
pub struct CryptoSchemeFlag {
	/// cryptography scheme
	#[arg(long, value_name = "SCHEME", value_enum, ignore_case = true, default_value_t = CryptoScheme::Sr25519)]
	pub scheme: CryptoScheme,
}

/// Optional flag for specifying output type
#[derive(Debug, Clone, Args)]
pub struct OutputTypeFlag {
	/// output format
	#[arg(long, value_name = "FORMAT", value_enum, ignore_case = true, default_value_t = OutputType::Text)]
	pub output_type: OutputType,
}

/// Optional flag for specifying network scheme
#[derive(Debug, Clone, Args)]
pub struct NetworkSchemeFlag {
	/// network address format
	#[arg(
		short = 'n',
		long,
		value_name = "NETWORK",
		ignore_case = true,
		value_parser = parse_ss58_address_format,
	)]
	pub network: Option<Ss58AddressFormat>,
}
