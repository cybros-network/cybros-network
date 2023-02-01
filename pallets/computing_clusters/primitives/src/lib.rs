#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use scale_codec::{Encode, Decode, MaxEncodedLen};
use scale_info::TypeInfo;
use frame_support::{
	RuntimeDebug
};

/// Type used for unique identifier of each cluster.
pub type ClusterId = u32;

// #[derive(Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug)]
// #[scale_info(skip_type_params(T))]
// pub struct Cluster<T: Config> {
// 	/// The identifier of the pool to which `who` belongs.
// 	pub id: ClusterId,
// 	// TODO: Who can use the cluster? All / Member-only
// }
//
// #[derive(Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug)]
// #[scale_info(skip_type_params(T))]
// pub struct ClusterWorker<T: Config> {
// }
