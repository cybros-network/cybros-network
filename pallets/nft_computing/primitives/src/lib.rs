#![cfg_attr(not(feature = "std"), no_std)]

use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use frame_support::RuntimeDebug;

use pallet_nfts::MintType;

/// Holds the information about minting.
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct NFTMintSettings<Price, BlockNumber, CollectionId> {
	/// Whether anyone can mint or if minters are restricted to some subset.
	pub mint_type: MintType<CollectionId>,
	/// An optional price per mint.
	pub price: Option<Price>,
	/// When the mint starts.
	pub start_block: Option<BlockNumber>,
	/// When the mint ends.
	pub end_block: Option<BlockNumber>,
}

impl<Price, BlockNumber, CollectionId> Default for NFTMintSettings<Price, BlockNumber, CollectionId> {
	fn default() -> Self {
		Self {
			mint_type: MintType::Issuer,
			price: None,
			start_block: None,
			end_block: None,
		}
	}
}
