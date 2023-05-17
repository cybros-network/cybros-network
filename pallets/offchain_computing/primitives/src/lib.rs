#![cfg_attr(not(feature = "std"), no_std)]

use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use frame_support::{
	pallet_prelude::BoundedVec,
	traits::Get,
	RuntimeDebug,
};

use base_primitives::ImplSpecVersion;

/// Generic data that stored on-chain
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(DataLimit))]
#[codec(mel_bound(AccountId: MaxEncodedLen, Balance: MaxEncodedLen))]
pub struct ChainStoredData<AccountId, Balance, DataLimit: Get<u32>> {
	/// The depositor
	pub depositor: AccountId,
	/// The balance deposited for this data.
	///
	/// This pays for the data stored in this struct.
	pub actual_deposit: Balance,
	pub surplus_deposit: Balance,
	/// General information concerning this collection. Limited in length by `StringLimit`. This
	/// will generally be either a JSON dump or the hash of some JSON which can be found on a
	/// hash-addressable global publication system such as IPFS.
	pub data: BoundedVec<u8, DataLimit>,
}

#[derive(Clone, Decode, Encode, MaxEncodedLen, Eq, PartialEq, RuntimeDebug, TypeInfo, Default)]
pub enum ApplicableScope {
	/// Only the owner could create tasks.
	#[default]
	Owner,
	/// Anyone could create tasks.
	Public,
	// TODO:
	// /// Only a user in allow list could create tasks.
	// AllowList,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TaskPolicy<PoolId, BlockNumber> {
	/// Policy's id
	pub id: PoolId,
	/// This policy is available to use
	pub availability: bool,
	/// Who can create new task
	pub creating_task_scope: ApplicableScope,
	// TODO：rates strategy
	// TODO: allow create scheduled task and rule
	/// When the policy starts.
	pub start_block: Option<BlockNumber>,
	/// When the policy ends.
	pub end_block: Option<BlockNumber>,
	pub tasks_count: u32,
}

// TODO: Rates strategy (bound to CreatingTaskPolicy), e.g. Pay a constant or by duration of processing fee for each task, pay to worker or the owner
// TODO: WorkerPolicy: How to slashing, max processing duration, and etc.

/// Information about a pool.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PoolInfo<PoolId, AccountId, Balance, ImplId> {
	/// Pool's id
	pub id: PoolId,
	/// Pool's owner.
	pub owner: AccountId,
	/// The total balance deposited by the owner for all the storage data associated with this
	/// pool. Used by `destroy`.
	pub owner_deposit: Balance,
	/// The implementation id
	pub impl_id: ImplId,
	/// Allow create new task
	pub creating_task_availability: bool,
	pub min_impl_spec_version: ImplSpecVersion,
	pub max_impl_spec_version: ImplSpecVersion,
	/// The total number of outstanding task policies of this pool.
	pub task_policies_count: u32,
	/// The total number of outstanding tasks of this pool.
	pub tasks_count: u32,
	/// The total number of outstanding workers of this pool.
	pub workers_count: u32,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TaskStatus {
	/// Initial status, the task is pending to be processed
	Pending,
	/// The worker is processing the task
	Processing,
	/// Ending status, the worker processed the item
	Processed,
	/// Ending status, the worker can't process the task (e.g. force offline)
	Discarded,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TaskResult {
	///  and report success
	Success,
	/// Ending status, the worker processed the item and report failed
	Fail,
	/// Ending status, the error occurred when processing the task, the error not relates to the worker itself
	Error,
	/// Ending status, the error occurred when processing the task, the error relates to the worker itself
	Panic,
}

// TODO: Idea: TaskType: info will copy to Task, advanceable, creatable, minimum_deposit (more than actual will save to surplus_deposit)

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TaskInfo<TaskId, PolicyId, AccountId, Balance> {
	pub id: TaskId,
	pub policy_id: PolicyId,
	pub owner: AccountId,
	pub depositor: AccountId,
	pub deposit: Balance,
	/// The implementation spec version
	pub impl_spec_version: ImplSpecVersion,
	pub status: TaskStatus,
	pub result: Option<TaskResult>,
	/// This is soft expiring time, which means even the task has expired,
	/// worker can still process it, and earning from it,
	/// But other can destroy the task
	pub expires_at: u64,
	pub created_at: u64,
	pub assignee: Option<AccountId>,
	pub assigned_at: Option<u64>,
	pub processing_at: Option<u64>,
	pub ended_at: Option<u64>,
}
