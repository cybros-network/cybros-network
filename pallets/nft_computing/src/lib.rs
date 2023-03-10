#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

/// The log target of this pallet.
pub const LOG_TARGET: &str = "runtime::nft_computing";

// Syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: $crate::LOG_TARGET,
			concat!("[{:?}] ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}

use frame_support::{
	traits::{
		tokens::nonfungibles_v2::{
			Inspect as NonFungiblesInspect,
			Create as NonFungiblesCreate,
			Destroy as NonFungiblesDestroy,
			Mutate as NonFungiblesMutate,
		},
		Currency, ReservableCurrency,
	},
};
use pallet_computing_workers::{
	traits::{WorkerLifecycleHooks, WorkerManageable},
	primitives::{OfflineReason, OnlinePayload, VerifiedAttestation},
};
use pallet_nfts::{
	CollectionSettings, CollectionSetting, ItemSettings, ItemSetting,
	ItemConfig, Incrementable, CollectionConfig,
};
use primitives::{NftMintSettings, NftItemAttributeKey, NftItemStatus, NftItemResult};

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub(crate) type CollectionConfigOf<T> = CollectionConfig<
	BalanceOf<T>,
	<T as frame_system::Config>::BlockNumber,
	<T as Config>::NftCollectionId
>;
pub type MintSettingsOf<T> = pallet_nfts::MintSettings<
	BalanceOf<T>,
	<T as frame_system::Config>::BlockNumber,
	<T as Config>::NftCollectionId
>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::{fmt::Display, prelude::*};
	use sp_runtime::traits::AtLeast32BitUnsigned;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type WorkerManageable: WorkerManageable<Self::AccountId, Self::BlockNumber>;

		type Currency: ReservableCurrency<Self::AccountId>;

		/// Identifier for the collection of NFT.
		type NftCollectionId: Member + Parameter + MaxEncodedLen + Copy + Display + AtLeast32BitUnsigned;

		/// The type used to identify an NFT within a collection.
		type NftItemId: Member + Parameter + MaxEncodedLen + Copy + Display + Incrementable;

		type Nfts: NonFungiblesInspect<
			Self::AccountId,
			ItemId = Self::NftItemId,
			CollectionId = Self::NftCollectionId,
		> + NonFungiblesCreate<
			Self::AccountId, CollectionConfigOf<Self>
		> + NonFungiblesDestroy<
			Self::AccountId
		> + NonFungiblesMutate<
			Self::AccountId, ItemConfig
		>;

		/// The maximum length of metadata stored on-chain.
		#[pallet::constant]
		type MetadataLimit: Get<u32>;

		/// The maximum length of processed output stored on-chain.
		#[pallet::constant]
		type OutputLimit: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CollectionCreated { worker: T::AccountId, collection_id: T::NftCollectionId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		WorkerNotFound,
		CollectionNotFound,
		ItemNotFound,
		NoPermission,
	}

	/// Stores the `CollectionId` that is going to be used for the next collection.
	/// This gets incremented whenever a new collection is created.
	#[pallet::storage]
	pub type NextCollectionItemId<T: Config> =
		StorageMap<
			_,
			Blake2_128Concat,
			T::NftCollectionId,
			T::NftItemId,
			OptionQuery,
		>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_collection(
			origin: OriginFor<T>,
			worker: T::AccountId,
			mint_settings: NftMintSettings<BalanceOf<T>, T::BlockNumber, T::NftCollectionId>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_owner(&who, &worker)?;

			let collection_config = CollectionConfigOf::<T> {
				settings: CollectionSettings::from_disabled(
					CollectionSetting::TransferableItems |
						CollectionSetting::UnlockedMetadata |
						CollectionSetting::UnlockedAttributes |
						CollectionSetting::UnlockedMaxSupply
				),
				max_supply: None,
				mint_settings: MintSettingsOf::<T> {
					mint_type: mint_settings.mint_type,
					price: mint_settings.price,
					start_block: mint_settings.start_block,
					end_block: mint_settings.end_block,
					default_item_settings: ItemSettings::from_disabled(
						ItemSetting::Transferable |
							ItemSetting::UnlockedMetadata |
							ItemSetting::UnlockedAttributes
					),
				}
			};

			// TODO: I'm thinking the owner should be the pallet account
			let collection_id =
				T::Nfts::create_collection(&worker, &worker, &collection_config)?;

			// TODO: remove this later, it just a test
			Self::deposit_event(Event::CollectionCreated { worker: worker.clone(), collection_id });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn mint(
			origin: OriginFor<T>,
			collection_id: T::NftCollectionId,
			metadata: BoundedVec<u8, T::MetadataLimit>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let item_id = NextCollectionItemId::<T>::get(&collection_id).unwrap_or(T::NftItemId::initial_value());
			let item_config = ItemConfig {
				settings: ItemSettings::from_disabled(
					ItemSetting::Transferable | ItemSetting::UnlockedMetadata | ItemSetting::UnlockedAttributes
				)
			};

			T::Nfts::mint_into(
				&collection_id,
				&item_id,
				&who,
				&item_config,
				false,
			)?;

			// empty key is metadata, this should be improved
			// TODO: need add `maybe_depositor` or something, or it won't reserve money !!!
			let metadata_key = Vec::<u8>::default();
			T::Nfts::set_attribute(
				&collection_id,
				&item_id,
				&metadata_key,
				&metadata
			)?;

			T::Nfts::set_typed_attribute::<NftItemAttributeKey, NftItemStatus>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::Status,
				&NftItemStatus::Pending
			)?;
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, T::BlockNumber>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::CreatedAt,
				&frame_system::Pallet::<T>::block_number()
			)?;
			// TODO: maybe more attributes? e.g. expires_at

			let next_collection_item_id = item_id.increment();
			NextCollectionItemId::<T>::insert(&collection_id, next_collection_item_id);

			Ok(())
		}

		/// Burn, the NFT owner burn it to recycle deposits.
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn burn(
			origin: OriginFor<T>,
			collection_id: T::NftCollectionId,
			item_id: T::NftItemId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// TODO: some case we should allow non-owner burn the item.
			Self::ensure_nft_owner(&who, &collection_id, &item_id)?;

			// TODO: Check status.
			// Q: allow burn when the worker processing it?
			// TODO: Handle when processing

			// TODO: some case we should allow non-owner burn the item.
			T::Nfts::burn(
				&collection_id,
				&item_id,
				None, // we have validate this above
			)?;

			// TODO: deposit event

			Ok(())
		}

		/// Acquire, the worker acquire a NFT, other workers will not attempt it
		/// Currently the collection only have one worker, so this step actually is unnecessary,
		/// but I still make the step, I should keep in mind in a same block many works would try to
		/// acquire an item, that's should be avoid
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn acquire(
			origin: OriginFor<T>,
			collection_id: T::NftCollectionId,
			item_id: T::NftItemId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_worker_collection(&who, &collection_id)?;

			// TODO: Performance can be improved
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, T::AccountId>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::AcquiredBy,
				&who
			)?;
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, T::BlockNumber>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::AcquiredAt,
				&frame_system::Pallet::<T>::block_number()
			)?;

			// TODO: Make a storage that store the worker acquired items
			// TODO: Add a constant for a worker maximum amount of acquired items

			// TODO: deposit event

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn reject(
			origin: OriginFor<T>,
			collection_id: T::NftCollectionId,
			item_id: T::NftItemId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_worker_collection(&who, &collection_id)?;

			// TODO: Performance can be improved
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, NftItemStatus>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::Status,
				&NftItemStatus::Rejected
			)?;
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, T::BlockNumber>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::RejectedAt,
				&frame_system::Pallet::<T>::block_number()
			)?;

			// TODO: Release acquired

			// TODO: deposit event

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn processing(
			origin: OriginFor<T>,
			collection_id: T::NftCollectionId,
			item_id: T::NftItemId
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_worker_collection(&who, &collection_id)?;

			// TODO: Performance can be improved
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, NftItemStatus>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::Status,
				&NftItemStatus::Processing
			)?;
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, T::BlockNumber>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::ProcessingAt,
				&frame_system::Pallet::<T>::block_number()
			)?;

			// TODO: deposit event

			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn processed(
			origin: OriginFor<T>,
			collection_id: T::NftCollectionId,
			item_id: T::NftItemId,
			result: NftItemResult,
			output: BoundedVec<u8, T::OutputLimit>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_worker_collection(&who, &collection_id)?;

			// TODO: Performance can be improved
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, NftItemStatus>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::Status,
				&NftItemStatus::Processed
			)?;
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, T::BlockNumber>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::ProcessedAt,
				&frame_system::Pallet::<T>::block_number()
			)?;
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, NftItemResult>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::Result,
				&result
			)?;
			T::Nfts::set_typed_attribute::<NftItemAttributeKey, Vec<u8>>(
				&collection_id,
				&item_id,
				&NftItemAttributeKey::Output,
				&output
			)?;

			// TODO: Release acquired

			// TODO: deposit event

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn ensure_worker(who: &T::AccountId) -> DispatchResult {
			ensure!(T::WorkerManageable::worker_exists(who), Error::<T>::WorkerNotFound);

			Ok(())
		}

		fn ensure_owner(who: &T::AccountId, worker: &T::AccountId) -> DispatchResult {
			if let Some(worker_info) = T::WorkerManageable::worker_info(worker) {
				ensure!(who == &worker_info.owner, Error::<T>::NoPermission);
			} else {
				return Err(Error::<T>::WorkerNotFound.into())
			}

			Ok(())
		}

		fn ensure_worker_collection(
			who: &T::AccountId,
			collection_id: &T::NftCollectionId
		) -> DispatchResult {
			Self::ensure_worker(who)?;

			let Some(owner) = T::Nfts::collection_owner(collection_id) else {
				return Err(Error::<T>::CollectionNotFound.into())
			};
			ensure!(
				who == &owner,
				Error::<T>::NoPermission
			);

			Ok(())
		}

		fn ensure_nft_owner(
			who: &T::AccountId,
			collection_id: &T::NftCollectionId,
			item_id: &T::NftItemId
		) -> DispatchResult {
			// Q: Do we need verify this is Worker's collection?
			let Some(owner) = T::Nfts::owner(collection_id, item_id) else {
				return Err(Error::<T>::NoPermission.into())
			};

			ensure!(
				who == &owner,
				Error::<T>::NoPermission
			);

			Ok(())
		}
	}

	impl<T: Config> WorkerLifecycleHooks<T::AccountId> for Pallet<T> {
		fn can_online(_worker: &T::AccountId, _payload: &OnlinePayload, _verified_attestation: &Option<VerifiedAttestation>) -> DispatchResult {
			Ok(())
		}

		fn after_online(_worker: &T::AccountId) {
			// Nothing to do
		}

		fn can_offline(_worker: &T::AccountId) -> bool {
			true
		}

		fn before_offline(_worker: &T::AccountId, _reason: OfflineReason) {
			// Nothing to do
		}

		fn after_refresh_attestation(_worker: &T::AccountId, _payload: &OnlinePayload, _verified_attestation: &Option<VerifiedAttestation>) {
			// Nothing to do
		}

		fn after_requesting_offline(_worker: &T::AccountId) {
			// Nothing to do
		}

		fn before_deregister(_worker: &T::AccountId) {
			// Nothing to do
		}
	}
}
