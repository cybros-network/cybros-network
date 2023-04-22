#![cfg_attr(not(feature = "std"), no_std)]

mod macros;
mod traits;
mod features;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
pub use primitives::*;

/// The log target of this pallet.
pub const LOG_TARGET: &str = "runtime::offchain_computing";

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
	traits::{Currency, ReservableCurrency, UnixTime},
	transactional,
};
use sp_runtime::{
	traits::{
		AtLeast32BitUnsigned, StaticLookup,
		// IdentifyAccount, Verify,
	},
	SaturatedConversion,
};

use pallet_offchain_computing_workers::{
	OfflineReason, OnlinePayload, VerifiedAttestation,
	OffchainWorkerLifecycleHooks, OffchainWorkerManageable,
	ImplSpecVersion
};

pub(crate) type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
pub(crate) type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub(crate) type ImplIdOf<T> =
	<<T as Config>::OffchainWorkerManageable as OffchainWorkerManageable<<T as frame_system::Config>::AccountId>>::ImplId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::{fmt::Display, prelude::*};
	use traits::AutoIncrement;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent> + TryInto<Event<Self>>;

		type OffchainWorkerManageable: OffchainWorkerManageable<Self::AccountId>;

		type Currency: ReservableCurrency<Self::AccountId>;

		type UnixTime: UnixTime;

		/// Identifier for the tasks' pool.
		type PoolId: Member + Parameter + MaxEncodedLen + Display + AtLeast32BitUnsigned + AutoIncrement;

		/// The type used to identify a task within a pool.
		type TaskId: Member + Parameter + MaxEncodedLen + Display + AtLeast32BitUnsigned + AutoIncrement;

		/// The type used to identify a policy within a pool.
		type PolicyId: Member + Parameter + MaxEncodedLen + Display + AtLeast32BitUnsigned + AutoIncrement;

		/// Standard collection creation is only allowed if the origin attempting it and the
		/// collection are in this set.
		type CreatePoolOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;

		/// The basic amount of funds that must be reserved for pool.
		#[pallet::constant]
		type CreatePoolDeposit: Get<BalanceOf<Self>>;

		/// The basic amount of funds that must be reserved for a task.
		#[pallet::constant]
		type CreatingTaskDeposit: Get<BalanceOf<Self>>;

		/// The basic amount of funds that must be reserved when adding metadata to your item.
		#[pallet::constant]
		type MetadataDepositBase: Get<BalanceOf<Self>>;

		/// The additional funds that must be reserved for the number of bytes store in input and output.
		#[pallet::constant]
		type DepositPerByte: Get<BalanceOf<Self>>;

		/// The limit of tasks a worker could be assigned
		#[pallet::constant]
		type MaxAssignedTasksPerWorker: Get<u32>;

		/// The limit of policies of a pool
		#[pallet::constant]
		type MaxPoliciesPerPool: Get<u32>;

		/// The limit of tasks in a pool
		#[pallet::constant]
		type MaxTasksPerPool: Get<u32>;

		/// The limit of workers in a pool
		#[pallet::constant]
		type MaxWorkersPerPool: Get<u32>;

		/// The min `expires_in` can be set
		#[pallet::constant]
		type MinTaskExpiresIn: Get<u64>;

		/// The max `expires_in` can be set
		#[pallet::constant]
		type MaxTaskExpiresIn: Get<u64>;

		/// The default `expires_in` if not given
		#[pallet::constant]
		type DefaultTaskExpiresIn: Get<u64>;

		/// The maximum length of pool's metadata stored on-chain.
		#[pallet::constant]
		type PoolMetadataLimit: Get<u32>;

		/// The maximum length of input stored on-chain.
		#[pallet::constant]
		type InputLimit: Get<u32>;

		/// The maximum length of processed output stored on-chain.
		#[pallet::constant]
		type OutputLimit: Get<u32>;

		/// The maximum length of proof stored on-chain.
		#[pallet::constant]
		type ProofLimit: Get<u32>;

		// TODO: Support create task by off-chain pre-sign message
		// /// Off-Chain signature type.
		// ///
		// /// Can verify whether an `Self::OffchainPublic` created a signature.
		// type OffchainSignature: Verify<Signer = Self::OffchainPublic> + Parameter;
		//
		// /// Off-Chain public key.
		// ///
		// /// Must identify as an on-chain `Self::AccountId`.
		// type OffchainPublic: IdentifyAccount<AccountId = Self::AccountId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PoolCreated { owner: T::AccountId, pool_id: T::PoolId, impl_id: ImplIdOf<T> },
		PoolDestroyed { pool_id: T::PoolId },
		PoolMetadataUpdated { pool_id: T::PoolId, metadata: BoundedVec<u8, T::PoolMetadataLimit> },
		PoolMetadataRemoved { pool_id: T::PoolId },
		PoolCreatingTaskAbilityEnabled { pool_id: T::PoolId },
		PoolCreatingTaskAbilityDisabled { pool_id: T::PoolId },
		CreatingTaskPolicyCreated { pool_id: T::PoolId, policy_id: T::PolicyId, policy: CreatingTaskPolicy<T::BlockNumber> },
		CreatingTaskPolicyDestroyed { pool_id: T::PoolId, policy_id: T::PolicyId },
		WorkerAdded { pool_id: T::PoolId, worker: T::AccountId },
		WorkerRemoved { pool_id: T::PoolId, worker: T::AccountId },
		TaskCreated {
			pool_id: T::PoolId,
			task_id: T::TaskId,
			policy_id: T::PolicyId,
			owner: T::AccountId,
			impl_spec_version: ImplSpecVersion,
			input: Option<BoundedVec<u8, T::InputLimit>>,
			expires_in: u64
		},
		TaskDestroyed { pool_id: T::PoolId, task_id: T::TaskId, destroyer: T::AccountId },
		TaskAssigned { pool_id: T::PoolId, task_id: T::TaskId, assignee: T::AccountId },
		TaskReleased { pool_id: T::PoolId, task_id: T::TaskId },
		TaskStatusUpdated { pool_id: T::PoolId, task_id: T::TaskId, status: TaskStatus },
		TaskResultUpdated { pool_id: T::PoolId, task_id: T::TaskId, result: TaskResult, output: Option<BoundedVec<u8, T::OutputLimit>>, proof: Option<BoundedVec<u8, T::ProofLimit>> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		InternalError,
		PoolIdTaken,
		PolicyIdTaken,
		TaskIdTaken,
		PoolNotEmpty,
		NoPermission,
		WorkerNotFound,
		WorkerNotInThePool,
		PoolNotFound,
		PoolCreatingTaskAbilityDisabled,
		CreatingTaskPoliciesPerPoolLimitExceeded,
		PolicyNotApplicable,
		PolicyNotFound,
		ExpiresInTooSmall,
		ExpiresInTooLarge,
		WorkerAlreadyAdded,
		ImplMismatched,
		ImplNotFound,
		TasksPerPoolLimitExceeded,
		TaskNotFound,
		WorkerAssignedTasksLimitExceeded,
		NoAssignableTask,
		TaskIsProcessing,
		TaskIsProcessed,
		TaskAssigneeLocked,
		TaskStillValid,
		TaskExpired,
		TaskAlreadyAssigned,
	}

	/// Pools info.
	#[pallet::storage]
	pub type Pools<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>,
	>;

	/// Metadata of a pool.
	#[pallet::storage]
	pub type PoolMetadata<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		ChainStoredData<T::AccountId, BalanceOf<T>, T::PoolMetadataLimit>,
		OptionQuery,
	>;

	/// Tasks info.
	#[pallet::storage]
	pub type CreatingTaskPolicies<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::PolicyId,
		CreatingTaskPolicy<T::BlockNumber>,
		OptionQuery,
	>;

	/// Workers of pools
	#[pallet::storage]
	pub type Workers<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AccountId,
		(),
		OptionQuery,
	>;

	/// Tasks info.
	#[pallet::storage]
	pub type Tasks<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::TaskId,
		TaskInfo<T::TaskId, T::AccountId, BalanceOf<T>, ImplSpecVersion>,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type TaskInputs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::TaskId,
		ChainStoredData<T::AccountId, BalanceOf<T>, T::InputLimit>,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type TaskOutputs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::TaskId,
		ChainStoredData<T::AccountId, BalanceOf<T>, T::OutputLimit>,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type TaskProofs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::TaskId,
		ChainStoredData<T::AccountId, BalanceOf<T>, T::ProofLimit>,
		OptionQuery,
	>;

	/// Stores the `PoolId` that is going to be used for the next pool.
	/// This gets incremented whenever a new pool is created.
	#[pallet::storage]
	pub type NextPoolId<T: Config> = StorageValue<_, T::PoolId, OptionQuery>;

	/// Stores the `TaskId` that is going to be used for the next task.
	/// This gets incremented whenever a new task is created.
	#[pallet::storage]
	pub type NextCreatingTaskPolicyId<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		T::PolicyId,
		OptionQuery,
	>;

	/// Stores the `TaskId` that is going to be used for the next task.
	/// This gets incremented whenever a new task is created.
	#[pallet::storage]
	pub type NextTaskId<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		T::TaskId,
		OptionQuery,
	>;

	/// The pools owned by any given account; set out this way so that pools owned by
	/// a single account can be enumerated.
	#[pallet::storage]
	pub type AccountOwningPools<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::PoolId,
		(),
		OptionQuery,
	>;

	#[pallet::storage]
	pub type WorkerServingPools<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::PoolId,
		(),
		OptionQuery,
	>;

	/// The tasks held by any given account; set out this way so that tasks owned by a single
	/// account can be enumerated.
	#[pallet::storage]
	pub type AccountOwningTasks<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>, // owner
			NMapKey<Blake2_128Concat, T::PoolId>,
			NMapKey<Blake2_128Concat, T::TaskId>,
		),
		(),
		OptionQuery,
	>;

	#[pallet::storage]
	pub type WorkerAssignedTasks<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>, // owner
			NMapKey<Blake2_128Concat, T::PoolId>,
			NMapKey<Blake2_128Concat, T::TaskId>,
		),
		(),
		OptionQuery,
	>;

	#[pallet::storage]
	pub type AssignableTasks<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::PoolId>,
			NMapKey<Blake2_128Concat, ImplSpecVersion>,
			NMapKey<Blake2_128Concat, T::TaskId>,
		),
		(),
		OptionQuery,
	>;

	#[pallet::storage]
	pub type WorkerAssignedTasksCounter<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		u32,
		ValueQuery
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[transactional]
		#[pallet::call_index(0)]
		#[pallet::weight({0})]
		pub fn create_pool(
			origin: OriginFor<T>,
			impl_id: ImplIdOf<T>
		) -> DispatchResult {
			let owner = T::CreatePoolOrigin::ensure_origin(origin)?;
			let pool_id = NextPoolId::<T>::get().unwrap_or(T::PoolId::initial_value());

			Self::do_create_pool(
				owner,
				pool_id.clone(),
				impl_id
			)?;

			let next_id = pool_id.increment();
			NextPoolId::<T>::set(Some(next_id));

			Ok(())
		}

		#[transactional]
		#[pallet::call_index(1)]
		#[pallet::weight({0})]
		pub fn destroy_pool(
			origin: OriginFor<T>,
			pool_id: T::PoolId
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_destroy_pool(who, pool_id)
		}

		#[transactional]
		#[pallet::call_index(2)]
		#[pallet::weight({0})]
		pub fn update_pool_metadata(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			metadata: Option<BoundedVec<u8, T::PoolMetadataLimit>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
			Self::ensure_pool_owner(&who, &pool_info)?;

			if let Some(metadata) = metadata {
				Self::do_update_pool_metadata(pool_info, metadata)
			} else {
				Self::do_remove_pool_metadata(pool_info)
			}
		}

		#[transactional]
		#[pallet::call_index(3)]
		#[pallet::weight({0})]
		pub fn toggle_pool_global_creating_task_ability(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			enabled: bool
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
			Self::ensure_pool_owner(&who, &pool_info)?;

			Self::do_toggle_pool_creating_task_ability(pool_info, enabled)
		}

		#[transactional]
		#[pallet::call_index(4)]
		#[pallet::weight({0})]
		pub fn create_creating_task_policy(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			policy: CreatingTaskPolicy<T::BlockNumber>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
			Self::ensure_pool_owner(&who, &pool_info)?;

			ensure!(
				pool_info.creating_task_policies_count.clone() <= T::MaxPoliciesPerPool::get(),
				Error::<T>::CreatingTaskPoliciesPerPoolLimitExceeded
			);

			let policy_id = NextCreatingTaskPolicyId::<T>::get(&pool_id).unwrap_or(T::PolicyId::initial_value());
			Self::do_create_creating_task_policy(
				pool_info,
				policy_id.clone(),
				policy
			)?;

			let next_id = policy_id.increment();
			NextCreatingTaskPolicyId::<T>::set(&pool_id, Some(next_id));

			Ok(())
		}

		#[transactional]
		#[pallet::call_index(5)]
		#[pallet::weight({0})]
		pub fn destroy_creating_task_policy(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			policy_id: T::PolicyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
			Self::ensure_pool_owner(&who, &pool_info)?;

			Self::do_destroy_creating_task_policy(
				pool_info,
				policy_id
			)
		}

		#[transactional]
		#[pallet::call_index(6)]
		#[pallet::weight({0})]
		pub fn add_worker(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			worker: AccountIdLookupOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
			Self::ensure_pool_owner(&who, &pool_info)?;

			let worker = T::Lookup::lookup(worker.clone())?;

			Self::do_add_worker(
				pool_info,
				worker
			)
		}

		#[transactional]
		#[pallet::call_index(7)]
		#[pallet::weight({0})]
		pub fn remove_worker(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			worker: AccountIdLookupOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let worker = T::Lookup::lookup(worker)?;

			let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let worker_info = T::OffchainWorkerManageable::worker_info(&worker).ok_or(Error::<T>::WorkerNotFound)?;

			let pool_owner = pool_info.owner.clone();
			let worker_owner = worker_info.owner;
			ensure!(
				pool_owner == who || worker_owner == who,
				Error::<T>::NoPermission
			);

			Self::do_remove_worker(
				pool_info,
				worker
			)
		}

		#[transactional]
		#[pallet::call_index(8)]
		#[pallet::weight({0})]
		pub fn create_task(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			policy_id: T::PolicyId,
			impl_spec_version: ImplSpecVersion,
			input: Option<BoundedVec<u8, T::InputLimit>>,
			soft_expires_in: Option<u64>,
			// TODO: Tips?
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let pool_info = Pools::<T>::get(&pool_id).ok_or(Error::<T>::PoolNotFound)?;
			ensure!(
				pool_info.creating_task_ability,
				Error::<T>::PoolCreatingTaskAbilityDisabled
			);
			ensure!(
				pool_info.tasks_count <= T::MaxTasksPerPool::get(),
				Error::<T>::TasksPerPoolLimitExceeded
			);

			let policy = CreatingTaskPolicies::<T>::get(&pool_id, &policy_id).ok_or(Error::<T>::PolicyNotFound)?;
			let current_block = frame_system::Pallet::<T>::block_number();
			if let Some(start_block) = policy.start_block {
				ensure!(current_block >= start_block , Error::<T>::PolicyNotApplicable);
			}
			if let Some(end_block) = policy.end_block {
				ensure!(current_block <= end_block, Error::<T>::PolicyNotApplicable);
			}
			match policy.permission {
				CreatingTaskPermission::Owner => {
					ensure!(
						&pool_info.owner == &who,
						Error::<T>::PolicyNotApplicable
					)
				},
				CreatingTaskPermission::Public => {}
			};

			let task_id = NextTaskId::<T>::get(&pool_id).unwrap_or(T::TaskId::initial_value());
			let now = T::UnixTime::now().as_secs().saturated_into::<u64>();
			Self::do_create_task(
				pool_info,
				task_id.clone(),
				policy_id.clone(),
				who.clone(),
				who,
				impl_spec_version,
				input,
				now,
				soft_expires_in
			)?;

			let next_id = task_id.increment();
			NextTaskId::<T>::set(&pool_id, Some(next_id));

			Ok(())
		}

		#[transactional]
		#[pallet::call_index(9)]
		#[pallet::weight({0})]
		pub fn destroy_task(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			task_id: T::TaskId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_destroy_task(
				who,
				pool_id,
				task_id
			)
		}

		#[transactional]
		#[pallet::call_index(10)]
		#[pallet::weight({0})]
		pub fn destroy_expired_task(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			task_id: T::TaskId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let now = T::UnixTime::now().as_secs().saturated_into::<u64>();
			Self::do_destroy_expired_task(
				pool_id,
				task_id,
				who,
				now
			)
		}

		#[transactional]
		#[pallet::call_index(11)]
		#[pallet::weight({0})]
		pub fn take_task(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			task_id: Option<T::TaskId>,
			processing: bool,
			soft_expires_in: Option<u64>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let now = T::UnixTime::now().as_secs().saturated_into::<u64>();
			let expires_in = soft_expires_in.unwrap_or(T::DefaultTaskExpiresIn::get());
			Self::do_assign_task(pool_id, task_id, who, now, processing, expires_in)
		}

		#[transactional]
		#[pallet::call_index(12)]
		#[pallet::weight({0})]
		pub fn release_task(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			task_id: T::TaskId
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_release_task(pool_id, task_id, who)
		}

		#[transactional]
		#[pallet::call_index(13)]
		#[pallet::weight({0})]
		pub fn submit_task_result(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			task_id: T::TaskId,
			result: TaskResult,
			output: Option<BoundedVec<u8, T::OutputLimit>>,
			proof: Option<BoundedVec<u8, T::ProofLimit>>,
			soft_expires_in: Option<u64>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let now = T::UnixTime::now().as_secs().saturated_into::<u64>();
			let expires_in = soft_expires_in.unwrap_or(T::DefaultTaskExpiresIn::get());
			Self::do_submit_task_result(pool_id, task_id, who, result, output, proof, now, expires_in)
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn ensure_pool_owner(
			who: &T::AccountId,
			pool_info: &PoolInfo<T::PoolId, T::AccountId, BalanceOf<T>, ImplIdOf<T>>
		) -> DispatchResult {
			ensure!(
				who == &pool_info.owner,
				Error::<T>::NoPermission
			);

			Ok(())
		}

		pub(crate) fn ensure_task_owner(
			who: &T::AccountId,
			task: &TaskInfo<T::TaskId, T::AccountId, BalanceOf<T>, ImplSpecVersion>
		) -> DispatchResult {
			ensure!(
				who == &task.owner,
				Error::<T>::NoPermission
			);

			Ok(())
		}

		pub(crate) fn ensure_task_expired(
			task: &TaskInfo<T::TaskId, T::AccountId, BalanceOf<T>, ImplSpecVersion>,
			now: u64
		) -> DispatchResult {
			// TODO: need deadline
			ensure!(
				task.expires_at < now,
				Error::<T>::TaskStillValid
			);

			Ok(())
		}

		// Comment this because of difficulty reason,
		// we decide to let the `expires_at` be soft expiring
		// pub(crate) fn ensure_task_not_expired(
		// 	task: &TaskInfo<T::TaskId, T::AccountId, BalanceOf<T>, ImplSpecVersion>,
		// 	now: u64
		// ) -> DispatchResult {
		// 	ensure!(
		// 		task.expires_at >= now,
		// 		Error::<T>::TaskExpired
		// 	);
		//
		// 	Ok(())
		// }

		pub(crate) fn ensure_worker_in_pool(
			pool_id: &T::PoolId,
			worker: &T::AccountId,
		) -> DispatchResult {
			ensure!(
				Workers::<T>::contains_key(pool_id, worker),
				Error::<T>::WorkerNotInThePool
			);

			Ok(())
		}

		pub(crate) fn ensure_task_assignee(
			task: &TaskInfo<T::TaskId, T::AccountId, BalanceOf<T>, ImplSpecVersion>,
			worker: &T::AccountId,
		) -> DispatchResult {
			let assignee = task.assignee.clone().ok_or(Error::<T>::NoPermission)?;

			ensure!(
				&assignee == worker,
				Error::<T>::NoPermission
			);

			Ok(())
		}
	}

	impl<T: Config> OffchainWorkerLifecycleHooks<T::AccountId, ImplIdOf<T>> for Pallet<T> {
		fn can_online(_worker: &T::AccountId, _payload: &OnlinePayload<ImplIdOf<T>>, _verified_attestation: &VerifiedAttestation) -> DispatchResult {
			Ok(())
		}

		fn after_online(_worker: &T::AccountId) {
			// Nothing to do
		}

		fn can_offline(worker: &T::AccountId) -> bool {
			WorkerAssignedTasksCounter::<T>::get(worker) == 0
		}

		fn after_unresponsive(_worker: &T::AccountId) {
			// Nothing to do
		}

		fn before_offline(worker: &T::AccountId, _reason: OfflineReason) {
			// TODO: Efficient and security (avoid a worker in too much pools)
			for pool_id in WorkerServingPools::<T>::iter_key_prefix(worker) {
				for task_id in WorkerAssignedTasks::<T>::iter_key_prefix((worker.clone(), pool_id.clone())) {
					let _: Result<(), DispatchError> = Tasks::<T>::try_mutate_exists(&pool_id, &task_id, |task| -> Result<(), DispatchError> {
						if let Some(mut task) = task.as_mut() {
							task.status = TaskStatus::Discarded;
							task.ended_at = Some(T::UnixTime::now().as_secs().saturated_into::<u64>());

							Self::deposit_event(Event::TaskStatusUpdated { pool_id: pool_id.clone(), task_id: task_id.clone(), status: TaskStatus::Discarded });
						}

						Ok(())
					});
				}
			}

			let _: Result<(), DispatchError> = WorkerAssignedTasksCounter::<T>::try_mutate(worker, |counter| -> Result<(), DispatchError> {
				*counter = 0;
				Ok(())
			});
		}

		fn after_refresh_attestation(_worker: &T::AccountId, _payload: &OnlinePayload<ImplIdOf<T>>, _verified_attestation: &VerifiedAttestation) {
			// Nothing to do
		}

		fn after_requesting_offline(_worker: &T::AccountId) {
			// Nothing to do
		}

		fn can_deregister(worker: &T::AccountId) -> bool {
			WorkerServingPools::<T>::iter_key_prefix(worker).next().is_none()
		}

		fn before_deregister(_worker: &T::AccountId) {
			// Nothing to do
		}
	}
}
