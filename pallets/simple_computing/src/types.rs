use frame_support::{
	sp_std::prelude::*,
	sp_runtime::Saturating,
	BoundedVec, RuntimeDebug,
};
use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use pallet_computing_workers::BalanceOf;
use crate::macros::impl_auto_increment;

pub trait AutoIncrement {
	fn increment(&self) -> Self;
	fn initial_value() -> Self;
}
impl_auto_increment!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

pub type JobCommand<T> = BoundedVec<u8, <T as crate::Config>::MaxJobCommandLen>;
pub type JobInput<T> = BoundedVec<u8, <T as crate::Config>::MaxJobInputLen>;
pub type JobOutput<T> = BoundedVec<u8, <T as crate::Config>::MaxJobOutputLen>;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum JobStatus {
	Created,
	Started,
	Completed,
	Timeout,
	Cancelled,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum JobResult {
	Success,
	Failed,
	Errored,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug, Clone, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub struct Job<T: crate::Config> {
	pub id: T::JobId,
	/// Discriminator used by a worker to decide how to execute a job.
	pub command: JobCommand<T>,
	/// Payload the worker should use when executing the job
	pub input: JobInput<T>,
	/// Number of blocks a job may be held by a worker before it is considered timeout after started.
	pub max_running_duration: Option<T::BlockNumber>,
	/// Balance of the job creator reserve for the job's storage
	pub reserved: BalanceOf<T>,
	/// The last block that the job valid
	pub deadline: Option<T::BlockNumber>,
	pub status: JobStatus,
	pub result: Option<JobResult>,
	pub output: Option<JobOutput<T>>,
	pub created_by: T::AccountId,
	pub created_at: Option<T::BlockNumber>,
	pub started_at: Option<T::BlockNumber>,
	pub completed_at: Option<T::BlockNumber>,
}
