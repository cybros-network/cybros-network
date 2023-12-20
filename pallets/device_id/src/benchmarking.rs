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

//! Nfts pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use enumflags2::{BitFlag, BitFlags};
use frame_benchmarking::v1::{
	account, benchmarks_instance_pallet, whitelist_account, whitelisted_caller, BenchmarkError,
};
use frame_support::{
	assert_ok,
	traits::{EnsureOrigin, Get, UnfilteredDispatchable},
	BoundedVec,
};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin as SystemOrigin};
use sp_io::crypto::{sr25519_generate, sr25519_sign};
use sp_runtime::{
	traits::{Bounded, IdentifyAccount, One},
	AccountId32, MultiSignature, MultiSigner,
};
use sp_std::prelude::*;

use crate::Pallet as ThePallet;

const SEED: u32 = 0;

fn create_collection<T: Config>(
) -> (T::CollectionId, T::AccountId, AccountIdLookupOf<T>) {
	let caller: T::AccountId = whitelisted_caller();
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let collection = T::Helper::collection(0);
	T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
	assert_ok!(ThePallet::<T>::force_create(
		SystemOrigin::Root.into(),
		caller_lookup.clone(),
		default_collection_config::<T>()
	));
	(collection, caller, caller_lookup)
}

fn add_collection_metadata<T: Config>() -> (T::AccountId, AccountIdLookupOf<T>) {
	let caller = Collection::<T>::get(T::Helper::collection(0)).unwrap().owner;
	if caller != whitelisted_caller() {
		whitelist_account!(caller);
	}
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	assert_ok!(ThePallet::<T>::set_collection_metadata(
		SystemOrigin::Signed(caller.clone()).into(),
		T::Helper::collection(0),
		vec![0; T::StringLimit::get() as usize].try_into().unwrap(),
	));
	(caller, caller_lookup)
}

fn mint_item<T: Config>(
	index: u16,
) -> (T::ItemId, T::AccountId, AccountIdLookupOf<T>) {
	let item = T::Helper::item(index);
	let collection = T::Helper::collection(0);
	let caller = Collection::<T>::get(collection).unwrap().owner;
	if caller != whitelisted_caller() {
		whitelist_account!(caller);
	}
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let item_exists = Item::<T>::contains_key(&collection, &item);
	let item_config = ItemConfigOf::<T>::get(&collection, &item);
	if item_exists {
		return (item, caller, caller_lookup)
	} else if let Some(item_config) = item_config {
		assert_ok!(DeviceId::<T>::force_mint(
			SystemOrigin::Signed(caller.clone()).into(),
			collection,
			item,
			caller_lookup.clone(),
			item_config,
		));
	} else {
		assert_ok!(DeviceId::<T>::mint(
			SystemOrigin::Signed(caller.clone()).into(),
			collection,
			item,
			caller_lookup.clone(),
			None,
		));
	}
	(item, caller, caller_lookup)
}

fn lock_item<T: Config>(
	index: u16,
) -> (T::ItemId, T::AccountId, AccountIdLookupOf<T>) {
	let caller = Collection::<T>::get(T::Helper::collection(0)).unwrap().owner;
	if caller != whitelisted_caller() {
		whitelist_account!(caller);
	}
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let item = T::Helper::item(index);
	assert_ok!(DeviceId::<T>::lock_item_transfer(
		SystemOrigin::Signed(caller.clone()).into(),
		T::Helper::collection(0),
		item,
	));
	(item, caller, caller_lookup)
}

fn burn_item<T: Config>(
	index: u16,
) -> (T::ItemId, T::AccountId, AccountIdLookupOf<T>) {
	let caller = Collection::<T>::get(T::Helper::collection(0)).unwrap().owner;
	if caller != whitelisted_caller() {
		whitelist_account!(caller);
	}
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let item = T::Helper::item(index);
	assert_ok!(DeviceId::<T>::burn(
		SystemOrigin::Signed(caller.clone()).into(),
		T::Helper::collection(0),
		item,
	));
	(item, caller, caller_lookup)
}

fn add_item_metadata<T: Config>(
	item: T::ItemId,
) -> (T::AccountId, AccountIdLookupOf<T>) {
	let caller = Collection::<T>::get(T::Helper::collection(0)).unwrap().owner;
	if caller != whitelisted_caller() {
		whitelist_account!(caller);
	}
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	assert_ok!(DeviceId::<T>::set_metadata(
		SystemOrigin::Signed(caller.clone()).into(),
		T::Helper::collection(0),
		item,
		vec![0; T::StringLimit::get() as usize].try_into().unwrap(),
	));
	(caller, caller_lookup)
}

fn add_item_attribute<T: Config>(
	item: T::ItemId,
) -> (BoundedVec<u8, T::KeyLimit>, T::AccountId, AccountIdLookupOf<T>) {
	let caller = Collection::<T>::get(T::Helper::collection(0)).unwrap().owner;
	if caller != whitelisted_caller() {
		whitelist_account!(caller);
	}
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let key: BoundedVec<_, _> = vec![0; T::KeyLimit::get() as usize].try_into().unwrap();
	assert_ok!(DeviceId::<T>::set_attribute(
		SystemOrigin::Signed(caller.clone()).into(),
		T::Helper::collection(0),
		Some(item),
		AttributeNamespace::CollectionOwner,
		key.clone(),
		vec![0; T::ValueLimit::get() as usize].try_into().unwrap(),
	));
	(key, caller, caller_lookup)
}

fn add_collection_attribute<T: Config>(
	i: u16,
) -> (BoundedVec<u8, T::KeyLimit>, T::AccountId, AccountIdLookupOf<T>) {
	let caller = Collection::<T>::get(T::Helper::collection(0)).unwrap().owner;
	if caller != whitelisted_caller() {
		whitelist_account!(caller);
	}
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let key: BoundedVec<_, _> = make_filled_vec(i, T::KeyLimit::get() as usize).try_into().unwrap();
	assert_ok!(DeviceId::<T>::set_attribute(
		SystemOrigin::Signed(caller.clone()).into(),
		T::Helper::collection(0),
		None,
		AttributeNamespace::CollectionOwner,
		key.clone(),
		vec![0; T::ValueLimit::get() as usize].try_into().unwrap(),
	));
	(key, caller, caller_lookup)
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
	// compare to the last event record
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

fn make_collection_config<T: Config>(
	disable_settings: BitFlags<CollectionSetting>,
) -> CollectionConfigFor<T> {
	CollectionConfig {
		settings: CollectionSettings::from_disabled(disable_settings),
		max_supply: None,
		mint_settings: MintSettings::default(),
	}
}

fn default_collection_config<T: Config>() -> CollectionConfigFor<T> {
	make_collection_config::<T>(CollectionSetting::empty())
}

fn default_item_config() -> ItemConfig {
	ItemConfig { settings: ItemSettings::all_enabled() }
}

fn make_filled_vec(value: u16, length: usize) -> Vec<u8> {
	let mut vec = vec![0u8; length];
	let mut s = Vec::from(value.to_be_bytes());
	vec.truncate(length - s.len());
	vec.append(&mut s);
	vec
}

benchmarks_instance_pallet! {
	where_clause {
		where
			T::OffchainSignature: From<MultiSignature>,
			T::AccountId: From<AccountId32>,
	}

	create {
		let collection = T::Helper::collection(0);
		let origin = T::CreateOrigin::try_successful_origin(&collection)
			.map_err(|_| BenchmarkError::Weightless)?;
		let caller = T::CreateOrigin::ensure_origin(origin.clone(), &collection).unwrap();
		whitelist_account!(caller);
		let admin = T::Lookup::unlookup(caller.clone());
		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
		let call = Call::<T>::create { admin, config: default_collection_config::<T>() };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::NextCollectionIdIncremented { next_id: Some(T::Helper::collection(1)) }.into());
	}

	force_create {
		let caller: T::AccountId = whitelisted_caller();
		let caller_lookup = T::Lookup::unlookup(caller.clone());
	}: _(SystemOrigin::Root, caller_lookup, default_collection_config::<T>())
	verify {
		assert_last_event::<T>(Event::NextCollectionIdIncremented { next_id: Some(T::Helper::collection(1)) }.into());
	}

	destroy {
		let m in 0 .. 1_000;
		let c in 0 .. 1_000;
		let a in 0 .. 1_000;

		let (collection, caller, _) = create_collection::<T>();
		add_collection_metadata::<T>();
		for i in 0..m {
			mint_item::<T>(i as u16);
			add_item_metadata::<T>(T::Helper::item(i as u16));
			lock_item::<T>(i as u16);
			burn_item::<T>(i as u16);
		}
		for i in 0..c {
			mint_item::<T>(i as u16);
			lock_item::<T>(i as u16);
			burn_item::<T>(i as u16);
		}
		for i in 0..a {
			add_collection_attribute::<T>(i as u16);
		}
		let witness = Collection::<T>::get(collection).unwrap().destroy_witness();
	}: _(SystemOrigin::Signed(caller), collection, witness)
	verify {
		assert_last_event::<T>(Event::Destroyed { collection }.into());
	}

	mint {
		let (collection, caller, caller_lookup) = create_collection::<T>();
		let item = T::Helper::item(0);
	}: _(SystemOrigin::Signed(caller.clone()), collection, item, caller_lookup, None)
	verify {
		assert_last_event::<T>(Event::Issued { collection, item, owner: caller }.into());
	}

	force_mint {
		let (collection, caller, caller_lookup) = create_collection::<T>();
		let item = T::Helper::item(0);
	}: _(SystemOrigin::Signed(caller.clone()), collection, item, caller_lookup, default_item_config())
	verify {
		assert_last_event::<T>(Event::Issued { collection, item, owner: caller }.into());
	}

	burn {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
	}: _(SystemOrigin::Signed(caller.clone()), collection, item)
	verify {
		assert_last_event::<T>(Event::Burned { collection, item, owner: caller }.into());
	}

	transfer {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);

		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
		T::Currency::make_free_balance_be(&target, T::Currency::minimum_balance());
	}: _(SystemOrigin::Signed(caller.clone()), collection, item, target_lookup)
	verify {
		assert_last_event::<T>(Event::Transferred { collection, item, from: caller, to: target }.into());
	}

	redeposit {
		let i in 0 .. 5_000;
		let (collection, caller, _) = create_collection::<T>();
		let items = (0..i).map(|x| mint_item::<T>(x as u16).0).collect::<Vec<_>>();
		DeviceId::<T>::force_collection_config(
			SystemOrigin::Root.into(),
			collection,
			make_collection_config::<T>(CollectionSetting::DepositRequired.into()),
		)?;
	}: _(SystemOrigin::Signed(caller.clone()), collection, items.clone())
	verify {
		assert_last_event::<T>(Event::Redeposited { collection, successful_items: items }.into());
	}

	lock_item_transfer {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
	}: _(SystemOrigin::Signed(caller.clone()), T::Helper::collection(0), T::Helper::item(0))
	verify {
		assert_last_event::<T>(Event::ItemTransferLocked { collection: T::Helper::collection(0), item: T::Helper::item(0) }.into());
	}

	unlock_item_transfer {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
		DeviceId::<T>::lock_item_transfer(
			SystemOrigin::Signed(caller.clone()).into(),
			collection,
			item,
		)?;
	}: _(SystemOrigin::Signed(caller.clone()), collection, item)
	verify {
		assert_last_event::<T>(Event::ItemTransferUnlocked { collection, item }.into());
	}

	lock_collection {
		let (collection, caller, _) = create_collection::<T>();
		let lock_settings = CollectionSettings::from_disabled(
			CollectionSetting::TransferableItems |
				CollectionSetting::UnlockedMetadata |
				CollectionSetting::UnlockedAttributes |
				CollectionSetting::UnlockedMaxSupply,
		);
	}: _(SystemOrigin::Signed(caller.clone()), collection, lock_settings)
	verify {
		assert_last_event::<T>(Event::CollectionLocked { collection }.into());
	}

	transfer_ownership {
		let (collection, caller, _) = create_collection::<T>();
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
		T::Currency::make_free_balance_be(&target, T::Currency::minimum_balance());
		let origin = SystemOrigin::Signed(target.clone()).into();
		DeviceId::<T>::set_accept_ownership(origin, Some(collection))?;
	}: _(SystemOrigin::Signed(caller), collection, target_lookup)
	verify {
		assert_last_event::<T>(Event::OwnerChanged { collection, new_owner: target }.into());
	}

	set_team {
		let (collection, caller, _) = create_collection::<T>();
		let target0 = Some(T::Lookup::unlookup(account("target", 0, SEED)));
		let target1 = Some(T::Lookup::unlookup(account("target", 1, SEED)));
		let target2 = Some(T::Lookup::unlookup(account("target", 2, SEED)));
	}: _(SystemOrigin::Signed(caller), collection, target0, target1, target2)
	verify {
		assert_last_event::<T>(Event::TeamChanged{
			collection,
			issuer: Some(account("target", 0, SEED)),
			admin: Some(account("target", 1, SEED)),
			freezer: Some(account("target", 2, SEED)),
		}.into());
	}

	force_collection_owner {
		let (collection, _, _) = create_collection::<T>();
		let origin =
			T::ForceOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
		T::Currency::make_free_balance_be(&target, T::Currency::minimum_balance());
		let call = Call::<T>::force_collection_owner {
			collection,
			owner: target_lookup,
		};
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::OwnerChanged { collection, new_owner: target }.into());
	}

	force_collection_config {
		let (collection, caller, _) = create_collection::<T>();
		let origin =
			T::ForceOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let call = Call::<T>::force_collection_config {
			collection,
			config: make_collection_config::<T>(CollectionSetting::DepositRequired.into()),
		};
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::CollectionConfigChanged { collection }.into());
	}

	lock_item_properties {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
		let lock_metadata = true;
		let lock_attributes = true;
	}: _(SystemOrigin::Signed(caller), collection, item, lock_metadata, lock_attributes)
	verify {
		assert_last_event::<T>(Event::ItemPropertiesLocked { collection, item, lock_metadata, lock_attributes }.into());
	}

	set_attribute {
		let key: BoundedVec<_, _> = vec![0u8; T::KeyLimit::get() as usize].try_into().unwrap();
		let value: BoundedVec<_, _> = vec![0u8; T::ValueLimit::get() as usize].try_into().unwrap();

		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
	}: _(SystemOrigin::Signed(caller), collection, Some(item), AttributeNamespace::CollectionOwner, key.clone(), value.clone())
	verify {
		assert_last_event::<T>(
			Event::AttributeSet {
				collection,
				maybe_item: Some(item),
				namespace: AttributeNamespace::CollectionOwner,
				key,
				value,
			}
			.into(),
		);
	}

	force_set_attribute {
		let key: BoundedVec<_, _> = vec![0u8; T::KeyLimit::get() as usize].try_into().unwrap();
		let value: BoundedVec<_, _> = vec![0u8; T::ValueLimit::get() as usize].try_into().unwrap();

		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
	}: _(SystemOrigin::Root, Some(caller), collection, Some(item), AttributeNamespace::CollectionOwner, key.clone(), value.clone())
	verify {
		assert_last_event::<T>(
			Event::AttributeSet {
				collection,
				maybe_item: Some(item),
				namespace: AttributeNamespace::CollectionOwner,
				key,
				value,
			}
			.into(),
		);
	}

	clear_attribute {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
		add_item_metadata::<T>(item);
		let (key, ..) = add_item_attribute::<T>(item);
	}: _(SystemOrigin::Signed(caller), collection, Some(item), AttributeNamespace::CollectionOwner, key.clone())
	verify {
		assert_last_event::<T>(
			Event::AttributeCleared {
				collection,
				maybe_item: Some(item),
				namespace: AttributeNamespace::CollectionOwner,
				key,
			}.into(),
		);
	}

	approve_item_attributes {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
	}: _(SystemOrigin::Signed(caller), collection, item, target_lookup)
	verify {
		assert_last_event::<T>(
			Event::ItemAttributesApprovalAdded {
				collection,
				item,
				delegate: target,
			}
			.into(),
		);
	}

	cancel_item_attributes_approval {
		let n in 0 .. 1_000;

		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
		DeviceId::<T>::approve_item_attributes(
			SystemOrigin::Signed(caller.clone()).into(),
			collection,
			item,
			target_lookup.clone(),
		)?;
		T::Currency::make_free_balance_be(&target, DepositBalanceOf::<T>::max_value());
		let value: BoundedVec<_, _> = vec![0u8; T::ValueLimit::get() as usize].try_into().unwrap();
		for i in 0..n {
			let key = make_filled_vec(i as u16, T::KeyLimit::get() as usize);
			DeviceId::<T>::set_attribute(
				SystemOrigin::Signed(target.clone()).into(),
				T::Helper::collection(0),
				Some(item),
				AttributeNamespace::Account(target.clone()),
				key.try_into().unwrap(),
				value.clone(),
			)?;
		}
		let witness = CancelAttributesApprovalWitness { account_attributes: n };
	}: _(SystemOrigin::Signed(caller), collection, item, target_lookup, witness)
	verify {
		assert_last_event::<T>(
			Event::ItemAttributesApprovalRemoved {
				collection,
				item,
				delegate: target,
			}
			.into(),
		);
	}

	set_metadata {
		let data: BoundedVec<_, _> = vec![0u8; T::StringLimit::get() as usize].try_into().unwrap();

		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
	}: _(SystemOrigin::Signed(caller), collection, item, data.clone())
	verify {
		assert_last_event::<T>(Event::ItemMetadataSet { collection, item, data }.into());
	}

	clear_metadata {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
		add_item_metadata::<T>(item);
	}: _(SystemOrigin::Signed(caller), collection, item)
	verify {
		assert_last_event::<T>(Event::ItemMetadataCleared { collection, item }.into());
	}

	set_collection_metadata {
		let data: BoundedVec<_, _> = vec![0u8; T::StringLimit::get() as usize].try_into().unwrap();

		let (collection, caller, _) = create_collection::<T>();
	}: _(SystemOrigin::Signed(caller), collection, data.clone())
	verify {
		assert_last_event::<T>(Event::CollectionMetadataSet { collection, data }.into());
	}

	clear_collection_metadata {
		let (collection, caller, _) = create_collection::<T>();
		add_collection_metadata::<T>();
	}: _(SystemOrigin::Signed(caller), collection)
	verify {
		assert_last_event::<T>(Event::CollectionMetadataCleared { collection }.into());
	}

	approve_transfer {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
		let delegate: T::AccountId = account("delegate", 0, SEED);
		let delegate_lookup = T::Lookup::unlookup(delegate.clone());
		let deadline = BlockNumberFor::<T>::max_value();
	}: _(SystemOrigin::Signed(caller.clone()), collection, item, delegate_lookup, Some(deadline))
	verify {
		assert_last_event::<T>(Event::TransferApproved { collection, item, owner: caller, delegate, deadline: Some(deadline) }.into());
	}

	cancel_approval {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
		let delegate: T::AccountId = account("delegate", 0, SEED);
		let delegate_lookup = T::Lookup::unlookup(delegate.clone());
		let origin = SystemOrigin::Signed(caller.clone()).into();
		let deadline = BlockNumberFor::<T>::max_value();
		DeviceId::<T>::approve_transfer(origin, collection, item, delegate_lookup.clone(), Some(deadline))?;
	}: _(SystemOrigin::Signed(caller.clone()), collection, item, delegate_lookup)
	verify {
		assert_last_event::<T>(Event::ApprovalCancelled { collection, item, owner: caller, delegate }.into());
	}

	clear_all_transfer_approvals {
		let (collection, caller, _) = create_collection::<T>();
		let (item, ..) = mint_item::<T>(0);
		let delegate: T::AccountId = account("delegate", 0, SEED);
		let delegate_lookup = T::Lookup::unlookup(delegate.clone());
		let origin = SystemOrigin::Signed(caller.clone()).into();
		let deadline = BlockNumberFor::<T>::max_value();
		DeviceId::<T>::approve_transfer(origin, collection, item, delegate_lookup.clone(), Some(deadline))?;
	}: _(SystemOrigin::Signed(caller.clone()), collection, item)
	verify {
		assert_last_event::<T>(Event::AllApprovalsCancelled {collection, item, owner: caller}.into());
	}

	set_accept_ownership {
		let caller: T::AccountId = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
		let collection = T::Helper::collection(0);
	}: _(SystemOrigin::Signed(caller.clone()), Some(collection))
	verify {
		assert_last_event::<T>(Event::OwnershipAcceptanceChanged {
			who: caller,
			maybe_collection: Some(collection),
		}.into());
	}

	set_collection_max_supply {
		let (collection, caller, _) = create_collection::<T>();
	}: _(SystemOrigin::Signed(caller.clone()), collection, u32::MAX)
	verify {
		assert_last_event::<T>(Event::CollectionMaxSupplySet {
			collection,
			max_supply: u32::MAX,
		}.into());
	}

	update_mint_settings {
		let (collection, caller, _) = create_collection::<T>();
		let mint_settings = MintSettings {
			mint_type: MintType::HolderOf(T::Helper::collection(0)),
			start_block: Some(One::one()),
			end_block: Some(One::one()),
			price: Some(ItemPrice::<T>::from(1u32)),
			default_item_settings: ItemSettings::all_enabled(),
		};
	}: _(SystemOrigin::Signed(caller.clone()), collection, mint_settings)
	verify {
		assert_last_event::<T>(Event::CollectionMintSettingsUpdated { collection }.into());
	}

	mint_pre_signed {
		let n in 0 .. T::MaxAttributesPerCall::get() as u32;
		let caller_public = sr25519_generate(0.into(), None);
		let caller = MultiSigner::Sr25519(caller_public).into_account().into();
		T::Currency::make_free_balance_be(&caller, DepositBalanceOf::<T>::max_value());
		let caller_lookup = T::Lookup::unlookup(caller.clone());

		let collection = T::Helper::collection(0);
		let item = T::Helper::item(0);
		assert_ok!(DeviceId::<T>::force_create(
			SystemOrigin::Root.into(),
			caller_lookup.clone(),
			default_collection_config::<T>()
		));

		let metadata = vec![0u8; T::StringLimit::get() as usize];
		let mut attributes = vec![];
		let attribute_value = vec![0u8; T::ValueLimit::get() as usize];
		for i in 0..n {
			let attribute_key = make_filled_vec(i as u16, T::KeyLimit::get() as usize);
			attributes.push((attribute_key, attribute_value.clone()));
		}
		let mint_data = PreSignedMint {
			collection,
			item,
			attributes,
			metadata: metadata.clone(),
			only_account: None,
			deadline: One::one(),
			mint_price: Some(DepositBalanceOf::<T>::min_value()),
		};
		let message = Encode::encode(&mint_data);
		let signature = MultiSignature::Sr25519(sr25519_sign(0.into(), &caller_public, &message).unwrap());

		let target: T::AccountId = account("target", 0, SEED);
		T::Currency::make_free_balance_be(&target, DepositBalanceOf::<T>::max_value());
		frame_system::Pallet::<T>::set_block_number(One::one());
	}: _(SystemOrigin::Signed(target.clone()), Box::new(mint_data), signature.into(), caller)
	verify {
		let metadata: BoundedVec<_, _> = metadata.try_into().unwrap();
		assert_last_event::<T>(Event::ItemMetadataSet { collection, item, data: metadata }.into());
	}

	set_attributes_pre_signed {
		let n in 0 .. T::MaxAttributesPerCall::get() as u32;
		let (collection, _, _) = create_collection::<T>();

		let item_owner: T::AccountId = account("item_owner", 0, SEED);
		let item_owner_lookup = T::Lookup::unlookup(item_owner.clone());

		let signer_public = sr25519_generate(0.into(), None);
		let signer: T::AccountId = MultiSigner::Sr25519(signer_public).into_account().into();

		T::Currency::make_free_balance_be(&item_owner, DepositBalanceOf::<T>::max_value());

		let item = T::Helper::item(0);
		assert_ok!(DeviceId::<T>::force_mint(
			SystemOrigin::Root.into(),
			collection,
			item,
			item_owner_lookup.clone(),
			default_item_config(),
		));

		let mut attributes = vec![];
		let attribute_value = vec![0u8; T::ValueLimit::get() as usize];
		for i in 0..n {
			let attribute_key = make_filled_vec(i as u16, T::KeyLimit::get() as usize);
			attributes.push((attribute_key, attribute_value.clone()));
		}
		let pre_signed_data = PreSignedAttributes {
			collection,
			item,
			attributes,
			namespace: AttributeNamespace::Account(signer.clone()),
			deadline: One::one(),
		};
		let message = Encode::encode(&pre_signed_data);
		let signature = MultiSignature::Sr25519(sr25519_sign(0.into(), &signer_public, &message).unwrap());

		frame_system::Pallet::<T>::set_block_number(One::one());
	}: _(SystemOrigin::Signed(item_owner.clone()), pre_signed_data, signature.into(), signer.clone())
	verify {
		assert_last_event::<T>(
			Event::PreSignedAttributesSet {
				collection,
				item,
				namespace: AttributeNamespace::Account(signer.clone()),
			}
			.into(),
		);
	}

	impl_benchmark_test_suite!(Nfts, crate::mock::new_test_ext(), crate::mock::Test);
}
