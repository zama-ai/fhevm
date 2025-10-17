use alloy_primitives::Keccak256;
use bigdecimal::num_bigint::BigInt;
use fhevm_engine_common::{types::AllowEvents, utils::safe_deserialize_key};
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    ClearConst, Database as ListenerDatabase, Handle, LogTfhe, TransactionHash,
};
use rand::Rng;
use sqlx::Postgres;
use std::sync::Arc;
use tracing::info;

use alloy::primitives::Log;
pub fn tfhe_event(data: TfheContractEvents) -> Log<TfheContractEvents> {
    let address = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    Log::<TfheContractEvents> { address, data }
}

pub const DEF_TYPE: FheType = FheType::FheUint64;

#[derive(Clone)]
pub enum FheType {
    FheBool = 0,
    FheUint4 = 1,
    FheUint8 = 2,
    FheUint16 = 3,
    FheUint32 = 4,
    FheUint64 = 5,
    FheUint128 = 6,
    FheUint160 = 7,
    FheUint256 = 8,
    FheBytes64 = 9,
    FheBytes128 = 10,
    FheBytes256 = 11,
}

impl From<u8> for FheType {
    fn from(value: u8) -> Self {
        match value {
            0 => FheType::FheBool,
            1 => FheType::FheUint4,
            2 => FheType::FheUint8,
            3 => FheType::FheUint16,
            4 => FheType::FheUint32,
            5 => FheType::FheUint64,
            6 => FheType::FheUint128,
            7 => FheType::FheUint160,
            8 => FheType::FheUint256,
            9 => FheType::FheBytes64,
            10 => FheType::FheBytes128,
            11 => FheType::FheBytes256,
            _ => panic!("Unsupported FheType"),
        }
    }
}

pub fn next_random_handle(ct_type: FheType) -> Handle {
    let ecfg = EnvConfig::new();
    let mut handle_hash = Keccak256::new();
    handle_hash.update(rand::rng().random::<u64>().to_be_bytes());
    let mut handle = handle_hash.finalize().to_vec();
    assert_eq!(handle.len(), 32);
    // Handle from computation
    handle[21] = 255u8;
    handle[22..30].copy_from_slice(&ecfg.chain_id.to_be_bytes());
    handle[30] = ct_type as u8;
    handle[31] = 0u8;
    Handle::from_slice(&handle)
}
pub fn default_dependence_cache_size() -> u16 {
    128
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum Transaction {
    ERC20Transfer,
    DEXSwapRequest,
    DEXSwapClaim,
    MULChain,
    ADDChain,
    InputVerif,
    GenPubDecHandles,
    GenUsrDecHandles,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum ERCTransferVariant {
    Whitepaper,
    NoCMUX,
    NA,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum GeneratorKind {
    Rate,
    Count,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum Dependence {
    Dependent,
    Independent,
    NA,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum Inputs {
    ReuseInputs,
    NewInputs,
    NA,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub struct Scenario {
    pub transaction: Transaction,
    pub variant: ERCTransferVariant,
    pub kind: GeneratorKind,
    pub inputs: Inputs,
    pub is_dependent: Dependence,
    pub contract_address: String,
    pub user_address: String,
    pub scenario: Vec<(f64, u64)>,
}

pub struct Job {
    pub id: u64,
    pub scenarios: Vec<Scenario>,
    pub cancel_token: tokio_util::sync::CancellationToken,
}

#[derive(Clone)]
pub struct Context {
    pub args: Args,
    pub ecfg: EnvConfig,
    pub cancel_token: tokio_util::sync::CancellationToken,
}

#[allow(dead_code)]
pub async fn allow_handle(
    handle: &Vec<u8>,
    event_type: AllowEvents,
    account_address: String,
    transaction_id: TransactionHash,
    pool: &sqlx::Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let started_at = std::time::Instant::now();

    let ecfg = EnvConfig::new();
    let _query =
            sqlx::query!(
                "INSERT INTO allowed_handles(tenant_id, handle, account_address, event_type, transaction_id) VALUES($1, $2, $3, $4, $5)
                     ON CONFLICT DO NOTHING;",
                ecfg.tenant_id,
                handle,
                account_address,
                event_type as i16,
                transaction_id.to_vec(),
            ).execute(pool).await?;
    let _query = sqlx::query!(
        "INSERT INTO pbs_computations(tenant_id, handle, transaction_id) VALUES($1, $2, $3) 
                     ON CONFLICT DO NOTHING;",
        ecfg.tenant_id,
        handle,
        transaction_id.to_vec()
    )
    .execute(pool)
    .await?;

    tracing::debug!(target: "tool", duration = ?started_at.elapsed(), "Handle allowed, db_query");
    Ok(())
}

#[allow(dead_code)]
pub async fn allow_handles(
    handles: &Vec<Vec<u8>>,
    event_type: AllowEvents,
    account_address: String,
    pool: &sqlx::Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ecfg = EnvConfig::new();
    let tenant_id = vec![ecfg.tenant_id; handles.len()];
    let account_address = vec![account_address; handles.len()];
    let event_type = vec![event_type as i16; handles.len()];
    let _query = sqlx::query!(
        "INSERT INTO allowed_handles(tenant_id, handle, account_address, event_type)
                 SELECT * FROM UNNEST($1::INTEGER[], $2::BYTEA[], $3::TEXT[], $4::SMALLINT[])
                 ON CONFLICT DO NOTHING;",
        &tenant_id,
        handles,
        &account_address,
        &event_type,
    )
    .execute(pool)
    .await?;
    let _query = sqlx::query!(
        "INSERT INTO pbs_computations(tenant_id, handle)
                 SELECT * FROM UNNEST($1::INTEGER[], $2::BYTEA[]) 
                 ON CONFLICT DO NOTHING;",
        &tenant_id,
        handles,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub fn as_scalar_uint(big_int: &BigInt) -> ClearConst {
    let (_, bytes) = big_int.to_bytes_be();
    ClearConst::from_be_slice(&bytes)
}

pub async fn generate_trivial_encrypt(
    _contract_address: &str,
    user_address: &str,
    transaction_hash: TransactionHash,
    listener_event_to_db: &mut ListenerDatabase,
    ct_type: Option<FheType>,
    ct_value: Option<u128>,
    is_allowed: bool,
) -> Result<Handle, Box<dyn std::error::Error>> {
    let caller = user_address.parse().unwrap();
    let ct_type = ct_type.unwrap_or(DEF_TYPE);
    let handle = next_random_handle(ct_type.clone());
    let ct_value = ct_value.unwrap_or(rand::rng().random::<u128>());
    let log = LogTfhe {
        event: tfhe_event(TfheContractEvents::TrivialEncrypt(
            host_listener::contracts::TfheContract::TrivialEncrypt {
                caller,
                pt: as_scalar_uint(&BigInt::from(ct_value)),
                toType: ct_type as u8,
                result: handle,
            },
        )),
        transaction_hash: Some(transaction_hash),
        is_allowed,
        block_number: 1,
    };
    let mut tx = listener_event_to_db.new_transaction().await?;
    listener_event_to_db
        .insert_tfhe_event(&mut tx, &log)
        .await?;
    tx.commit().await?;
    Ok(handle)
}

pub async fn query_and_save_pks(
    tenant_id: i32,
    pool: &sqlx::PgPool,
) -> Result<(tfhe::CompactPublicKey, Arc<tfhe::zk::CompactPkeCrs>), Box<dyn std::error::Error>> {
    let keys = KEYS.read().await;
    if let Some(keys) = keys.as_ref() {
        return Ok(keys.clone());
    }
    drop(keys);
    let mut keys = KEYS.write().await;
    if let Some(keys) = keys.as_ref() {
        return Ok(keys.clone());
    }

    info!("Querying database for keys of tenant {}", tenant_id);

    let tenants = sqlx::query!(
        "
            SELECT tenant_id, chain_id, acl_contract_address, verifying_contract_address, pks_key, public_params
            FROM tenants
            WHERE tenant_id = $1
        ",
        tenant_id,
    )
    .fetch_one(pool)
    .await?;

    let pks: tfhe::CompactPublicKey = safe_deserialize_key(&tenants.pks_key)?;
    let public_params: Arc<tfhe::zk::CompactPkeCrs> =
        Arc::new(safe_deserialize_key(&tenants.public_params)?);

    keys.replace((pks.clone(), public_params.clone()));
    Ok((pks, public_params))
}

pub async fn get_ciphertext_digests(
    handle: &[u8],
    pool: &sqlx::PgPool,
    max_retries: usize,
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
    for _ in 0..max_retries {
        let digests = sqlx::query!(
            "
            SELECT ciphertext, ciphertext128
            FROM ciphertext_digest
            WHERE handle = $1
            ",
            handle,
        )
        .fetch_one(pool)
        .await;

        if let Ok(digests) = digests {
            if digests.ciphertext.is_some() && digests.ciphertext128.is_some() {
                return Ok((digests.ciphertext.unwrap(), digests.ciphertext128.unwrap()));
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
    Ok((vec![], vec![]))
}

/// User configuration in which benchmarks must be run.
#[derive(Default, Clone)]
pub struct EnvConfig {
    #[allow(dead_code)]
    pub evgen_scenario: String,
    #[allow(dead_code)]
    pub evgen_db_url: String,
    #[allow(dead_code)]
    pub acl_contract_address: String,
    #[allow(dead_code)]
    pub chain_id: i64,
    #[allow(dead_code)]
    pub api_key: String,
    #[allow(dead_code)]
    pub tenant_id: i32,
    #[allow(dead_code)]
    pub synthetic_chain_length: u32,
    #[allow(dead_code)]
    pub min_decryption_type: u8,
    #[allow(dead_code)]
    pub max_decryption_type: u8,
    #[allow(dead_code)]
    pub output_handles_for_pub_decryption: String,
    #[allow(dead_code)]
    pub output_handles_for_usr_decryption: String,
}

use std::env;

use crate::args::Args;
use crate::zk_gen::KEYS;
impl EnvConfig {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let evgen_scenario: String = match env::var("EVGEN_SCENARIO") {
            Ok(val) => val,
            Err(_) => "data/evgen_scenario.csv".to_string(),
        };
        let evgen_db_url: String = match env::var("EVGEN_DB_URL") {
            Ok(val) => val,
            Err(_) => "postgresql://postgres:postgres@127.0.0.1:5432/coprocessor".to_string(),
        };
        let acl_contract_address: String = match env::var("ACL_CONTRACT_ADDRESS") {
            Ok(val) => val,
            Err(_) => "0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c".to_string(),
        };
        let chain_id: i64 = match env::var("CHAIN_ID") {
            Ok(val) => val.parse::<i64>().unwrap(),
            Err(_) => 12345i64,
        };
        let api_key: String = match env::var("API_KEY") {
            Ok(val) => val,
            Err(_) => "a1503fb6-d79b-4e9e-826d-44cf262f3e05".to_string(),
        };
        let tenant_id: i32 = match env::var("TENANT_ID") {
            Ok(val) => val.parse::<i32>().unwrap(),
            Err(_) => 1i32,
        };
        let synthetic_chain_length: u32 = match env::var("SYNTHETIC_CHAIN_LENGTH") {
            Ok(val) => val.parse::<u32>().unwrap(),
            Err(_) => 10u32,
        };
        let min_decryption_type: u8 = match env::var("MIN_DECRYPTION_TYPE") {
            Ok(val) => val.parse::<u8>().unwrap(),
            Err(_) => 0u8,
        };
        let max_decryption_type: u8 = match env::var("MAX_DECRYPTION_TYPE") {
            Ok(val) => val.parse::<u8>().unwrap(),
            Err(_) => 6u8,
        };
        let output_handles_for_pub_decryption: String =
            match env::var("OUTPUT_HANDLES_FOR_PUB_DECRYPTION") {
                Ok(val) => val,
                Err(_) => "data/handles_for_pub_decryption".to_string(),
            };
        let output_handles_for_usr_decryption: String =
            match env::var("OUTPUT_HANDLES_FOR_USR_DECRYPTION") {
                Ok(val) => val,
                Err(_) => "data/handles_for_usr_decryption".to_string(),
            };

        EnvConfig {
            evgen_scenario,
            evgen_db_url,
            acl_contract_address,
            chain_id,
            api_key,
            tenant_id,
            synthetic_chain_length,
            min_decryption_type,
            max_decryption_type,
            output_handles_for_pub_decryption,
            output_handles_for_usr_decryption,
        }
    }
}

pub async fn insert_tfhe_event(
    listener_event_to_db: &ListenerDatabase,
    transaction_hash: TransactionHash,
    event: Log<TfheContractEvents>,
    is_allowed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let started_at = tokio::time::Instant::now();
    let mut tx = listener_event_to_db.new_transaction().await?;
    let log = LogTfhe {
        event,
        transaction_hash: Some(transaction_hash),
        is_allowed,
        block_number: 1,
    };
    listener_event_to_db
        .insert_tfhe_event(&mut tx, &log)
        .await?;
    tx.commit().await?;
    tracing::debug!(target: "tool", duration = ?started_at.elapsed(), "TFHE event, db_query");
    Ok(())
}
