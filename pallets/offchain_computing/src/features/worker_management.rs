use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub(crate) fn do_authorize_worker(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		worker: T::AccountId
	) -> DispatchResult {
		ensure!(
			!PoolAuthorizedWorkers::<T>::contains_key(&worker, &pool_info.id),
			Error::<T>::WorkerAlreadyAdded
		);

		let worker_info = T::OffchainWorkerManageable::worker_info(&worker).ok_or(Error::<T>::WorkerNotFound)?;
		ensure!(
			worker_info.impl_id == pool_info.impl_id.clone(),
			Error::<T>::ImplMismatched
		);

		PoolAuthorizedWorkers::<T>::insert(&worker, &pool_info.id, ());

		let mut new_pool_info = pool_info.clone();
		new_pool_info.workers_count += 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::WorkerAdded { pool_id: pool_info.id, worker: worker.clone() });
		Ok(())
	}

	pub(crate) fn do_revoke_worker(
		pool_info: PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
		worker: T::AccountId,
	) -> DispatchResult {
		ensure!(
			PoolAuthorizedWorkers::<T>::contains_key(&worker, &pool_info.id),
			Error::<T>::WorkerNotFound
		);

		if WorkerSubscribedPools::<T>::contains_key(&worker, &pool_info.id) {
			WorkerSubscribedPools::<T>::remove(&worker, &pool_info.id);

			Self::deposit_event(Event::WorkerUnsubscribed { worker: worker.clone(), pool_id: pool_info.id.clone() });
		}

		PoolAuthorizedWorkers::<T>::remove(&worker, &pool_info.id);

		let mut new_pool_info = pool_info.clone();
		new_pool_info.workers_count -= 1;
		Pools::<T>::insert(&pool_info.id, new_pool_info);

		Self::deposit_event(Event::WorkerRemoved { pool_id: pool_info.id, worker });
		Ok(())
	}

	pub(crate) fn do_subscribe_pool(
		worker: T::AccountId,
		pool_id: T::PoolId
	) -> DispatchResult {
		ensure!(
			PoolAuthorizedWorkers::<T>::contains_key(&worker, &pool_id),
			Error::<T>::WorkerNotInThePool
		);
		ensure!(
			!WorkerSubscribedPools::<T>::contains_key(&worker, &pool_id),
			Error::<T>::WorkerAlreadySubscribed
		);

		let subscribed_pools_count = CounterForWorkerSubscribedPools::<T>::get(&worker);
		ensure!(
			subscribed_pools_count < T::MaxSubscribedPoolsPerWorker::get(),
			Error::<T>::WorkerSubscribedPoolsLimitExceeded
		);

		CounterForWorkerSubscribedPools::<T>::insert(&worker, subscribed_pools_count + 1);
		WorkerSubscribedPools::<T>::insert(&worker, &pool_id, ());

		Self::deposit_event(Event::WorkerSubscribed { worker, pool_id });
		Ok(())
	}

	pub(crate) fn do_unsubscribe_pool(
		worker: T::AccountId,
		pool_id: T::PoolId
	) -> DispatchResult {
		ensure!(
			WorkerSubscribedPools::<T>::contains_key(&worker, &pool_id),
			Error::<T>::WorkerNotSubscribeThePool
		);

		WorkerSubscribedPools::<T>::remove(&worker, &pool_id);

		Self::deposit_event(Event::WorkerUnsubscribed { worker, pool_id });
		Ok(())
	}
}
