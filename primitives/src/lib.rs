//! Low-level types used throughout the Substrate code.

#![allow(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use constants::*;

pub mod types {
	use sp_runtime::{
		generic,
		traits::{IdentifyAccount, Verify},
		MultiSignature,
	};

	/// An index to a block.
	pub type BlockNumber = u32;

	/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
	pub type Signature = MultiSignature;

	/// Some way of identifying an account on the chain. We intentionally make it equivalent
	/// to the public key of our transaction signing scheme.
	pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

	/// The type for looking up accounts. We don't expect more than 4 billion of them.
	pub type AccountIndex = u32;

	/// Balance of an account.
	pub type Balance = u128;

	/// Id of a NFT collection.
	pub type CollectionId = u32;

	/// Id of a NFT item.
	pub type ItemId = u32;

	/// Type used for expressing timestamp.
	pub type Moment = u64;

	/// Index of a transaction in the chain.
	pub type Index = u32;

	/// A hash of some data used by the chain.
	pub type Hash = sp_core::H256;

	/// A timestamp: milliseconds since the unix epoch.
	/// `u64` is enough to represent a duration of half a billion years, when the
	/// time scale is milliseconds.
	pub type Timestamp = u64;

	/// Digest item type.
	pub type DigestItem = generic::DigestItem;
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use crate::types::BlockNumber;
	use sp_runtime::{generic, traits::BlakeTwo256};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}

pub mod constants {
	/// Money matters.
	pub mod currency {
		use crate::types::Balance;

		pub const MILLI_CENTS: Balance = 1_000_000;
		pub const CENTS: Balance = 1_000 * MILLI_CENTS;
		pub const UNITS: Balance = 1_000 * CENTS;

		/// Existential deposit.
		pub const EXISTENTIAL_DEPOSIT: u128 = CENTS;

		pub const fn deposit(items: u32, bytes: u32) -> Balance {
			items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
		}
	}

	/// Time.
	pub mod time {
		use crate::types::{BlockNumber, Moment};

		/// Since BABE is probabilistic this is the average expected block time that
		/// we are targeting. Blocks will be produced at a minimum duration defined
		/// by `SLOT_DURATION`, but some slots will not be allocated to any
		/// authority and hence no block will be produced. We expect to have this
		/// block time on average following the defined slot duration and the value
		/// of `c` configured for BABE (where `1 - c` represents the probability of
		/// a slot being empty).
		/// This value is only used indirectly to define the unit constants below
		/// that are expressed in blocks. The rest of the code should use
		/// `SLOT_DURATION` instead (like the Timestamp pallet for calculating the
		/// minimum period).
		///
		/// If using BABE with secondary slots (default) then all of the slots will
		/// always be assigned, in which case `MILLI_SECS_PER_BLOCK` and
		/// `SLOT_DURATION` should have the same value.
		///
		/// <https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html#-6.-practical-results>
		pub const MILLI_SECS_PER_BLOCK: Moment = 6000;
		pub const SECS_PER_BLOCK: Moment = MILLI_SECS_PER_BLOCK / 1000;

		// NOTE: Currently it is not possible to change the slot duration after the chain has started.
		//       Attempting to do so will brick block production.
		pub const SLOT_DURATION: Moment = MILLI_SECS_PER_BLOCK;

		// These time units are defined in number of blocks.
		pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
		pub const HOURS: BlockNumber = MINUTES * 60;
		pub const DAYS: BlockNumber = HOURS * 24;
	}

	pub mod weight {
		use frame_support::weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight};
		use sp_runtime::Perbill;

		/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
		/// This is used to limit the maximal weight of a single extrinsic.
		pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
		/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
		/// by  Operational  extrinsics.
		pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
		/// We allow for 2 seconds of compute with a 6 second average block time, with maximum proof size.
		pub const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), u64::MAX);
	}
}
