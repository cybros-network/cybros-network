#![cfg_attr(not(feature = "std"), no_std)]

use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use frame_support::{
	pallet_prelude::BoundedVec,
	traits::Get,
	RuntimeDebug,
};

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
pub enum CreatingTaskPermission {
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
pub struct CreatingTaskPolicy<BlockNumber> {
	/// Whether anyone can mint or if minters are restricted to some subset.
	pub permission: CreatingTaskPermission,
	// TODO：rates strategy
	// /// An optional price per create task.
	// pub price: Option<Balance>,
	// TODO: allow create scheduled task and rule
	/// When the policy starts.
	pub start_block: Option<BlockNumber>,
	/// When the policy ends.
	pub end_block: Option<BlockNumber>,
}

// TODO: Rates strategy (bound to CreatingTaskPolicy), e.g. Pay a constant or by duration of processing fee for each task, pay to worker or the owner
// TODO: WorkerPolicy: How to slashing, max processing duration, and etc.

/// Information about a pool.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PoolInfo<PoolId, AccountId, Balance> {
	/// Pool's id
	pub id: PoolId,
	/// Pool's owner.
	pub owner: AccountId,
	/// The total balance deposited by the owner for all the storage data associated with this
	/// pool. Used by `destroy`.
	pub owner_deposit: Balance,
	/// Pool's stash account.
	pub stash_account: AccountId,
	/// Allow creating task
	pub creating_task_ability: bool,
	/// The total number of outstanding create task policies of this pool.
	pub creating_task_policies_count: u32,
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
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TaskResult {
	///  and report success
	Success,
	/// Ending status, the worker processed the item and report failed
	Failed,
	/// Ending status, the worker processed the item and report success
	Errored,
}

// TODO: Idea: TaskType: info will copy to Task, advanceable, creatable, minimum_deposit (more than actual will save to surplus_deposit)

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TaskInfo<TaskId, AccountId, Balance> {
	pub id: TaskId,
	pub creator: AccountId,
	pub owner: AccountId,
	pub owner_deposit: Balance,
	pub status: TaskStatus,
	pub result: Option<TaskResult>,
	/// This is soft expiring time, which means even the task has expired,
	/// worker can still process it, and earning from it,
	/// But other can destroy the task
	pub expires_at: u64,
	pub created_by: AccountId,
	pub created_at: u64,
	pub assignee: Option<AccountId>,
	pub assigned_at: Option<u64>,
	pub processing_at: Option<u64>,
	pub processed_at: Option<u64>,
}
