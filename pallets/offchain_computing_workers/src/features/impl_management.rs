use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub fn do_register_impl(
		impl_id: T::ImplId,
		owner: T::AccountId,
		attestation_method: AttestationMethod,
		deployment_permission: ImplDeploymentPermission,
	) -> DispatchResult {
		ensure!(!Impls::<T>::contains_key(&impl_id), Error::<T>::ImplIdTaken);

		let deposit = T::RegisterImplDeposit::get();
		T::Currency::reserve(&owner, deposit)?;

		let impl_info = ImplInfo::<T::ImplId, T::AccountId, BalanceOf<T>> {
			id: impl_id.clone(),
			owner: owner.clone(),
			owner_deposit: deposit,
			attestation_method: attestation_method.clone(),
			deployment_permission: deployment_permission.clone(),
			build_restriction: Default::default(),
			workers_count: 0,
		};

		Impls::<T>::insert(impl_id, impl_info);
		AccountOwningImpls::<T>::insert(&owner, &impl_id, ());

		Self::deposit_event(
			Event::ImplRegistered {
				owner, attestation_method, impl_id, deployment_permission
			}
		);
		Ok(())
	}

	pub fn do_deregister_impl(
		who: T::AccountId,
		impl_id: T::ImplId
	) -> DispatchResult {
		let impl_info = Impls::<T>::get(&impl_id).ok_or(Error::<T>::ImplNotFound)?;
		Self::ensure_impl_owner(&who, &impl_info)?;
		ensure!(impl_info.workers_count == 0, Error::<T>::ImplStillInUse);

		if let Some(metadata_entry) = ImplMetadata::<T>::take(&impl_id) {
			T::Currency::unreserve(&metadata_entry.depositor, metadata_entry.actual_deposit);
		}

		let _ = RegisteredImplBuildMagicBytes::<T>::clear_prefix(&impl_id, T::MaxRegisteredImplBuildMagicBytes::get(), None);
		RegisteredImplBuildMagicBytesCounter::<T>::remove(&&impl_info.id);

		Impls::<T>::remove(&impl_id);
		AccountOwningImpls::<T>::remove(&impl_info.owner, &impl_id);

		T::Currency::unreserve(&impl_info.owner, impl_info.owner_deposit);

		Self::deposit_event(Event::ImplDeregistered { impl_id });
		Ok(())
	}

	pub(crate) fn do_update_impl_metadata(
		impl_info: ImplInfo<T::ImplId, T::AccountId, BalanceOf<T>>,
		new_metadata: BoundedVec<u8, T::ImplMetadataLimit>
	) -> DispatchResult {
		ImplMetadata::<T>::try_mutate_exists(impl_info.id, |metadata_entry| {
			let deposit = T::DepositPerByte::get()
				.saturating_mul(((new_metadata.len()) as u32).into())
				.saturating_add(T::ImplMetadataDepositBase::get());

			let old_deposit = metadata_entry.take().map_or(Zero::zero(), |m| m.actual_deposit);
			if deposit > old_deposit {
				T::Currency::reserve(&impl_info.owner, deposit - old_deposit)?;
			} else if deposit < old_deposit {
				T::Currency::unreserve(&impl_info.owner, old_deposit - deposit);
			}

			*metadata_entry = Some(ChainStoredData {
				depositor: impl_info.owner.clone(),
				actual_deposit: deposit,
				surplus_deposit: Zero::zero(),
				data: new_metadata.clone()
			});

			Self::deposit_event(
				Event::ImplMetadataUpdated { impl_id: impl_info.id, metadata: new_metadata.clone() }
			);
			Ok(())
		})
	}

	pub(crate) fn do_remove_impl_metadata(
		impl_info: ImplInfo<T::ImplId, T::AccountId, BalanceOf<T>>,
	) -> DispatchResult {
		let Some(metadata_entry) = ImplMetadata::<T>::get(&impl_info.id) else {
			return Ok(())
		};

		ImplMetadata::<T>::remove(&impl_info.id);

		T::Currency::unreserve(&impl_info.owner, metadata_entry.actual_deposit);

		Self::deposit_event(Event::ImplMetadataRemoved { impl_id: impl_info.id });
		Ok(())
	}

	pub(crate) fn do_update_impl_version_restriction(
		mut impl_info: ImplInfo<T::ImplId, T::AccountId, BalanceOf<T>>,
		restriction: ImplBuildRestriction,
	) -> DispatchResult {
		let impl_id = impl_info.id;

		impl_info.build_restriction = restriction.clone();
		Impls::<T>::insert(&impl_id, impl_info);

		Self::deposit_event(Event::<T>::ImplBuildRestrictionUpdated { impl_id, restriction });
		Ok(())
	}

	pub(crate) fn do_register_impl_build_magic_bytes(
		impl_info: ImplInfo<T::ImplId, T::AccountId, BalanceOf<T>>,
		build_version: ImplBuildVersion,
		build_magic_bytes: ImplBuildMagicBytes
	) -> DispatchResult {
		let impl_id = impl_info.id;
		ensure!(
			!RegisteredImplBuildMagicBytes::<T>::contains_key(&impl_id, &build_version),
			Error::<T>::ImplBuildMagicBytesAlreadyRegistered
		);

		RegisteredImplBuildMagicBytesCounter::<T>::try_mutate(&impl_id, |counter| -> Result<(), DispatchError> {
			ensure!(
				counter <= &mut T::MaxRegisteredImplBuildMagicBytes::get(),
				Error::<T>::ImplBuildMagicBytesLimitExceeded
			);

			*counter += 1;
			Ok(())
		})?;
		RegisteredImplBuildMagicBytes::<T>::insert(&impl_id, &build_version, build_magic_bytes.clone());

		Self::deposit_event(Event::<T>::ImplBuildMagicBytesRegistered { impl_id, version: build_version, magic_bytes: build_magic_bytes });

		Ok(())
	}

	pub(crate) fn do_deregister_impl_build_magic_bytes(
		impl_info: ImplInfo<T::ImplId, T::AccountId, BalanceOf<T>>,
		build_version: ImplBuildVersion,
	) -> DispatchResult {
		let impl_id = impl_info.id;
		ensure!(
			!RegisteredImplBuildMagicBytes::<T>::contains_key(&impl_id, &build_version),
			Error::<T>::ImplBuildMagicBytesAlreadyRegistered
		);

		RegisteredImplBuildMagicBytes::<T>::remove(&impl_id, &build_version);
		RegisteredImplBuildMagicBytesCounter::<T>::try_mutate(&impl_id, |counter| -> Result<(), DispatchError> {
			*counter -= 1;
			Ok(())
		})?;

		Self::deposit_event(Event::<T>::ImplBuildMagicBytesDeregistered { impl_id, version: build_version });

		Ok(())
	}
}
