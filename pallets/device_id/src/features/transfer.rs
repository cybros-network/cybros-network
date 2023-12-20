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

//! This module contains helper methods to perform the transfer functionalities
//! of the NFTs pallet.

use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	/// Transfer an NFT to the specified destination account.
	///
	/// - `collection`: The ID of the collection to which the NFT belongs.
	/// - `item`: The ID of the NFT to transfer.
	/// - `dest`: The destination account to which the NFT will be transferred.
	/// - `with_details`: A closure that provides access to the collection and item details,
	///   allowing customization of the transfer process.
	///
	/// This function performs the actual transfer of an NFT to the destination account.
	/// It checks various conditions like item lock status and transferability settings
	/// for the collection and item before transferring the NFT.
	///
	/// # Errors
	///
	/// This function returns a dispatch error in the following cases:
	/// - If the collection ID is invalid ([`UnknownCollection`](crate::Error::UnknownCollection)).
	/// - If the item ID is invalid ([`UnknownItem`](crate::Error::UnknownItem)).
	/// - If the item is locked or transferring it is disabled
	///   ([`ItemLocked`](crate::Error::ItemLocked)).
	/// - If the collection or item is non-transferable
	///   ([`ItemsNonTransferable`](crate::Error::ItemsNonTransferable)).
	pub fn do_transfer(
		collection: T::ProductId,
		item: T::DeviceId,
		dest: T::AccountId,
		with_details: impl FnOnce(
			&ProductEntryFor<T>,
			&mut DeviceEntryFor<T>,
		) -> DispatchResult,
	) -> DispatchResult {
		// Retrieve collection details.
		let collection_details =
			Collection::<T>::get(&collection).ok_or(Error::<T>::UnknownCollection)?;

		// Ensure the item is not locked.
		ensure!(!T::Locker::is_locked(collection, item), Error::<T>::ItemLocked);

		// Ensure the item is not transfer disabled on the system level attribute.
		ensure!(
			!Self::has_system_attribute(&collection, &item, PalletAttributes::TransferDisabled)?,
			Error::<T>::ItemLocked
		);

		// Retrieve collection config and check if items are transferable.
		let collection_config = Self::get_collection_config(&collection)?;
		ensure!(
			collection_config.is_setting_enabled(ProductSetting::TransferableItems),
			Error::<T>::ItemsNonTransferable
		);

		// Retrieve item config and check if the item is transferable.
		let item_config = Self::get_item_config(&collection, &item)?;
		ensure!(
			item_config.is_setting_enabled(ItemSetting::Transferable),
			Error::<T>::ItemLocked
		);

		// Retrieve the item details.
		let mut details =
			Item::<T>::get(&collection, &item).ok_or(Error::<T>::UnknownItem)?;

		// Perform the transfer with custom details using the provided closure.
		with_details(&collection_details, &mut details)?;

		// Update account ownership information.
		Account::<T>::remove((&details.owner, &collection, &item));
		Account::<T>::insert((&dest, &collection, &item), ());
		let origin = details.owner;
		details.owner = dest;

		// Update item details.
		Item::<T>::insert(&collection, &item, &details);

		// Emit `Transferred` event.
		Self::deposit_event(Event::Transferred {
			collection,
			item,
			from: origin,
			to: details.owner,
		});
		Ok(())
	}

	/// Transfer ownership of a collection to another account.
	///
	/// - `origin`: The account requesting the transfer.
	/// - `collection`: The ID of the collection to transfer ownership.
	/// - `owner`: The new account that will become the owner of the collection.
	///
	/// This function transfers the ownership of a collection to the specified account.
	/// It performs checks to ensure that the `origin` is the current owner and that the
	/// new owner is an acceptable account based on the collection's acceptance settings.
	pub(crate) fn do_transfer_ownership(
		origin: T::AccountId,
		collection: T::ProductId,
		new_owner: T::AccountId,
	) -> DispatchResult {
		// Check if the new owner is acceptable based on the collection's acceptance settings.
		let acceptable_collection = OwnershipAcceptance::<T>::get(&new_owner);
		ensure!(acceptable_collection.as_ref() == Some(&collection), Error::<T>::Unaccepted);

		// Try to retrieve and mutate the collection details.
		Collection::<T>::try_mutate(collection, |maybe_details| {
			let details = maybe_details.as_mut().ok_or(Error::<T>::UnknownCollection)?;
			// Check if the `origin` is the current owner of the collection.
			ensure!(origin == details.owner, Error::<T>::NoPermission);
			if details.owner == new_owner {
				return Ok(())
			}

			// Move the deposit to the new owner.
			T::Currency::repatriate_reserved(
				&details.owner,
				&new_owner,
				details.owner_deposit,
				Reserved,
			)?;

			// Update account ownership information.
			CollectionAccount::<T>::remove(&details.owner, &collection);
			CollectionAccount::<T>::insert(&new_owner, &collection, ());

			details.owner = new_owner.clone();
			OwnershipAcceptance::<T>::remove(&new_owner);
			frame_system::Pallet::<T>::dec_consumers(&new_owner);

			// Emit `OwnerChanged` event.
			Self::deposit_event(Event::OwnerChanged { collection, new_owner });
			Ok(())
		})
	}
	/// Set or unset the ownership acceptance for an account regarding a specific collection.
	///
	/// - `who`: The account for which to set or unset the ownership acceptance.
	/// - `maybe_collection`: An optional collection ID to set the ownership acceptance.
	///
	/// If `maybe_collection` is `Some(collection)`, then the account `who` will accept
	/// ownership transfers for the specified collection. If `maybe_collection` is `None`,
	/// then the account `who` will unset the ownership acceptance, effectively refusing
	/// ownership transfers for any collection.
	pub(crate) fn do_set_accept_ownership(
		who: T::AccountId,
		maybe_collection: Option<T::ProductId>,
	) -> DispatchResult {
		let exists = OwnershipAcceptance::<T>::contains_key(&who);
		match (exists, maybe_collection.is_some()) {
			(false, true) => {
				frame_system::Pallet::<T>::inc_consumers(&who)?;
			},
			(true, false) => {
				frame_system::Pallet::<T>::dec_consumers(&who);
			},
			_ => {},
		}
		if let Some(collection) = maybe_collection.as_ref() {
			OwnershipAcceptance::<T>::insert(&who, collection);
		} else {
			OwnershipAcceptance::<T>::remove(&who);
		}

		// Emit `OwnershipAcceptanceChanged` event.
		Self::deposit_event(Event::OwnershipAcceptanceChanged { who, maybe_collection });
		Ok(())
	}

	/// Forcefully change the owner of a collection.
	///
	/// - `collection`: The ID of the collection to change ownership.
	/// - `owner`: The new account that will become the owner of the collection.
	///
	/// This function allows for changing the ownership of a collection without any checks.
	/// It moves the deposit to the new owner, updates the collection's owner, and emits
	/// an `OwnerChanged` event.
	pub(crate) fn do_force_collection_owner(
		collection: T::ProductId,
		owner: T::AccountId,
	) -> DispatchResult {
		// Try to retrieve and mutate the collection details.
		Collection::<T>::try_mutate(collection, |maybe_details| {
			let details = maybe_details.as_mut().ok_or(Error::<T>::UnknownCollection)?;
			if details.owner == owner {
				return Ok(())
			}

			// Move the deposit to the new owner.
			T::Currency::repatriate_reserved(
				&details.owner,
				&owner,
				details.owner_deposit,
				Reserved,
			)?;

			// Update collection accounts and set the new owner.
			CollectionAccount::<T>::remove(&details.owner, &collection);
			CollectionAccount::<T>::insert(&owner, &collection, ());
			details.owner = owner.clone();

			// Emit `OwnerChanged` event.
			Self::deposit_event(Event::OwnerChanged { collection, new_owner: owner });
			Ok(())
		})
	}
}
