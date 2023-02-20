use std::str::FromStr;
use runtime_primitives::types::AccountId;
use runtime::{
	AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ChainProfile {
	name: String,
	id: String,
	root_key: AccountId,
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
	endowed_accounts: Vec<(AccountId, String)>,
}

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let chain_profile_in_bytes = include_bytes!("../res/development_chain_profile.json");
	let chain_profile: ChainProfile =
		serde_json::from_slice(chain_profile_in_bytes).expect("Bad chain profile");

	Ok(
		ChainSpec::from_genesis(
			// Name
			&chain_profile.name.clone(),
			// ID
			&chain_profile.id.clone(),
			ChainType::Development,
			move || {
				let chain_profile = chain_profile.clone();
				genesis_config(
					wasm_binary,
					// Initial PoA authorities
					chain_profile.initial_authorities,
					// Sudo account
					chain_profile.root_key,
					// Pre-funded accounts
					chain_profile.endowed_accounts
						.into_iter()
						.map(|(k, amount)| (k, u128::from_str(&amount).expect("Bad amount")))
						.collect(),
					true
				)
			},
			// Boot nodes
			vec![],
			// Telemetry
			None,
			// Protocol ID
			None,
			// Properties
			None,
			None,
			// Extensions
			None,
		)
	)
}

pub fn local_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let chain_profile_in_bytes = include_bytes!("../res/local_chain_profile.json");
	let chain_profile: ChainProfile =
		serde_json::from_slice(chain_profile_in_bytes).expect("Bad chain profile");

	Ok(
		ChainSpec::from_genesis(
			// Name
			&chain_profile.name.clone(),
			// ID
			&chain_profile.id.clone(),
			ChainType::Local,
			move || {
				let chain_profile = chain_profile.clone();
				genesis_config(
					wasm_binary,
					// Initial PoA authorities
					chain_profile.initial_authorities,
					// Sudo account
					chain_profile.root_key,
					// Pre-funded accounts
					chain_profile.endowed_accounts
						.into_iter()
						.map(|(k, amount)| (k, u128::from_str(&amount).expect("Bad amount")))
						.collect(),
					true
				)
			},
			// Boot nodes
			vec![],
			// Telemetry
			None,
			// Protocol ID
			None,
			// Properties
			None,
			None,
			// Extensions
			None,
		)
	)
}

/// Configure initial storage state for FRAME modules.
fn genesis_config(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, u128)>,
	_enable_println: bool,
) -> GenesisConfig {
	assert!(
		initial_authorities
			.iter()
			.map(|(k, _, _)| k)
			.chain(&[root_key.clone()])
			.cloned()
			.all(|account| {
				endowed_accounts
					.iter()
					.any(|(endowed, _)| account == endowed.clone())
			}),
		"All the genesis accounts must be endowed; qed."
	);

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts,
		},
		transaction_payment: Default::default(),
		vesting: Default::default(),
		aura: AuraConfig { authorities: initial_authorities.iter().map(|x| (x.1.clone())).collect() },
		grandpa: GrandpaConfig { authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect() },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
	}
}
