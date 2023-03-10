#![cfg_attr(not(feature = "std"), no_std)]

use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use frame_support::RuntimeDebug;

use pallet_nfts::MintType;

/// Holds the information about minting.
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct NftMintSettings<Price, BlockNumber, CollectionId> {
	/// Whether anyone can mint or if minters are restricted to some subset.
	pub mint_type: MintType<CollectionId>, // TODO: remove dependency of pallet_nfts
	/// An optional price per mint.
	pub price: Option<Price>,
	/// When the mint starts.
	pub start_block: Option<BlockNumber>,
	/// When the mint ends.
	pub end_block: Option<BlockNumber>,
}

impl<Price, BlockNumber, CollectionId> Default for NftMintSettings<Price, BlockNumber, CollectionId> {
	fn default() -> Self {
		Self {
			mint_type: MintType::Issuer,
			price: None,
			start_block: None,
			end_block: None,
		}
	}
}

#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum NftItemAttributeKey {
	AcquiredBy,
	Status,
	Result,
	Output,
	CreatedAt,
	AcquiredAt,
	RejectedAt,
	ProcessingAt,
	ProcessedAt,
}

#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum NftItemStatus {
	/// Initial status, the item is pending to be processed
	Pending,
	/// Ending status, the worker reject to process the item, maybe the metadata is invalid.
	Rejected,
	/// The worker is processing the item
	Processing,
	/// Ending status, the worker processed the item
	Processed,
}

#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum NftItemResult {
	///  and report success
	Success,
	/// Ending status, the worker processed the item and report failed
	Failed,
	/// Ending status, the worker processed the item and report success
	Errored,
}
