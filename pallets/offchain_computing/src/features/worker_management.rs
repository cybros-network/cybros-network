use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub(crate) fn do_add_worker(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		worker: T::AccountId
	) -> DispatchResult {
		ensure!(
			!Workers::<T>::contains_key(&pool_info.id, &worker),
			Error::<T>::WorkerAlreadyAdded
		);

		let worker_info = T::OffchainWorkerManageable::worker_info(&worker).ok_or(Error::<T>::WorkerNotFound)?;
		ensure!(
			worker_info.impl_id == pool_info.impl_id.clone(),
			Error::<T>::ImplMismatched
		);

		Workers::<T>::insert(&pool_info.id, &worker, ());
		WorkerServingPools::<T>::insert(&worker, &pool_info.id, ());

		let mut new_pool_info = pool_info.clone();
		new_pool_info.workers_count += 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::WorkerAdded { pool_id: pool_info.id, worker: worker.clone() });
		Ok(())
	}

	pub(crate) fn do_remove_worker(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		worker: T::AccountId,
	) -> DispatchResult {
		ensure!(
			Workers::<T>::contains_key(&pool_info.id, &worker),
			Error::<T>::WorkerNotFound
		);

		Workers::<T>::remove(&pool_info.id, &worker);
		WorkerServingPools::<T>::remove(&worker, &pool_info.id);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.workers_count -= 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::WorkerRemoved { pool_id: pool_info.id, worker });
		Ok(())
	}
}
