// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # DeviceId Module
//!
//! A simple, secure module for dealing with non-fungible items.
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`Support`](../frame_support/index.html)

#![recursion_limit = "256"]
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;

mod common_functions;
/// A library providing the feature set of this pallet. It contains modules with helper methods that
/// perform storage updates and checks required by this pallet's dispatchables. To use pallet level
/// features, make sure to set appropriate bitflags for [`Config::Features`] in your runtime
/// configuration trait.
mod features;
mod impl_nonfungibles;
mod types;

pub mod macros;
pub mod weights;

use scale_codec::{Decode, Encode};
use frame_support::traits::{
    tokens::Locker, BalanceStatus::Reserved, Currency, EnsureOriginWithArg, Incrementable,
    ReservableCurrency,
};
use sp_runtime::{
    traits::{IdentifyAccount, Saturating, StaticLookup, Verify, Zero},
    RuntimeDebug,
};
use sp_std::prelude::*;

pub use pallet::*;
pub use types::*;
pub use weights::WeightInfo;

/// The log target of this pallet.
pub const LOG_TARGET: &'static str = "runtime::device_id";

/// A type alias for the account ID type used in the dispatchable functions of this pallet.
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[cfg(feature = "runtime-benchmarks")]
    pub trait BenchmarkHelper<CollectionId, ItemId> {
        fn collection(i: u16) -> CollectionId;
        fn item(i: u16) -> ItemId;
    }
    #[cfg(feature = "runtime-benchmarks")]
    impl<CollectionId: From<u16>, ItemId: From<u16>> BenchmarkHelper<CollectionId, ItemId> for () {
        fn collection(i: u16) -> CollectionId {
            i.into()
        }
        fn item(i: u16) -> ItemId {
            i.into()
        }
    }

    #[pallet::config]
    /// The module configuration trait.
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Identifier for the collection of item.
        ///
        /// SAFETY: The functions in the `Incrementable` trait are fallible. If the functions
        /// of the implementation both return `None`, the automatic CollectionId generation
        /// should not be used. So the `create` and `force_create` extrinsics and the
        /// `create_collection` function will return an `UnknownCollection` Error. Instead use
        /// the `create_collection_with_id` function. However, if the `Incrementable` trait
        /// implementation has an incremental order, the `create_collection_with_id` function
        /// should not be used as it can claim a value in the ID sequence.
        type CollectionId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;

        /// The type used to identify a unique item within a collection.
        type ItemId: Member + Parameter + MaxEncodedLen + Copy;

        /// The currency mechanism, used for paying for reserves.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The origin which may forcibly create or destroy an item or otherwise alter privileged
        /// attributes.
        type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Standard collection creation is only allowed if the origin attempting it and the
        /// collection are in this set.
        type CreateOrigin: EnsureOriginWithArg<
            Self::RuntimeOrigin,
            Self::CollectionId,
            Success = Self::AccountId,
        >;

        /// Locker trait to enable Locking mechanism downstream.
        type Locker: Locker<Self::CollectionId, Self::ItemId>;

        /// The basic amount of funds that must be reserved for collection.
        #[pallet::constant]
        type CollectionDeposit: Get<DepositBalanceOf<Self>>;

        /// The basic amount of funds that must be reserved for an item.
        #[pallet::constant]
        type ItemDeposit: Get<DepositBalanceOf<Self>>;

        /// The basic amount of funds that must be reserved when adding metadata to your item.
        #[pallet::constant]
        type MetadataDepositBase: Get<DepositBalanceOf<Self>>;

        /// The basic amount of funds that must be reserved when adding an attribute to an item.
        #[pallet::constant]
        type AttributeDepositBase: Get<DepositBalanceOf<Self>>;

        /// The additional funds that must be reserved for the number of bytes store in metadata,
        /// either "normal" metadata or attribute metadata.
        #[pallet::constant]
        type DepositPerByte: Get<DepositBalanceOf<Self>>;

        /// The maximum length of data stored on-chain.
        #[pallet::constant]
        type StringLimit: Get<u32>;

        /// The maximum length of an attribute key.
        #[pallet::constant]
        type KeyLimit: Get<u32>;

        /// The maximum length of an attribute value.
        #[pallet::constant]
        type ValueLimit: Get<u32>;

        /// The maximum approvals an item could have.
        #[pallet::constant]
        type ApprovalsLimit: Get<u32>;

        /// The maximum attributes approvals an item could have.
        #[pallet::constant]
        type ItemAttributesApprovalsLimit: Get<u32>;

        /// The max number of tips a user could send.
        #[pallet::constant]
        type MaxTips: Get<u32>;

        /// The max duration in blocks for deadlines.
        #[pallet::constant]
        type MaxDeadlineDuration: Get<BlockNumberFor<Self>>;

        /// The max number of attributes a user could set per call.
        #[pallet::constant]
        type MaxAttributesPerCall: Get<u32>;

        /// Off-Chain signature type.
        ///
        /// Can verify whether an `Self::OffchainPublic` created a signature.
        type OffchainSignature: Verify<Signer = Self::OffchainPublic> + Parameter;

        /// Off-Chain public key.
        ///
        /// Must identify as an on-chain `Self::AccountId`.
        type OffchainPublic: IdentifyAccount<AccountId = Self::AccountId>;

        #[cfg(feature = "runtime-benchmarks")]
        /// A set of helper functions for benchmarking.
        type Helper: BenchmarkHelper<Self::CollectionId, Self::ItemId>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    /// Details of a collection.
    #[pallet::storage]
    pub type Collection<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::CollectionId,
        CollectionDetails<T::AccountId, DepositBalanceOf<T>>,
    >;

    /// The collection, if any, of which an account is willing to take ownership.
    #[pallet::storage]
    pub type OwnershipAcceptance<T: Config> =
    StorageMap<_, Blake2_128Concat, T::AccountId, T::CollectionId>;

    /// The items held by any given account; set out this way so that items owned by a single
    /// account can be enumerated.
    #[pallet::storage]
    pub type Account<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::AccountId>, // owner
            NMapKey<Blake2_128Concat, T::CollectionId>,
            NMapKey<Blake2_128Concat, T::ItemId>,
        ),
        (),
        OptionQuery,
    >;

    /// The collections owned by any given account; set out this way so that collections owned by
    /// a single account can be enumerated.
    #[pallet::storage]
    pub type CollectionAccount<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::CollectionId,
        (),
        OptionQuery,
    >;

    /// The items in existence and their ownership details.
    #[pallet::storage]
    /// Stores collection roles as per account.
    pub type CollectionRoleOf<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::CollectionId,
        Blake2_128Concat,
        T::AccountId,
        CollectionRoles,
        OptionQuery,
    >;

    /// The items in existence and their ownership details.
    #[pallet::storage]
    pub type Item<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::CollectionId,
        Blake2_128Concat,
        T::ItemId,
        ItemDetails<T::AccountId, ItemDepositOf<T>, ApprovalsOf<T>>,
        OptionQuery,
    >;

    /// Metadata of a collection.
    #[pallet::storage]
    pub type CollectionMetadataOf<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::CollectionId,
        CollectionMetadata<DepositBalanceOf<T>, T::StringLimit>,
        OptionQuery,
    >;

    /// Metadata of an item.
    #[pallet::storage]
    pub type ItemMetadataOf<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::CollectionId,
        Blake2_128Concat,
        T::ItemId,
        ItemMetadata<ItemMetadataDepositOf<T>, T::StringLimit>,
        OptionQuery,
    >;

    /// Attributes of a collection.
    #[pallet::storage]
    pub type Attribute<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::CollectionId>,
            NMapKey<Blake2_128Concat, Option<T::ItemId>>,
            NMapKey<Blake2_128Concat, AttributeNamespace<T::AccountId>>,
            NMapKey<Blake2_128Concat, BoundedVec<u8, T::KeyLimit>>,
        ),
        (BoundedVec<u8, T::ValueLimit>, AttributeDepositOf<T>),
        OptionQuery,
    >;

    /// Item attribute approvals.
    #[pallet::storage]
    pub type ItemAttributesApprovalsOf<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::CollectionId,
        Blake2_128Concat,
        T::ItemId,
        ItemAttributesApprovals<T>,
        ValueQuery,
    >;

    /// Stores the `CollectionId` that is going to be used for the next collection.
    /// This gets incremented whenever a new collection is created.
    #[pallet::storage]
    pub type NextCollectionId<T: Config> =
        StorageValue<_, T::CollectionId, OptionQuery>;

    /// Config of a collection.
    #[pallet::storage]
    pub type CollectionConfigOf<T: Config> =
        StorageMap<_, Blake2_128Concat, T::CollectionId, CollectionConfigFor<T>, OptionQuery>;

    /// Config of an item.
    #[pallet::storage]
    pub type ItemConfigOf<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::CollectionId,
        Blake2_128Concat,
        T::ItemId,
        ItemConfig,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A `collection` was created.
        Created { collection: T::CollectionId, creator: T::AccountId, owner: T::AccountId },
        /// A `collection` was force-created.
        ForceCreated { collection: T::CollectionId, owner: T::AccountId },
        /// A `collection` was destroyed.
        Destroyed { collection: T::CollectionId },
        /// An `item` was issued.
        Issued { collection: T::CollectionId, item: T::ItemId, owner: T::AccountId },
        /// An `item` was transferred.
        Transferred {
            collection: T::CollectionId,
            item: T::ItemId,
            from: T::AccountId,
            to: T::AccountId,
        },
        /// An `item` was destroyed.
        Burned { collection: T::CollectionId, item: T::ItemId, owner: T::AccountId },
        /// An `item` became non-transferable.
        ItemTransferLocked { collection: T::CollectionId, item: T::ItemId },
        /// An `item` became transferable.
        ItemTransferUnlocked { collection: T::CollectionId, item: T::ItemId },
        /// `item` metadata or attributes were locked.
        ItemPropertiesLocked {
            collection: T::CollectionId,
            item: T::ItemId,
            lock_metadata: bool,
            lock_attributes: bool,
        },
        /// Some `collection` was locked.
        CollectionLocked { collection: T::CollectionId },
        /// The owner changed.
        OwnerChanged { collection: T::CollectionId, new_owner: T::AccountId },
        /// The management team changed.
        TeamChanged {
            collection: T::CollectionId,
            issuer: Option<T::AccountId>,
            admin: Option<T::AccountId>,
            freezer: Option<T::AccountId>,
        },
        /// A `collection` has had its config changed by the `Force` origin.
        CollectionConfigChanged { collection: T::CollectionId },
        /// New metadata has been set for a `collection`.
        CollectionMetadataSet { collection: T::CollectionId, data: BoundedVec<u8, T::StringLimit> },
        /// Metadata has been cleared for a `collection`.
        CollectionMetadataCleared { collection: T::CollectionId },
        /// New metadata has been set for an item.
        ItemMetadataSet {
            collection: T::CollectionId,
            item: T::ItemId,
            data: BoundedVec<u8, T::StringLimit>,
        },
        /// Metadata has been cleared for an item.
        ItemMetadataCleared { collection: T::CollectionId, item: T::ItemId },
        /// The deposit for a set of `item`s within a `collection` has been updated.
        Redeposited { collection: T::CollectionId, successful_items: Vec<T::ItemId> },
        /// New attribute metadata has been set for a `collection` or `item`.
        AttributeSet {
            collection: T::CollectionId,
            maybe_item: Option<T::ItemId>,
            key: BoundedVec<u8, T::KeyLimit>,
            value: BoundedVec<u8, T::ValueLimit>,
            namespace: AttributeNamespace<T::AccountId>,
        },
        /// Attribute metadata has been cleared for a `collection` or `item`.
        AttributeCleared {
            collection: T::CollectionId,
            maybe_item: Option<T::ItemId>,
            key: BoundedVec<u8, T::KeyLimit>,
            namespace: AttributeNamespace<T::AccountId>,
        },
        /// A new approval to modify item attributes was added.
        ItemAttributesApprovalAdded {
            collection: T::CollectionId,
            item: T::ItemId,
            delegate: T::AccountId,
        },
        /// A new approval to modify item attributes was removed.
        ItemAttributesApprovalRemoved {
            collection: T::CollectionId,
            item: T::ItemId,
            delegate: T::AccountId,
        },
        /// Ownership acceptance has changed for an account.
        OwnershipAcceptanceChanged { who: T::AccountId, maybe_collection: Option<T::CollectionId> },
        /// Max supply has been set for a collection.
        CollectionMaxSupplySet { collection: T::CollectionId, max_supply: u32 },
        /// Mint settings for a collection had changed.
        CollectionMintSettingsUpdated { collection: T::CollectionId },
        /// Event gets emitted when the `NextCollectionId` gets incremented.
        NextCollectionIdIncremented { next_id: Option<T::CollectionId> },
        /// New attributes have been set for an `item` of the `collection`.
        PreSignedAttributesSet {
            collection: T::CollectionId,
            item: T::ItemId,
            namespace: AttributeNamespace<T::AccountId>,
        },
        /// A new attribute in the `Pallet` namespace was set for the `collection` or an `item`
        /// within that `collection`.
        PalletAttributeSet {
            collection: T::CollectionId,
            item: Option<T::ItemId>,
            attribute: PalletAttributes<T::CollectionId>,
            value: BoundedVec<u8, T::ValueLimit>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The signing account has no permission to do the operation.
        NoPermission,
        /// The given item ID is unknown.
        UnknownCollection,
        /// The item ID has already been used for an item.
        AlreadyExists,
        /// The owner turned out to be different to what was expected.
        WrongOwner,
        /// The witness data given does not match the current state of the chain.
        BadWitness,
        /// Collection ID is already taken.
        CollectionIdInUse,
        /// Items within that collection are non-transferable.
        ItemsNonTransferable,
        /// The provided account is not a delegate.
        NotDelegate,
        /// The delegate turned out to be different to what was expected.
        WrongDelegate,
        /// The named owner has not signed ownership acceptance of the collection.
        Unaccepted,
        /// The item is locked (non-transferable).
        ItemLocked,
        /// Item's attributes are locked.
        LockedItemAttributes,
        /// Collection's attributes are locked.
        LockedCollectionAttributes,
        /// Item's metadata is locked.
        LockedItemMetadata,
        /// Collection's metadata is locked.
        LockedCollectionMetadata,
        /// All items have been minted.
        MaxSupplyReached,
        /// The max supply is locked and can't be changed.
        MaxSupplyLocked,
        /// The provided max supply is less than the number of items a collection already has.
        MaxSupplyTooSmall,
        /// The given item ID is unknown.
        UnknownItem,
        /// The given item has no metadata set.
        MetadataNotFound,
        /// The provided attribute can't be found.
        AttributeNotFound,
        /// The item has reached its approval limit.
        ReachedApprovalLimit,
        /// The deadline has already expired.
        DeadlineExpired,
        /// The duration provided should be less than or equal to `MaxDeadlineDuration`.
        WrongDuration,
        /// The provided setting can't be set.
        WrongSetting,
        /// Item's config already exists and should be equal to the provided one.
        InconsistentItemConfig,
        /// Config for a collection or an item can't be found.
        NoConfig,
        /// Some roles were not cleared.
        RolesNotCleared,
        /// Mint has not started yet.
        MintNotStarted,
        /// Mint has already ended.
        MintEnded,
        /// The provided Item was already used for claiming.
        AlreadyClaimed,
        /// The provided data is incorrect.
        IncorrectData,
        /// The extrinsic was sent by the wrong origin.
        WrongOrigin,
        /// The provided signature is incorrect.
        WrongSignature,
        /// The provided metadata might be too long.
        IncorrectMetadata,
        /// Can't set more attributes per one call.
        MaxAttributesLimitReached,
        /// The provided namespace isn't supported in this call.
        WrongNamespace,
        /// Can't delete non-empty collections.
        CollectionNotEmpty,
        /// The witness data should be provided.
        WitnessRequired,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Issue a new collection of non-fungible items from a public origin.
        ///
        /// This new collection has no items initially and its owner is the origin.
        ///
        /// The origin must be Signed and the sender must have sufficient funds free.
        ///
        /// `CollectionDeposit` funds of sender are reserved.
        ///
        /// Parameters:
        /// - `admin`: The admin of this collection. The admin is the initial address of each
        /// member of the collection's admin team.
        ///
        /// Emits `Created` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create())]
        pub fn create(
            origin: OriginFor<T>,
            admin: AccountIdLookupOf<T>,
            config: CollectionConfigFor<T>,
        ) -> DispatchResult {
            let collection = NextCollectionId::<T>::get()
                .or(T::CollectionId::initial_value())
                .ok_or(Error::<T>::UnknownCollection)?;

            let owner = T::CreateOrigin::ensure_origin(origin, &collection)?;
            let admin = T::Lookup::lookup(admin)?;

            // DepositRequired can be disabled by calling the force_create() only
            ensure!(
				!config.has_disabled_setting(CollectionSetting::DepositRequired),
				Error::<T>::WrongSetting
			);

            Self::do_create_collection(
                collection,
                owner.clone(),
                admin.clone(),
                config,
                T::CollectionDeposit::get(),
                Event::Created { collection, creator: owner, owner: admin },
            )?;

            Self::set_next_collection_id(collection);
            Ok(())
        }

        /// Issue a new collection of non-fungible items from a privileged origin.
        ///
        /// This new collection has no items initially.
        ///
        /// The origin must conform to `ForceOrigin`.
        ///
        /// Unlike `create`, no funds are reserved.
        ///
        /// - `owner`: The owner of this collection of items. The owner has full superuser
        ///   permissions over this item, but may later change and configure the permissions using
        ///   `transfer_ownership` and `set_team`.
        ///
        /// Emits `ForceCreated` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::force_create())]
        pub fn force_create(
            origin: OriginFor<T>,
            owner: AccountIdLookupOf<T>,
            config: CollectionConfigFor<T>,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            let owner = T::Lookup::lookup(owner)?;

            let collection = NextCollectionId::<T>::get()
                .or(T::CollectionId::initial_value())
                .ok_or(Error::<T>::UnknownCollection)?;

            Self::do_create_collection(
                collection,
                owner.clone(),
                owner.clone(),
                config,
                Zero::zero(),
                Event::ForceCreated { collection, owner },
            )?;

            Self::set_next_collection_id(collection);
            Ok(())
        }

        /// Destroy a collection of fungible items.
        ///
        /// The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the
        /// owner of the `collection`.
        ///
        /// NOTE: The collection must have 0 items to be destroyed.
        ///
        /// - `collection`: The identifier of the collection to be destroyed.
        /// - `witness`: Information on the items minted in the collection. This must be
        /// correct.
        ///
        /// Emits `Destroyed` event when successful.
        ///
        /// Weight: `O(m + c + a)` where:
        /// - `m = witness.item_metadatas`
        /// - `c = witness.item_configs`
        /// - `a = witness.attributes`
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::destroy(
            witness.item_metadata,
            witness.item_configs,
            witness.attributes,
        ))]
        pub fn destroy(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            witness: DestroyWitness,
        ) -> DispatchResultWithPostInfo {
            let maybe_check_owner = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            let details = Self::do_destroy_collection(collection, witness, maybe_check_owner)?;

            Ok(Some(T::WeightInfo::destroy(
                details.item_metadata,
                details.item_configs,
                details.attributes,
            ))
                .into())
        }

        /// Mint an item of a particular collection.
        ///
        /// The origin must be Signed and the sender must comply with the `mint_settings` rules.
        ///
        /// - `collection`: The collection of the item to be minted.
        /// - `item`: An identifier of the new item.
        /// - `mint_to`: Account into which the item will be minted.
        /// - `witness_data`: When the mint type is `HolderOf(collection_id)`, then the owned
        ///   item_id from that collection needs to be provided within the witness data object.
        ///
        /// Note: the deposit will be taken from the `origin` and not the `owner` of the `item`.
        ///
        /// Emits `Issued` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            mint_to: AccountIdLookupOf<T>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            let mint_to = T::Lookup::lookup(mint_to)?;
            let item_config =
                ItemConfig { settings: Self::get_default_item_settings(&collection)? };

            Self::do_mint(
                collection,
                item,
                Some(caller.clone()),
                mint_to.clone(),
                item_config,
                |_collection_details, collection_config| {
                    let mint_settings = collection_config.mint_settings;
                    let now = frame_system::Pallet::<T>::block_number();

                    if let Some(start_block) = mint_settings.start_block {
                        ensure!(start_block <= now, Error::<T>::MintNotStarted);
                    }
                    if let Some(end_block) = mint_settings.end_block {
                        ensure!(end_block >= now, Error::<T>::MintEnded);
                    }

                    match mint_settings.mint_type {
                        MintType::Issuer => {
                            ensure!(
								Self::has_role(&collection, &caller, CollectionRole::Issuer),
								Error::<T>::NoPermission
							);
                        },
                        _ => {},
                    }

                    Ok(())
                },
            )
        }

        /// Mint an item of a particular collection from a privileged origin.
        ///
        /// The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the
        /// Issuer of the `collection`.
        ///
        /// - `collection`: The collection of the item to be minted.
        /// - `item`: An identifier of the new item.
        /// - `mint_to`: Account into which the item will be minted.
        /// - `item_config`: A config of the new item.
        ///
        /// Emits `Issued` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::force_mint())]
        pub fn force_mint(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            mint_to: AccountIdLookupOf<T>,
            item_config: ItemConfig,
        ) -> DispatchResult {
            let maybe_check_origin = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            let mint_to = T::Lookup::lookup(mint_to)?;

            if let Some(check_origin) = maybe_check_origin {
                ensure!(
					Self::has_role(&collection, &check_origin, CollectionRole::Issuer),
					Error::<T>::NoPermission
				);
            }
            Self::do_mint(collection, item, None, mint_to, item_config, |_, _| Ok(()))
        }

        /// Destroy a single item.
        ///
        /// The origin must conform to `ForceOrigin` or must be Signed and the signing account must
        /// be the owner of the `item`.
        ///
        /// - `collection`: The collection of the item to be burned.
        /// - `item`: The item to be burned.
        ///
        /// Emits `Burned`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::burn())]
        pub fn burn(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
        ) -> DispatchResult {
            let maybe_check_origin = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;

            Self::do_burn(collection, item, |details| {
                if let Some(check_origin) = maybe_check_origin {
                    ensure!(details.owner == check_origin, Error::<T>::NoPermission);
                }
                Ok(())
            })
        }

        /// Move an item from the sender account to another.
        ///
        /// Origin must be Signed and the signing account must be either:
        /// - the Owner of the `item`;
        /// - the approved delegate for the `item` (in this case, the approval is reset).
        ///
        /// Arguments:
        /// - `collection`: The collection of the item to be transferred.
        /// - `item`: The item to be transferred.
        /// - `dest`: The account to receive ownership of the item.
        ///
        /// Emits `Transferred`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            dest: AccountIdLookupOf<T>,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            let dest = T::Lookup::lookup(dest)?;

            Self::do_transfer(collection, item, dest, |_, details| {
                ensure!(details.owner == origin, Error::<T>::NoPermission);
                Ok(())
            })
        }

        /// Re-evaluate the deposits on some items.
        ///
        /// Origin must be Signed and the sender should be the Owner of the `collection`.
        ///
        /// - `collection`: The collection of the items to be reevaluated.
        /// - `items`: The items of the collection whose deposits will be reevaluated.
        ///
        /// NOTE: This exists as a best-effort function. Any items which are unknown or
        /// in the case that the owner account does not have reservable funds to pay for a
        /// deposit increase are ignored. Generally the owner isn't going to call this on items
        /// whose existing deposit is less than the refreshed deposit as it would only cost them,
        /// so it's of little consequence.
        ///
        /// It will still return an error in the case that the collection is unknown or the signer
        /// is not permitted to call it.
        ///
        /// Weight: `O(items.len())`
        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::redeposit(items.len() as u32))]
        pub fn redeposit(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            items: Vec<T::ItemId>,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;

            let collection_details =
                Collection::<T>::get(&collection).ok_or(Error::<T>::UnknownCollection)?;
            ensure!(collection_details.owner == origin, Error::<T>::NoPermission);

            let config = Self::get_collection_config(&collection)?;
            let deposit = match config.is_setting_enabled(CollectionSetting::DepositRequired) {
                true => T::ItemDeposit::get(),
                false => Zero::zero(),
            };

            let mut successful = Vec::with_capacity(items.len());
            for item in items.into_iter() {
                let mut details = match Item::<T>::get(&collection, &item) {
                    Some(x) => x,
                    None => continue,
                };
                let old = details.deposit.amount;
                if old > deposit {
                    T::Currency::unreserve(&details.deposit.account, old - deposit);
                } else if deposit > old {
                    if T::Currency::reserve(&details.deposit.account, deposit - old).is_err() {
                        // NOTE: No alterations made to collection_details in this iteration so far,
                        // so this is OK to do.
                        continue
                    }
                } else {
                    continue
                }
                details.deposit.amount = deposit;
                Item::<T>::insert(&collection, &item, &details);
                successful.push(item);
            }

            Self::deposit_event(Event::<T>::Redeposited {
                collection,
                successful_items: successful,
            });

            Ok(())
        }

        /// Disallow further unprivileged transfer of an item.
        ///
        /// Origin must be Signed and the sender should be the Freezer of the `collection`.
        ///
        /// - `collection`: The collection of the item to be changed.
        /// - `item`: The item to become non-transferable.
        ///
        /// Emits `ItemTransferLocked`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(8)]
        #[pallet::weight(T::WeightInfo::lock_item_transfer())]
        pub fn lock_item_transfer(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            Self::do_lock_item_transfer(origin, collection, item)
        }

        /// Re-allow unprivileged transfer of an item.
        ///
        /// Origin must be Signed and the sender should be the Freezer of the `collection`.
        ///
        /// - `collection`: The collection of the item to be changed.
        /// - `item`: The item to become transferable.
        ///
        /// Emits `ItemTransferUnlocked`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(9)]
        #[pallet::weight(T::WeightInfo::unlock_item_transfer())]
        pub fn unlock_item_transfer(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            Self::do_unlock_item_transfer(origin, collection, item)
        }

        /// Disallows specified settings for the whole collection.
        ///
        /// Origin must be Signed and the sender should be the Owner of the `collection`.
        ///
        /// - `collection`: The collection to be locked.
        /// - `lock_settings`: The settings to be locked.
        ///
        /// Note: it's possible to only lock(set) the setting, but not to unset it.
        ///
        /// Emits `CollectionLocked`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(10)]
        #[pallet::weight(T::WeightInfo::lock_collection())]
        pub fn lock_collection(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            lock_settings: CollectionSettings,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            Self::do_lock_collection(origin, collection, lock_settings)
        }

        /// Change the Owner of a collection.
        ///
        /// Origin must be Signed and the sender should be the Owner of the `collection`.
        ///
        /// - `collection`: The collection whose owner should be changed.
        /// - `owner`: The new Owner of this collection. They must have called
        ///   `set_accept_ownership` with `collection` in order for this operation to succeed.
        ///
        /// Emits `OwnerChanged`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(11)]
        #[pallet::weight(T::WeightInfo::transfer_ownership())]
        pub fn transfer_ownership(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            new_owner: AccountIdLookupOf<T>,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            let new_owner = T::Lookup::lookup(new_owner)?;
            Self::do_transfer_ownership(origin, collection, new_owner)
        }

        /// Change the Issuer, Admin and Freezer of a collection.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
        /// `collection`.
        ///
        /// Note: by setting the role to `None` only the `ForceOrigin` will be able to change it
        /// after to `Some(account)`.
        ///
        /// - `collection`: The collection whose team should be changed.
        /// - `issuer`: The new Issuer of this collection.
        /// - `admin`: The new Admin of this collection.
        /// - `freezer`: The new Freezer of this collection.
        ///
        /// Emits `TeamChanged`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(12)]
        #[pallet::weight(T::WeightInfo::set_team())]
        pub fn set_team(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            issuer: Option<AccountIdLookupOf<T>>,
            admin: Option<AccountIdLookupOf<T>>,
            freezer: Option<AccountIdLookupOf<T>>,
        ) -> DispatchResult {
            let maybe_check_owner = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            let issuer = issuer.map(T::Lookup::lookup).transpose()?;
            let admin = admin.map(T::Lookup::lookup).transpose()?;
            let freezer = freezer.map(T::Lookup::lookup).transpose()?;
            Self::do_set_team(maybe_check_owner, collection, issuer, admin, freezer)
        }

        /// Change the Owner of a collection.
        ///
        /// Origin must be `ForceOrigin`.
        ///
        /// - `collection`: The identifier of the collection.
        /// - `owner`: The new Owner of this collection.
        ///
        /// Emits `OwnerChanged`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(13)]
        #[pallet::weight(T::WeightInfo::force_collection_owner())]
        pub fn force_collection_owner(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            owner: AccountIdLookupOf<T>,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            let new_owner = T::Lookup::lookup(owner)?;
            Self::do_force_collection_owner(collection, new_owner)
        }

        /// Change the config of a collection.
        ///
        /// Origin must be `ForceOrigin`.
        ///
        /// - `collection`: The identifier of the collection.
        /// - `config`: The new config of this collection.
        ///
        /// Emits `CollectionConfigChanged`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(14)]
        #[pallet::weight(T::WeightInfo::force_collection_config())]
        pub fn force_collection_config(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            config: CollectionConfigFor<T>,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            Self::do_force_collection_config(collection, config)
        }

        /// Disallows changing the metadata or attributes of the item.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Admin
        /// of the `collection`.
        ///
        /// - `collection`: The collection if the `item`.
        /// - `item`: An item to be locked.
        /// - `lock_metadata`: Specifies whether the metadata should be locked.
        /// - `lock_attributes`: Specifies whether the attributes in the `CollectionOwner` namespace
        ///   should be locked.
        ///
        /// Note: `lock_attributes` affects the attributes in the `CollectionOwner` namespace only.
        /// When the metadata or attributes are locked, it won't be possible the unlock them.
        ///
        /// Emits `ItemPropertiesLocked`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(18)]
        #[pallet::weight(T::WeightInfo::lock_item_properties())]
        pub fn lock_item_properties(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            lock_metadata: bool,
            lock_attributes: bool,
        ) -> DispatchResult {
            let maybe_check_origin = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            Self::do_lock_item_properties(
                maybe_check_origin,
                collection,
                item,
                lock_metadata,
                lock_attributes,
            )
        }

        /// Set an attribute for a collection or item.
        ///
        /// Origin must be Signed and must conform to the namespace ruleset:
        /// - `CollectionOwner` namespace could be modified by the `collection` Admin only;
        /// - `ItemOwner` namespace could be modified by the `maybe_item` owner only. `maybe_item`
        ///   should be set in that case;
        /// - `Account(AccountId)` namespace could be modified only when the `origin` was given a
        ///   permission to do so;
        ///
        /// The funds of `origin` are reserved according to the formula:
        /// `AttributeDepositBase + DepositPerByte * (key.len + value.len)` taking into
        /// account any already reserved funds.
        ///
        /// - `collection`: The identifier of the collection whose item's metadata to set.
        /// - `maybe_item`: The identifier of the item whose metadata to set.
        /// - `namespace`: Attribute's namespace.
        /// - `key`: The key of the attribute.
        /// - `value`: The value to which to set the attribute.
        ///
        /// Emits `AttributeSet`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(19)]
        #[pallet::weight(T::WeightInfo::set_attribute())]
        pub fn set_attribute(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            maybe_item: Option<T::ItemId>,
            namespace: AttributeNamespace<T::AccountId>,
            key: BoundedVec<u8, T::KeyLimit>,
            value: BoundedVec<u8, T::ValueLimit>,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            let depositor = match namespace {
                AttributeNamespace::CollectionOwner =>
                    Self::collection_owner(collection).ok_or(Error::<T>::UnknownCollection)?,
                _ => origin.clone(),
            };
            Self::do_set_attribute(origin, collection, maybe_item, namespace, key, value, depositor)
        }

        /// Force-set an attribute for a collection or item.
        ///
        /// Origin must be `ForceOrigin`.
        ///
        /// If the attribute already exists and it was set by another account, the deposit
        /// will be returned to the previous owner.
        ///
        /// - `set_as`: An optional owner of the attribute.
        /// - `collection`: The identifier of the collection whose item's metadata to set.
        /// - `maybe_item`: The identifier of the item whose metadata to set.
        /// - `namespace`: Attribute's namespace.
        /// - `key`: The key of the attribute.
        /// - `value`: The value to which to set the attribute.
        ///
        /// Emits `AttributeSet`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(20)]
        #[pallet::weight(T::WeightInfo::force_set_attribute())]
        pub fn force_set_attribute(
            origin: OriginFor<T>,
            set_as: Option<T::AccountId>,
            collection: T::CollectionId,
            maybe_item: Option<T::ItemId>,
            namespace: AttributeNamespace<T::AccountId>,
            key: BoundedVec<u8, T::KeyLimit>,
            value: BoundedVec<u8, T::ValueLimit>,
        ) -> DispatchResult {
            T::ForceOrigin::ensure_origin(origin)?;
            Self::do_force_set_attribute(set_as, collection, maybe_item, namespace, key, value)
        }

        /// Clear an attribute for a collection or item.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
        /// attribute.
        ///
        /// Any deposit is freed for the collection's owner.
        ///
        /// - `collection`: The identifier of the collection whose item's metadata to clear.
        /// - `maybe_item`: The identifier of the item whose metadata to clear.
        /// - `namespace`: Attribute's namespace.
        /// - `key`: The key of the attribute.
        ///
        /// Emits `AttributeCleared`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(21)]
        #[pallet::weight(T::WeightInfo::clear_attribute())]
        pub fn clear_attribute(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            maybe_item: Option<T::ItemId>,
            namespace: AttributeNamespace<T::AccountId>,
            key: BoundedVec<u8, T::KeyLimit>,
        ) -> DispatchResult {
            let maybe_check_owner = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            Self::do_clear_attribute(maybe_check_owner, collection, maybe_item, namespace, key)
        }

        /// Approve item's attributes to be changed by a delegated third-party account.
        ///
        /// Origin must be Signed and must be an owner of the `item`.
        ///
        /// - `collection`: A collection of the item.
        /// - `item`: The item that holds attributes.
        /// - `delegate`: The account to delegate permission to change attributes of the item.
        ///
        /// Emits `ItemAttributesApprovalAdded` on success.
        #[pallet::call_index(22)]
        #[pallet::weight(T::WeightInfo::approve_item_attributes())]
        pub fn approve_item_attributes(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            delegate: AccountIdLookupOf<T>,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            let delegate = T::Lookup::lookup(delegate)?;
            Self::do_approve_item_attributes(origin, collection, item, delegate)
        }

        /// Cancel the previously provided approval to change item's attributes.
        /// All the previously set attributes by the `delegate` will be removed.
        ///
        /// Origin must be Signed and must be an owner of the `item`.
        ///
        /// - `collection`: Collection that the item is contained within.
        /// - `item`: The item that holds attributes.
        /// - `delegate`: The previously approved account to remove.
        ///
        /// Emits `ItemAttributesApprovalRemoved` on success.
        #[pallet::call_index(23)]
        #[pallet::weight(T::WeightInfo::cancel_item_attributes_approval(
        witness.account_attributes
        ))]
        pub fn cancel_item_attributes_approval(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            delegate: AccountIdLookupOf<T>,
            witness: CancelAttributesApprovalWitness,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            let delegate = T::Lookup::lookup(delegate)?;
            Self::do_cancel_item_attributes_approval(origin, collection, item, delegate, witness)
        }

        /// Set the metadata for an item.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Admin of the
        /// `collection`.
        ///
        /// If the origin is Signed, then funds of signer are reserved according to the formula:
        /// `MetadataDepositBase + DepositPerByte * data.len` taking into
        /// account any already reserved funds.
        ///
        /// - `collection`: The identifier of the collection whose item's metadata to set.
        /// - `item`: The identifier of the item whose metadata to set.
        /// - `data`: The general information of this item. Limited in length by `StringLimit`.
        ///
        /// Emits `ItemMetadataSet`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(24)]
        #[pallet::weight(T::WeightInfo::set_metadata())]
        pub fn set_metadata(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            data: BoundedVec<u8, T::StringLimit>,
        ) -> DispatchResult {
            let maybe_check_origin = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            Self::do_set_item_metadata(maybe_check_origin, collection, item, data, None)
        }

        /// Clear the metadata for an item.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Admin of the
        /// `collection`.
        ///
        /// Any deposit is freed for the collection's owner.
        ///
        /// - `collection`: The identifier of the collection whose item's metadata to clear.
        /// - `item`: The identifier of the item whose metadata to clear.
        ///
        /// Emits `ItemMetadataCleared`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(25)]
        #[pallet::weight(T::WeightInfo::clear_metadata())]
        pub fn clear_metadata(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
        ) -> DispatchResult {
            let maybe_check_origin = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            Self::do_clear_item_metadata(maybe_check_origin, collection, item)
        }

        /// Set the metadata for a collection.
        ///
        /// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Admin of
        /// the `collection`.
        ///
        /// If the origin is `Signed`, then funds of signer are reserved according to the formula:
        /// `MetadataDepositBase + DepositPerByte * data.len` taking into
        /// account any already reserved funds.
        ///
        /// - `collection`: The identifier of the item whose metadata to update.
        /// - `data`: The general information of this item. Limited in length by `StringLimit`.
        ///
        /// Emits `CollectionMetadataSet`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(26)]
        #[pallet::weight(T::WeightInfo::set_collection_metadata())]
        pub fn set_collection_metadata(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            data: BoundedVec<u8, T::StringLimit>,
        ) -> DispatchResult {
            let maybe_check_origin = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            Self::do_set_collection_metadata(maybe_check_origin, collection, data)
        }

        /// Clear the metadata for a collection.
        ///
        /// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Admin of
        /// the `collection`.
        ///
        /// Any deposit is freed for the collection's owner.
        ///
        /// - `collection`: The identifier of the collection whose metadata to clear.
        ///
        /// Emits `CollectionMetadataCleared`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(27)]
        #[pallet::weight(T::WeightInfo::clear_collection_metadata())]
        pub fn clear_collection_metadata(
            origin: OriginFor<T>,
            collection: T::CollectionId,
        ) -> DispatchResult {
            let maybe_check_origin = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            Self::do_clear_collection_metadata(maybe_check_origin, collection)
        }

        /// Set (or reset) the acceptance of ownership for a particular account.
        ///
        /// Origin must be `Signed` and if `maybe_collection` is `Some`, then the signer must have a
        /// provider reference.
        ///
        /// - `maybe_collection`: The identifier of the collection whose ownership the signer is
        ///   willing to accept, or if `None`, an indication that the signer is willing to accept no
        ///   ownership transferal.
        ///
        /// Emits `OwnershipAcceptanceChanged`.
        #[pallet::call_index(28)]
        #[pallet::weight(T::WeightInfo::set_accept_ownership())]
        pub fn set_accept_ownership(
            origin: OriginFor<T>,
            maybe_collection: Option<T::CollectionId>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::do_set_accept_ownership(who, maybe_collection)
        }

        /// Set the maximum number of items a collection could have.
        ///
        /// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of
        /// the `collection`.
        ///
        /// - `collection`: The identifier of the collection to change.
        /// - `max_supply`: The maximum number of items a collection could have.
        ///
        /// Emits `CollectionMaxSupplySet` event when successful.
        #[pallet::call_index(29)]
        #[pallet::weight(T::WeightInfo::set_collection_max_supply())]
        pub fn set_collection_max_supply(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            max_supply: u32,
        ) -> DispatchResult {
            let maybe_check_owner = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            Self::do_set_collection_max_supply(maybe_check_owner, collection, max_supply)
        }

        /// Update mint settings.
        ///
        /// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Issuer
        /// of the `collection`.
        ///
        /// - `collection`: The identifier of the collection to change.
        /// - `mint_settings`: The new mint settings.
        ///
        /// Emits `CollectionMintSettingsUpdated` event when successful.
        #[pallet::call_index(30)]
        #[pallet::weight(T::WeightInfo::update_mint_settings())]
        pub fn update_mint_settings(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            mint_settings: MintSettings<BlockNumberFor<T>>,
        ) -> DispatchResult {
            let maybe_check_origin = T::ForceOrigin::try_origin(origin)
                .map(|_| None)
                .or_else(|origin| ensure_signed(origin).map(Some).map_err(DispatchError::from))?;
            Self::do_update_mint_settings(maybe_check_origin, collection, mint_settings)
        }

        /// Mint an item by providing the pre-signed approval.
        ///
        /// Origin must be Signed.
        ///
        /// - `mint_data`: The pre-signed approval that consists of the information about the item,
        ///   its metadata, attributes, who can mint it (`None` for anyone) and until what block
        ///   number.
        /// - `signature`: The signature of the `data` object.
        /// - `signer`: The `data` object's signer. Should be an Issuer of the collection.
        ///
        /// Emits `Issued` on success.
        /// Emits `AttributeSet` if the attributes were provided.
        /// Emits `ItemMetadataSet` if the metadata was not empty.
        #[pallet::call_index(37)]
        #[pallet::weight(T::WeightInfo::mint_pre_signed(mint_data.attributes.len() as u32))]
        pub fn mint_pre_signed(
            origin: OriginFor<T>,
            mint_data: Box<PreSignedMintOf<T>>,
            signature: T::OffchainSignature,
            signer: T::AccountId,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            Self::validate_signature(&Encode::encode(&mint_data), &signature, &signer)?;
            Self::do_mint_pre_signed(origin, *mint_data, signer)
        }

        /// Set attributes for an item by providing the pre-signed approval.
        ///
        /// Origin must be Signed and must be an owner of the `data.item`.
        ///
        /// - `data`: The pre-signed approval that consists of the information about the item,
        ///   attributes to update and until what block number.
        /// - `signature`: The signature of the `data` object.
        /// - `signer`: The `data` object's signer. Should be an Admin of the collection for the
        ///   `CollectionOwner` namespace.
        ///
        /// Emits `AttributeSet` for each provided attribute.
        /// Emits `ItemAttributesApprovalAdded` if the approval wasn't set before.
        /// Emits `PreSignedAttributesSet` on success.
        #[pallet::call_index(38)]
        #[pallet::weight(T::WeightInfo::set_attributes_pre_signed(data.attributes.len() as u32))]
        pub fn set_attributes_pre_signed(
            origin: OriginFor<T>,
            data: PreSignedAttributesOf<T>,
            signature: T::OffchainSignature,
            signer: T::AccountId,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            Self::validate_signature(&Encode::encode(&data), &signature, &signer)?;
            Self::do_set_attributes_pre_signed(origin, data, signer)
        }
    }
}
