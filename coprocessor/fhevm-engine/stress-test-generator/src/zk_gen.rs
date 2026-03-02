use crate::utils::{
    new_transaction_id, next_random_handle, pool, query_and_save_pks, EnvConfig, Inputs, DEF_TYPE,
};
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::utils::to_hex;
use host_listener::database::tfhe_event_propagate::{Database as ListenerDatabase, Handle};
use rand::Rng;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, ops::DerefMut};
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::utils::Context;

const SIZE: usize = 92;
const CACHED_INPUTS_COUNT: u8 = 16;

type ContractKey = (String, String);
type ContractValues = Vec<Option<Handle>>;
type ContractInputs = RwLock<HashMap<ContractKey, ContractValues>>;

lazy_static::lazy_static! {
    pub static ref ZK_PROOF_ID: std::sync::atomic::AtomicI64 =
        std::sync::atomic::AtomicI64::new(rand::rng().random::<i64>().abs());


    pub static ref CONTRACT_INPUTS: ContractInputs = RwLock::new(HashMap::new());

    pub static ref KEYS: RwLock<Option<(tfhe::CompactPublicKey, Arc<tfhe::zk::CompactPkeCrs>)>> = RwLock::new(None);

}

#[derive(Debug, Clone)]
struct ZkData {
    pub contract_address: String,
    pub user_address: String,
    pub acl_contract_address: String,
    pub chain_id: ChainId,
}

impl ZkData {
    pub fn assemble(&self) -> anyhow::Result<[u8; SIZE]> {
        let contract_bytes =
            alloy_primitives::Address::from_str(&self.contract_address)?.into_array();
        let user_bytes = alloy_primitives::Address::from_str(&self.user_address)?.into_array();
        let acl_bytes =
            alloy_primitives::Address::from_str(&self.acl_contract_address)?.into_array();
        let chain_id_bytes: [u8; 32] =
            Into::<alloy_primitives::U256>::into(self.chain_id).to_be_bytes();

        // Copy contract address into the first 20 bytes
        let front: Vec<u8> = [contract_bytes, user_bytes, acl_bytes].concat();
        let mut data = [0_u8; SIZE];
        data[..60].copy_from_slice(front.as_slice());
        data[60..].copy_from_slice(&chain_id_bytes);
        Ok(data)
    }
}

#[allow(clippy::too_many_arguments)]
async fn insert_proof(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    request_id: i64,
    zk_pok: Vec<u8>,
    aux: ZkData,
    db_notify_channel: &str,
    transaction_id: Handle,
    retry_count: i32,
    _block_number: u64,
) -> Result<(), sqlx::Error> {
    //  Insert ZkPok into database
    sqlx::query(
            "INSERT INTO verify_proofs (zk_proof_id, input, host_chain_id, contract_address, user_address, verified, transaction_id, retry_count)
            VALUES ($1, $2, $3, $4, $5, NULL, $6, $7)" 
        ).bind(request_id)
        .bind(zk_pok)
        .bind(aux.chain_id.as_i64())
        .bind(aux.contract_address.clone())
        .bind(aux.user_address.clone())
        .bind(transaction_id.to_vec())
        .bind(retry_count)
        .execute(tx.deref_mut()).await?;
    sqlx::query("SELECT pg_notify($1, '')")
        .bind(db_notify_channel)
        .execute(tx.deref_mut())
        .await
        .unwrap();

    // We cannot begin measuring the transaction as it will always fail due to VerifyProofNotRequested in L2
    // see also: VerifyProofNotRequested(uint256) | 0x4711083f
    /*
        let _ = telemetry::try_begin_transaction(
            pool,
            aux.chain_id,
            &transaction_id.to_vec(),
            _block_number,
        )
        .await;
    */

    Ok(())
}
async fn wait_for_verification_and_handle(
    pool: &sqlx::PgPool,
    zk_proof_id: i64,
    max_retries: usize,
) -> Result<Vec<Handle>, sqlx::Error> {
    for _ in 0..max_retries {
        let result = sqlx::query!(
            "SELECT verified, handles FROM verify_proofs WHERE zk_proof_id = $1",
            zk_proof_id
        )
        .fetch_one(pool)
        .await?;

        match result.verified {
            Some(verified) => {
                if !verified {
                    error!(zk_proof_id, "ZK verification failed")
                }
                let Some(handle) = result.handles else {
                    error!(zk_proof_id, "No handle generated");
                    return Err(sqlx::Error::RowNotFound);
                };
                assert!(handle.len() % 32 == 0);
                return Ok(handle
                    .chunks(32)
                    .map(|c| Handle::right_padding_from(c))
                    .collect());
            }
            None => tokio::time::sleep(Duration::from_millis(100)).await,
        }
    }
    error!(
        zk_proof_id,
        max_retries, "Couldn't verify the ZK, timeout reached"
    );
    Err(sqlx::Error::RowNotFound)
}

pub async fn generate_random_handle_amount_if_none(
    ctx: &Context,
    result: Option<Handle>,
    contract_address: &String,
    user_address: &String,
) -> Result<Handle, Box<dyn std::error::Error>> {
    if let Some(res) = result {
        return Ok(res);
    }
    Ok(generate_random_handle_vec(ctx, 1, contract_address, user_address).await?[0])
}

pub async fn generate_random_handle_vec(
    ctx: &Context,
    count: u8,
    contract_address: &String,
    user_address: &String,
) -> Result<Vec<Handle>, Box<dyn std::error::Error>> {
    assert!(count <= 254);
    let ecfg = EnvConfig::new();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&ecfg.evgen_db_url)
        .await
        .unwrap();

    let transaction_id = next_random_handle(DEF_TYPE);

    let zk_data = ZkData {
        contract_address: contract_address.to_owned(),
        user_address: user_address.to_owned(),
        acl_contract_address: ecfg.acl_contract_address,
        chain_id: ecfg.chain_id,
    };
    let aux_data = zk_data.to_owned().assemble()?;

    let (pks, public_params) = query_and_save_pks(&pool).await?;

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&pks);
    for _ in 0..count {
        // TODO: we default to u64s here
        builder.push(rand::rng().random::<u64>());
    }

    info!(target: "tool", "ZK Transaction: tx_id: {:?}, inputs = {:?}", to_hex(transaction_id.as_ref()), count);

    let the_list = builder
        .build_with_proof_packed(&public_params, &aux_data, tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();
    let zk_pok = fhevm_engine_common::utils::safe_serialize(&the_list);
    let zk_id = ZK_PROOF_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let mut db_tx = pool.begin().await?;

    insert_proof(
        &mut db_tx,
        zk_id,
        zk_pok,
        zk_data,
        &ctx.args.zkproof_notify_channel,
        transaction_id,
        0,
        0,
    )
    .await?;

    db_tx.commit().await?;

    info!(zk_id, count, "waiting for verification...");
    let handles = wait_for_verification_and_handle(&pool, zk_id, 5000).await?;
    info!(handles = ?handles.iter().map(hex::encode), count = handles.len(), "received handles");

    Ok(handles)
}

pub async fn generate_and_insert_inputs_batch(
    ctx: &Context,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    listener_event_to_db: &ListenerDatabase,
    batch_size: usize,
    inputs_count: u8,
    contract_address: &String,
    user_address: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(inputs_count <= 254);
    let ecfg = EnvConfig::new();
    let pool = pool(listener_event_to_db).await;

    let (pks, public_params) = query_and_save_pks(&pool).await?;

    // Generate a batch of zkpoks
    for idx in 0..batch_size {
        let transaction_id = new_transaction_id();

        let zk_data = ZkData {
            contract_address: contract_address.to_owned(),
            user_address: user_address.to_owned(),
            acl_contract_address: ecfg.acl_contract_address.clone(),
            chain_id: ecfg.chain_id,
        };
        let aux_data = zk_data.to_owned().assemble()?;

        let mut builder = tfhe::ProvenCompactCiphertextList::builder(&pks);
        for _ in 0..inputs_count {
            builder.push(rand::rng().random::<u64>());
        }
        let zk_id = ZK_PROOF_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        info!(target: "tool", "zkpok id: {}, count = {:?}, seq_num = {} of {} , txn: {:?}, ", zk_id, inputs_count, idx, batch_size, transaction_id );
        let the_list = builder
            .build_with_proof_packed(&public_params, &aux_data, tfhe::zk::ZkComputeLoad::Proof)
            .unwrap();

        let zk_pok = fhevm_engine_common::utils::safe_serialize(&the_list);

        let retry_count = 5;
        // retry_count = 5 to ensure the txn-sender will delete it after first try
        // If not deleted, txn-sender will report too many VerifyProofNotRequested errors
        // In devnet, verify_proof_resp_max_retries: 6,
        insert_proof(
            tx,
            zk_id,
            zk_pok.clone(),
            zk_data,
            &ctx.args.zkproof_notify_channel,
            transaction_id,
            retry_count,
            0,
        )
        .await?;
    }

    Ok(())
}

pub async fn get_inputs_vector(
    ctx: &Context,
    in_type: Inputs,
    contract_address: &String,
    user_address: &String,
) -> Result<Vec<Option<Handle>>, Box<dyn std::error::Error>> {
    if in_type == Inputs::NA {
        return Ok(vec![]);
    }
    if in_type == Inputs::NewInputs {
        return Ok(vec![None; CACHED_INPUTS_COUNT as usize]);
    }

    let contract_inputs = CONTRACT_INPUTS
        .read()
        .await
        .get(&(contract_address.to_owned(), user_address.to_owned()))
        .cloned();

    if let Some(contract_inputs) = contract_inputs {
        Ok(contract_inputs.to_owned())
    } else {
        let count = CACHED_INPUTS_COUNT;
        info!(count, "No cached inputs found, generating new ones");

        let inputs = generate_random_handle_vec(ctx, count, contract_address, user_address)
            .await?
            .into_iter()
            .map(Some)
            .collect::<Vec<Option<Handle>>>();

        info!(contract_address = %contract_address, user_address = %user_address, "Inserting new contract inputs into cache");
        CONTRACT_INPUTS.write().await.insert(
            (contract_address.to_owned(), user_address.to_owned()),
            inputs.to_owned(),
        );

        info!(inputs = ?inputs, "Generated new contract inputs");
        Ok(inputs)
    }
}

pub async fn generate_input_verification_transaction(
    ctx: &Context,
    count: u32,
    batch_size: u8,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    for _ in 0..count {
        generate_random_handle_vec(ctx, batch_size, contract_address, user_address).await?;
    }
    Ok((next_random_handle(DEF_TYPE), next_random_handle(DEF_TYPE)))
}
