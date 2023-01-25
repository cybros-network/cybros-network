#![allow(dead_code)]

mod metrics;
mod task_manager;

pub mod config;
pub mod error;

#[doc(hidden)]
pub use std::{ops::Deref, result::Result, sync::Arc};
pub use crate::framework::tracing::TracingReceiver;
pub use self::{
	config::{
		BasePath, Configuration, TaskType,
	},
	error::Error,
	task_manager::{SpawnTaskHandle, Task, TaskManager, TaskRegistry, DEFAULT_GROUP_NAME}
};
