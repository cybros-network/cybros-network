use frame_support::{sp_std::prelude::*, BoundedVec, RuntimeDebug};
use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

pub type JobPayloadVec<T> = BoundedVec<u8, <T as crate::Config>::MaxJobPayloadLen>;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum JobStatus {
	Created,
	// Enqueued, // Just note that no queue in simple computing
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
	pub status: JobStatus,
	pub result: Option<JobResult>,
	pub created_by: T::AccountId,
	pub created_at: Option<T::BlockNumber>,
	// pub assigned_at: Option<T::BlockNumber>, // Just note that no assign in simple computing
	// pub enqueued_at: Option<T::BlockNumber>, // Just note that no queue in simple computing
	pub started_at: Option<T::BlockNumber>,
	pub completed_at: Option<T::BlockNumber>,
	pub payload: JobPayloadVec<T>,
}
