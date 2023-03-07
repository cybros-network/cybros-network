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

use sp_runtime::traits::Zero;
use frame_support::{
	traits::{
		tokens::AttributeNamespace,
		Currency, ExistenceRequirement,
	},
	BoundedVec,
};
use pallet_computing_workers::{
	traits::{WorkerLifecycleHooks, WorkerManageable},
	primitives::{OfflineReason, OnlinePayload, VerifiedAttestation},
};
use pallet_nfts::{
	CollectionSettings, CollectionSetting, ItemSettings, ItemSetting,
	ItemConfig, MintType, Incrementable,
	CollectionRole, Attribute, AttributeDeposit, MintWitness, PalletAttributes,
	Account,
};
use primitives::NFTMintSettings;

pub type BalanceOf<T, I = ()> =
	<<T as pallet_nfts::Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type CollectionIdOf<T, I = ()> = <T as pallet_nfts::Config<I>>::CollectionId;
pub type ItemIdOf<T, I = ()> = <T as pallet_nfts::Config<I>>::ItemId;
pub type CollectionDepositOf<T, I = ()> = <T as pallet_nfts::Config<I>>::CollectionDeposit;
pub type CollectionConfigOf<T, I = ()> = pallet_nfts::CollectionConfig<
	BalanceOf<T, I>,
	<T as frame_system::Config>::BlockNumber,
	CollectionIdOf<T, I>
>;
pub type MintSettingsOf<T, I = ()> = pallet_nfts::MintSettings<
	BalanceOf<T, I>,
	<T as frame_system::Config>::BlockNumber,
	CollectionIdOf<T, I>
>;

pub type PalletNFT<T, I = ()> = pallet_nfts::Pallet<T, I>;

pub type NFTMintSettingsOf<T, I = ()> = NFTMintSettings<
	BalanceOf<T, I>,
	<T as frame_system::Config>::BlockNumber,
	CollectionIdOf<T, I>
>;

pub type KeyBoundedVec<T, I = ()> = BoundedVec::<u8, <T as pallet_nfts::Config<I>>::KeyLimit>;
pub type ValueBoundedVec<T, I = ()> = BoundedVec::<u8, <T as pallet_nfts::Config<I>>::ValueLimit>;
pub type StringBoundedVec<T, I = ()> = BoundedVec::<u8, <T as pallet_nfts::Config<I>>::StringLimit>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T, I = ()>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config + pallet_nfts::Config<I> {
		/// Because this pallet emits events, it depends on the runtime definition of an event.
		type RuntimeEvent: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type WorkerManageable: WorkerManageable<Self::AccountId, Self::BlockNumber>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		CollectionCreated { worker: T::AccountId, collection_id: CollectionIdOf<T, I> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T, I = ()> {
		NotTheOwner,
		WorkerNotExists,
	}

	/// Stores the `CollectionId` that is going to be used for the next collection.
	/// This gets incremented whenever a new collection is created.
	#[pallet::storage]
	pub type NextItemId<T: Config<I>, I: 'static = ()> =
		StorageMap<
			_,
			Blake2_128Concat,
			CollectionIdOf<T, I>,
			ItemIdOf<T, I>,
			OptionQuery,
		>;

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_collection(
			origin: OriginFor<T>,
			worker: T::AccountId,
			mint_settings: NFTMintSettingsOf<T, I>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_owner(&who, &worker)?;

			let collection_config = CollectionConfigOf::<T, I> {
				settings: CollectionSettings::from_disabled(
					CollectionSetting::TransferableItems |
						CollectionSetting::UnlockedMetadata |
						CollectionSetting::UnlockedAttributes |
						CollectionSetting::UnlockedMaxSupply
				),
				max_supply: None,
				mint_settings: MintSettingsOf::<T, I> {
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

			let collection =
				pallet_nfts::NextCollectionId::<T, I>::get().unwrap_or(CollectionIdOf::<T, I>::initial_value());

			pallet_nfts::Pallet::<T, I>::do_create_collection(
				collection,
				who.clone(),
				who.clone(),
				collection_config,
				CollectionDepositOf::<T, I>::get(),
				pallet_nfts::Event::<T, I>::Created { collection, creator: who.clone(), owner: who.clone() },
			)?;

			// let collection_id =
			// 	T::NonFungibles::create_collection(&worker, &worker, &collection_config)?;
			// // TODO: add a mapping
			//
			// // TODO: CollectionId need Debug and Clone, need PR to Substrate
			// Self::deposit_event(Event::CollectionCreated { worker: worker.clone(), collection_id });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn mint(
			origin: OriginFor<T>,
			collection_id: CollectionIdOf<T, I>,
			witness_data: Option<MintWitness<ItemIdOf<T, I>>>,
			metadata: StringBoundedVec<T, I>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let item_id = NextItemId::<T, I>::get(&collection_id).unwrap_or(0u32.into());
			let item_config = ItemConfig {
				settings: ItemSettings::from_disabled(
					ItemSetting::Transferable | ItemSetting::UnlockedMetadata
				)
			};

			pallet_nfts::Pallet::<T, I>::do_mint(
				collection_id,
				item_id.into(),
				Some(who.clone()),
				who.clone(),
				item_config,
				|collection_details, collection_config| {
					// Issuer can mint regardless of mint settings
					if PalletNFT::<T, I>::has_role(&collection_id, &who, CollectionRole::Issuer) {
						return Ok(())
					}

					let mint_settings = collection_config.mint_settings;
					let now = frame_system::Pallet::<T>::block_number();

					if let Some(start_block) = mint_settings.start_block {
						ensure!(start_block <= now, pallet_nfts::Error::<T, I>::MintNotStarted);
					}
					if let Some(end_block) = mint_settings.end_block {
						ensure!(end_block >= now, pallet_nfts::Error::<T, I>::MintEnded);
					}

					match mint_settings.mint_type {
						MintType::Issuer => return Err(pallet_nfts::Error::<T, I>::NoPermission.into()),
						MintType::HolderOf(collection_id) => {
							let MintWitness { owner_of_item } =
								witness_data.ok_or(pallet_nfts::Error::<T, I>::BadWitness)?;

							let has_item = Account::<T, I>::contains_key((
								&who,
								&collection_id,
								&owner_of_item,
							));
							ensure!(has_item, pallet_nfts::Error::<T, I>::BadWitness);

							let attribute_key = PalletNFT::<T, I>::construct_attribute_key(
								PalletAttributes::<T::CollectionId>::UsedToClaim(collection_id)
									.encode(),
							)?;

							let key = (
								&collection_id,
								Some(owner_of_item),
								AttributeNamespace::Pallet,
								&attribute_key,
							);
							let already_claimed = Attribute::<T, I>::contains_key(key.clone());
							ensure!(!already_claimed, pallet_nfts::Error::<T, I>::AlreadyClaimed);

							let value = PalletNFT::<T, I>::construct_attribute_value(vec![0])?;
							Attribute::<T, I>::insert(
								key,
								(value, AttributeDeposit { account: None, amount: Zero::zero() }),
							);
						},
						_ => {},
					}

					if let Some(price) = mint_settings.price {
						T::Currency::transfer(
							&who,
							&collection_details.owner,
							price,
							ExistenceRequirement::KeepAlive,
						)?;
					}

					Ok(())
				},
			)?;

			PalletNFT::<T, I>::do_set_item_metadata(
				None,
				collection_id,
				item_id.into(),
				metadata,
				Some(who.clone()),
			)?;

			// PalletNFT::<T, I>::do_set_attribute(
			// 	who.clone(),
			// 	collection_id,
			// 	Some(item_id.into()),
			// 	AttributeNamespace::<T::AccountId>::CollectionOwner,
			// 	BoundedVec::<u8, <T as pallet_nfts::Config<I>>::KeyLimit>::truncate_from("foo".into()),
			// 	ValueBoundedVec::<T, I>,
			// 	who.clone(),
			// )?;

			// PalletNFT::<T, I>::do_force_set_attribute(
			// 	set_as: None,
			// 	collection_id,
			// 	Some(item_id.into()),
			// 	AttributeNamespace::<T::AccountId>::Pallet,
			// 	KeyBoundedVec::<T, I>::truncate_from("validated".into()),
			// 	ValueBoundedVec::<T, I>::,
			// )?;

			let next_item_id = item_id + 1u32.into();
			NextItemId::<T, I>::insert(&collection_id, next_item_id);

			Ok(())
		}
	}

	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		fn ensure_owner(who: &T::AccountId, worker: &T::AccountId) -> DispatchResult {
			if let Some(worker_info) = T::WorkerManageable::worker_info(worker) {
				ensure!(who == &worker_info.owner, Error::<T, I>::NotTheOwner);
			} else {
				return Err(Error::<T, I>::WorkerNotExists.into())
			}

			Ok(())
		}

		fn ensure_worker(who: &T::AccountId) -> DispatchResult {
			ensure!(T::WorkerManageable::worker_exists(who), Error::<T, I>::WorkerNotExists);

			Ok(())
		}
	}

	impl<T: Config<I>, I: 'static> WorkerLifecycleHooks<T::AccountId> for Pallet<T, I> {
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
