use crate::*;
use frame_support::{
	parameter_types,
	weights::Weight,
};
use sp_core::ConstU32;

parameter_types! {
	/// Allocate at most 20% of each block for message processing.
	///
	/// Is set to 20% since the scheduler can already consume a maximum of 80%.
	pub MessageQueueServiceWeight: Option<Weight> = Some(Perbill::from_percent(20) * RuntimeBlockWeights::get().max_block);
}

impl pallet_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	/// NOTE: Always set this to `NoopMessageProcessor` for benchmarking.
	#[cfg(feature = "runtime-benchmarks")]
	type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor;
	type Size = u32;
	type QueueChangeHandler = ();
	type HeapSize = ConstU32<{ 64 * 1024 }>;
	type MaxStale = ConstU32<128>;
	type ServiceWeight = MessageQueueServiceWeight;
}
