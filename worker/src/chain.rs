#![allow(dead_code)]

use subxt::{
	config::{
		extrinsic_params::{BaseExtrinsicParams, BaseExtrinsicParamsBuilder},
		substrate::{SubstrateHeader, H256, BlakeTwo256},
		polkadot::PlainTip,
		WithExtrinsicParams, Config,
	},
	utils::{
		account_id::AccountId32,
		multi_address::MultiAddress,
		multi_signature::MultiSignature,
	},
};

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a polkadot node.
pub type CybrosExtrinsicParams<T> = BaseExtrinsicParams<T, PlainTip>;

/// A builder which leads to [`CybrosExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type CybrosExtrinsicParamsBuilder<T> = BaseExtrinsicParamsBuilder<T, PlainTip>;

/// Default set of commonly used types by Substrate runtimes.
// Note: We only use this at the type level, so it should be impossible to
// create an instance of it.
pub enum SubstrateConfig {}

// Subxt makes its own types, `runtime-primitives` is incompatible here, so have to hard code
impl Config for SubstrateConfig {
	type Index = u32;
	type BlockNumber = u32;
	type Hash = H256;
	type AccountId = AccountId32;
	type Address = MultiAddress<Self::AccountId, ()>;
	type Signature = MultiSignature;
	type Hasher = BlakeTwo256;
	type Header = SubstrateHeader<Self::BlockNumber, BlakeTwo256>;
	type ExtrinsicParams = CybrosExtrinsicParams<Self>;
}

/// Default set of commonly used types by Cybros nodes.
pub type CybrosConfig = WithExtrinsicParams<
	SubstrateConfig,
	CybrosExtrinsicParams<SubstrateConfig>,
>;
