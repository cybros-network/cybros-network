#![allow(dead_code)]

mod builder;
mod metrics;
mod task_manager;

pub mod config;
pub mod error;

#[doc(hidden)]
pub use std::{ops::Deref, result::Result, sync::Arc};
pub use crate::tracing::TracingReceiver;
pub use self::{
	builder::{
		new_worker,
		SpawnTasksParams, spawn_tasks
	},
	config::{
		BasePath, Configuration, TaskType,
	},
	error::Error,
	task_manager::{SpawnTaskHandle, Task, TaskManager, TaskRegistry, DEFAULT_GROUP_NAME}
};

pub mod key_types {
	use sp_core::crypto::KeyTypeId;

	pub const IDENTITY: KeyTypeId = KeyTypeId(*b"iden");
	pub const OLM_IDENTITY: KeyTypeId = KeyTypeId(*b"olmi");
}
