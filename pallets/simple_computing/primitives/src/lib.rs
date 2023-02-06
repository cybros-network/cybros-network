#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	sp_std::prelude::*,
	pallet_prelude::*,
	BoundedVec, RuntimeDebug,
};
use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

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

#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug)]
#[scale_info(skip_type_params(CommandLimit, InputLimit, OutputLimit))]
pub struct Job<
	AccountId, Balance, BlockNumber,
	JobId,
	CommandLimit: Get<u32>, InputLimit: Get<u32>, OutputLimit: Get<u32>
> {
	pub id: JobId,
	/// Discriminator used by a worker to decide how to execute a job.
	pub command: BoundedVec<u8, CommandLimit>,
	/// Payload the worker should use when executing the job
	pub input: BoundedVec<u8, InputLimit>,
	/// Number of blocks a job may be held by a worker before it is considered timeout after started.
	pub max_running_duration: Option<BlockNumber>,
	/// Balance of the job creator reserve for the job's storage
	pub reserved: Balance,
	/// The last block that the job valid
	pub deadline: Option<BlockNumber>,
	pub status: JobStatus,
	pub result: Option<JobResult>,
	pub output: Option<BoundedVec<u8, OutputLimit>>,
	pub created_by: AccountId,
	pub created_at: Option<BlockNumber>,
	pub started_at: Option<BlockNumber>,
	pub completed_at: Option<BlockNumber>,
}
