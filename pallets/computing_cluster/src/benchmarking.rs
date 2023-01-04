//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as ComputingCluster;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	impl_benchmark_test_suite!(ComputingCluster, crate::mock::new_test_ext(), crate::mock::Test);
}
