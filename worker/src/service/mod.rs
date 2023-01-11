#![allow(dead_code)]

mod builder;
mod metrics;
mod task_manager;

pub mod config;
pub mod error;

pub use self::{
	error::Error,
	builder::new_worker,
};
pub use self::config::{
	BasePath, Configuration, TaskType,
};

pub use crate::tracing::TracingReceiver;
#[doc(hidden)]
pub use std::{ops::Deref, result::Result, sync::Arc};
pub use self::task_manager::{SpawnTaskHandle, Task, TaskManager, TaskRegistry, DEFAULT_GROUP_NAME};
