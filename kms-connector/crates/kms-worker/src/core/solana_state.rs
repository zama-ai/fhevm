use anyhow::{Context, Result, anyhow, bail};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use borsh::{BorshDeserialize, BorshSerialize};
use reqwest::Client;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

const STATE_ACCOUNT_DISCRIMINATOR: [u8; 8] = *b"FHEHOST0";
const STATE_ACCOUNT_LAYOUT_VERSION: u32 = 1;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    BorshSerialize,
    BorshDeserialize,
)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    BorshSerialize,
    BorshDeserialize,
)]
pub struct EvmAddress([u8; 20]);

impl From<[u8; 20]> for EvmAddress {
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

pub fn host_identity_from_evm_address(address: EvmAddress) -> Pubkey {
    let mut bytes = [0_u8; 32];
    bytes[12..].copy_from_slice(&address.0);
    Pubkey::new(bytes)
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    BorshSerialize,
    BorshDeserialize,
)]
pub struct Handle([u8; 32]);

impl From<[u8; 32]> for Handle {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UserDecryptionDelegation {
    pub expiration_date: u64,
    pub last_slot_delegate_or_revoke: u64,
    pub delegation_counter: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AclState {
    owner: Pubkey,
    executor_program: Pubkey,
    paused: bool,
    pausers: BTreeSet<Pubkey>,
    persisted_allowed_pairs: BTreeSet<(Handle, Pubkey)>,
    allowed_for_decryption: BTreeSet<Handle>,
    user_decryption_delegations: BTreeMap<(Pubkey, Pubkey, Pubkey), UserDecryptionDelegation>,
    deny_list: BTreeSet<Pubkey>,
}

impl AclState {
    pub fn persist_allowed(&self, handle: Handle, account: Pubkey) -> bool {
        self.persisted_allowed_pairs.contains(&(handle, account))
    }

    pub fn is_allowed_for_decryption(&self, handle: Handle) -> bool {
        self.allowed_for_decryption.contains(&handle)
    }

    pub fn is_handle_delegated_for_user_decryption(
        &self,
        delegator: Pubkey,
        delegate: Pubkey,
        contract_address: Pubkey,
        handle: Handle,
        current_timestamp: u64,
    ) -> bool {
        self.persist_allowed(handle, delegator)
            && self.persist_allowed(handle, contract_address)
            && self
                .user_decryption_delegations
                .get(&(delegator, delegate, contract_address))
                .map(|delegation| delegation.expiration_date >= current_timestamp)
                .unwrap_or(false)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct HostProgramState {
    owner: Pubkey,
    upgrade_authority: Pubkey,
    host_chain_id: u64,
    state_version: u32,
    acl: AclState,
}

impl HostProgramState {
    pub fn acl(&self) -> &AclState {
        &self.acl
    }
}

#[derive(Clone, Debug)]
pub struct SolanaStateClient {
    client: Client,
    rpc_url: String,
    state_pda: String,
}

impl SolanaStateClient {
    pub fn new(rpc_url: impl Into<String>, state_pda: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            rpc_url: rpc_url.into(),
            state_pda: state_pda.into(),
        }
    }

    pub async fn fetch_state(&self) -> Result<HostProgramState> {
        let result = self
            .rpc_call(
                "getAccountInfo",
                json!([
                    self.state_pda,
                    {
                        "encoding": "base64",
                        "commitment": "confirmed"
                    }
                ]),
            )
            .await?;

        let value = result
            .get("value")
            .ok_or_else(|| anyhow!("getAccountInfo missing value"))?;
        if value.is_null() {
            bail!(
                "state account {} not found on Solana host chain",
                self.state_pda
            );
        }

        let data_b64 = value
            .get("data")
            .and_then(Value::as_array)
            .and_then(|items| items.first())
            .and_then(Value::as_str)
            .context("getAccountInfo returned unsupported data format")?;

        let bytes = BASE64_STANDARD
            .decode(data_b64)
            .context("failed to decode Solana state account base64")?;
        decode_stored_state(&bytes)
    }

    async fn rpc_call(&self, method: &str, params: Value) -> Result<Value> {
        let response = self
            .client
            .post(&self.rpc_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": params,
            }))
            .send()
            .await
            .with_context(|| format!("rpc request failed for {method}"))?
            .error_for_status()
            .with_context(|| format!("rpc HTTP error for {method}"))?
            .json::<Value>()
            .await
            .with_context(|| format!("rpc JSON parse failed for {method}"))?;

        if let Some(error) = response.get("error") {
            bail!("rpc {method} returned error: {error}");
        }

        response
            .get("result")
            .cloned()
            .with_context(|| format!("rpc {method} missing result"))
    }
}

#[derive(Debug, BorshDeserialize)]
struct StoredHostProgramState {
    discriminator: [u8; 8],
    layout_version: u32,
    state: HostProgramState,
}

fn decode_stored_state(bytes: &[u8]) -> Result<HostProgramState> {
    let mut slice = bytes;
    let stored = StoredHostProgramState::deserialize(&mut slice)
        .map_err(|err| anyhow!("failed to deserialize Solana host state: {err}"))?;
    if stored.discriminator != STATE_ACCOUNT_DISCRIMINATOR
        || stored.layout_version != STATE_ACCOUNT_LAYOUT_VERSION
    {
        bail!("unexpected Solana host state account layout");
    }
    Ok(stored.state)
}
