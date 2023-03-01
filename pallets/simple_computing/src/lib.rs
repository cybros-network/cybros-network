#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod macros;

pub mod traits;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The log target of this pallet.
pub const LOG_TARGET: &str = "runtime::simple_computing";

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
	sp_runtime::Saturating,
	traits::{Currency, ReservableCurrency},
};
use pallet_computing_workers::{
	traits::{WorkerLifecycleHooks, WorkerManageable},
	primitives::{OfflineReason, OnlinePayload, VerifiedAttestation},
};
use crate::traits::*;
use primitives::*;

pub(crate) type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type Job<T> = primitives::Job<
	<T as frame_system::Config>::AccountId, BalanceOf<T>, <T as frame_system::Config>::BlockNumber,
	<T as Config>::JobId,
	<T as Config>::MaxJobCommandLen, <T as Config>::MaxJobInputLen, <T as Config>::MaxJobOutputLen
>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The system's currency for payment.
		type Currency: ReservableCurrency<Self::AccountId>;

		type WorkerManageable: WorkerManageable<Self::AccountId, Self::BlockNumber>;

		type JobId: Member + Parameter + MaxEncodedLen + Copy + AutoIncrement;

		#[pallet::constant]
		type JobDepositBase: Get<BalanceOf<Self>>;

		#[pallet::constant]
		type JobInputDepositPerByte: Get<BalanceOf<Self>>;

		#[pallet::constant]
		type MinJobRunningDurationLen: Get<u32>;

		#[pallet::constant]
		type MaxJobCommandLen: Get<u32>;

		#[pallet::constant]
		type MaxJobInputLen: Get<u32>;

		#[pallet::constant]
		type MaxJobOutputLen: Get<u32>;
	}

	#[pallet::storage]
	pub(crate) type AssignedJobs<T: Config> = StorageMap<_, Identity, T::AccountId, Job<T>>;

	#[pallet::storage]
	pub(crate) type CompletedJobs<T: Config> = StorageDoubleMap<
		_,
		Identity,
		T::AccountId,
		Identity,
		T::JobId,
		Job<T>
	>;

	#[pallet::storage]
	pub(super) type NextJobId<T: Config> = StorageMap<
		_,
		Identity,
		T::AccountId,
		T::JobId,
		OptionQuery
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		JobCreated { worker: T::AccountId },
		JobStarted { worker: T::AccountId, deadline: Option<T::BlockNumber> },
		JobCompleted { worker: T::AccountId, result: JobResult },
		JobReclaimed { worker: T::AccountId, job_id: T::JobId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		InsufficientFundsForReserving,
		NoPermission,
		NotTheOwner,
		WorkerNotExists,
		JobNotExists,
		AlreadyAssigned,
		AlreadyStarted,
		MaxRunningDurationTooShort,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_job(
			origin: OriginFor<T>,
			worker: T::AccountId,
			command: BoundedVec<u8, T::MaxJobCommandLen>,
			input: BoundedVec<u8, T::MaxJobInputLen>,
			max_running_duration: Option<T::BlockNumber>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_owner(&who, &worker)?;

			ensure!(!AssignedJobs::<T>::contains_key(&worker), Error::<T>::AlreadyAssigned);

			if let Some(max_running_duration) = max_running_duration {
				ensure!(
					max_running_duration >= T::MinJobRunningDurationLen::get().into(),
					Error::<T>::MaxRunningDurationTooShort
				);
			}

			let deposit_base = T::JobDepositBase::get();
			let per_byte = T::JobInputDepositPerByte::get();
			let reserved =
				deposit_base.saturating_add(per_byte.saturating_mul((input.len() as u32).into()));

			T::Currency::reserve(&who, reserved)?;

			let job_id  =
				NextJobId::<T>::get(&worker).unwrap_or(T::JobId::initial_value());

			let job = Job::<T> {
				id: job_id,
				command,
				status: JobStatus::Created,
				result: None,
				output: None,
				created_by: who,
				created_at: Some(frame_system::Pallet::<T>::block_number()),
				started_at: None,
				completed_at: None,
				input,
				max_running_duration,
				reserved,
				deadline: None,
			};
			AssignedJobs::<T>::insert(&worker, job);

			Self::deposit_event(Event::JobCreated { worker: worker.clone() });

			let next_job_id = job_id.increment();
			NextJobId::<T>::insert(&worker, next_job_id);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn start_job(origin: OriginFor<T>) -> DispatchResult {
			let worker = ensure_signed(origin)?;
			// ensure worker or owner
			Self::ensure_worker(&worker)?;

			let Some(mut job) = AssignedJobs::<T>::get(&worker) else {
				return Err(Error::<T>::JobNotExists.into())
			};

			ensure!(job.status == JobStatus::Created, Error::<T>::AlreadyStarted);

			let current_block = frame_system::Pallet::<T>::block_number();
			let deadline = if let Some(max_running_duration) = job.max_running_duration {
				Some(current_block + max_running_duration)
			} else {
				None
			};

			job.status = JobStatus::Started;
			job.started_at = Some(current_block);
			job.deadline = deadline;

			AssignedJobs::<T>::insert(&worker, job);

			Self::deposit_event(Event::JobStarted { worker, deadline });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn complete_job(
			origin: OriginFor<T>,
			result: JobResult,
			output: Option<BoundedVec<u8, T::MaxJobOutputLen>>,
		) -> DispatchResult {
			let worker = ensure_signed(origin)?;
			Self::ensure_worker(&worker)?;

			let Some(mut job) = AssignedJobs::<T>::get(&worker) else {
				return Err(Error::<T>::JobNotExists.into())
			};

			ensure!(job.status == JobStatus::Started, Error::<T>::AlreadyStarted);

			job.status = JobStatus::Completed;
			job.result = Some(result);
			job.output = output;
			job.completed_at = Some(frame_system::Pallet::<T>::block_number());

			CompletedJobs::<T>::insert(&worker, job.id, job);
			AssignedJobs::<T>::remove(&worker);

			Self::deposit_event(Event::JobCompleted { worker, result });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn reclaim_completed_job(
			origin: OriginFor<T>,
			worker: T::AccountId,
			job_id: T::JobId
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let Some(worker_info) = T::WorkerManageable::worker_info(&worker) else {
				return Err(Error::<T>::WorkerNotExists.into())
			};

			ensure!(who == worker || who == worker_info.owner, Error::<T>::NoPermission);

			let Some(job) = CompletedJobs::<T>::take(&worker, &job_id) else {
				return Err(Error::<T>::JobNotExists.into())
			};

			T::Currency::unreserve(&job.created_by, job.reserved);

			Self::deposit_event(Event::JobReclaimed { worker, job_id });

			Ok(())
		}

		// TODO: Cancel Job (called by the owner)
		// TODO: Remove Job (called by the owner)
		// TODO: Report a job is timeout (called by anyone)
		// TODO: Do we need the worker keeping report the progress of the job? how?
	}

	impl<T: Config> Pallet<T> {
		fn ensure_owner(who: &T::AccountId, worker: &T::AccountId) -> DispatchResult {
			if let Some(worker_info) = T::WorkerManageable::worker_info(worker) {
				ensure!(who == &worker_info.owner, Error::<T>::NotTheOwner);
			} else {
				return Err(Error::<T>::WorkerNotExists.into())
			}

			Ok(())
		}

		fn ensure_worker(who: &T::AccountId) -> DispatchResult {
			ensure!(T::WorkerManageable::worker_exists(who), Error::<T>::WorkerNotExists);

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

		fn can_offline(worker: &T::AccountId) -> bool {
			!AssignedJobs::<T>::contains_key(worker)
		}

		fn before_offline(worker: &T::AccountId, reason: OfflineReason) {
			if reason == OfflineReason::Graceful {
				return
			}

			let Some(job) = AssignedJobs::<T>::get(worker) else {
				return
			};

			T::Currency::unreserve(&job.created_by, job.reserved);
			AssignedJobs::<T>::remove(worker)
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
