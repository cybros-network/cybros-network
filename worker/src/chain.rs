use subxt::{
	config::{WithExtrinsicParams, Config},
	tx::{BaseExtrinsicParams, PlainTip}
};
use node_primitives::{types::*, opaque::*};

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a polkadot node.
pub type CybrosExtrinsicParams<T> = BaseExtrinsicParams<T, PlainTip>;

/// Default set of commonly used types by Substrate runtimes.
// Note: We only use this at the type level, so it should be impossible to
// create an instance of it.
pub enum SubstrateConfig {}

impl Config for SubstrateConfig {
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = Hashing;
	type AccountId = AccountId;
	type Address = sp_runtime::MultiAddress<AccountId, ()>;
	type Header = Header;
	type Signature = Signature;
	type ExtrinsicParams = CybrosExtrinsicParams<Self>;
}

/// Default set of commonly used types by Cybros nodes.
pub type CybrosConfig = WithExtrinsicParams<
	SubstrateConfig,
	CybrosExtrinsicParams<SubstrateConfig>,
>;
