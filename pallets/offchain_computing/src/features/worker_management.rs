use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub fn do_add_worker(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		worker: &T::AccountId,
	) -> DispatchResult {
		ensure!(
			!Workers::<T>::contains_key(&pool_info.id, &worker),
			Error::<T>::WorkerAlreadyAdded
		);
		Self::ensure_worker(worker)?;

		Workers::<T>::insert(&pool_info.id, worker, ());

		let mut new_pool_info = pool_info.clone();
		new_pool_info.workers_count += 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Ok(())
	}

	pub fn do_remove_worker(
		pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>>,
		worker: &T::AccountId,
	) -> DispatchResult {
		ensure!(
			Workers::<T>::contains_key(&pool_info.id, &worker),
			Error::<T>::WorkerNotFound
		);

		Workers::<T>::remove(&pool_info.id, worker);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.workers_count -= 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Ok(())
	}
}