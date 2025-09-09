use crate::utils::{next_random_handle, query_and_save_pks, EnvConfig, Inputs, DEF_TYPE};
use host_listener::database::tfhe_event_propagate::Handle;
use rand::Rng;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info};

const SIZE: usize = 92;

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
    pub chain_id: i64,
}

impl ZkData {
    pub fn assemble(&self) -> anyhow::Result<[u8; SIZE]> {
        let contract_bytes =
            alloy_primitives::Address::from_str(&self.contract_address)?.into_array();
        let user_bytes = alloy_primitives::Address::from_str(&self.user_address)?.into_array();
        let acl_bytes =
            alloy_primitives::Address::from_str(&self.acl_contract_address)?.into_array();
        let chain_id_bytes: [u8; 32] = alloy_primitives::U256::from(self.chain_id)
            .to_owned()
            .to_be_bytes();

        // Copy contract address into the first 20 bytes
        let front: Vec<u8> = [contract_bytes, user_bytes, acl_bytes].concat();
        let mut data = [0_u8; SIZE];
        data[..60].copy_from_slice(front.as_slice());
        data[60..].copy_from_slice(&chain_id_bytes);
        Ok(data)
    }
}

async fn insert_proof(
    pool: &sqlx::PgPool,
    request_id: i64,
    zk_pok: &[u8],
    aux: &ZkData,
) -> Result<(), sqlx::Error> {
    //  Insert ZkPok into database
    sqlx::query(
            "INSERT INTO verify_proofs (zk_proof_id, input, chain_id, contract_address, user_address, verified)
            VALUES ($1, $2, $3, $4, $5, NULL )" 
        ).bind(request_id)
        .bind(zk_pok)
        .bind(aux.chain_id)
        .bind(aux.contract_address.clone())
        .bind(aux.user_address.clone())
        .execute(pool).await?;
    sqlx::query("SELECT pg_notify($1, '')")
        .bind("fhevm")
        .execute(pool)
        .await
        .unwrap();
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
    result: Option<Handle>,
    contract_address: &String,
    user_address: &String,
) -> Result<Handle, Box<dyn std::error::Error>> {
    if let Some(res) = result {
        return Ok(res);
    }
    Ok(generate_random_handle_vec(1, contract_address, user_address).await?[0])
}

pub async fn generate_random_handle_vec(
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

    let zk_data = ZkData {
        contract_address: contract_address.to_owned(),
        user_address: user_address.to_owned(),
        acl_contract_address: ecfg.acl_contract_address,
        chain_id: ecfg.chain_id,
    };
    let aux_data = zk_data.to_owned().assemble()?;

    let (pks, public_params) = query_and_save_pks(ecfg.tenant_id, &pool).await?;

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&pks);
    for _ in 0..count {
        // TODO: we default to u64s here
        builder.push(rand::rng().random::<u64>());
    }
    let the_list = builder
        .build_with_proof_packed(&public_params, &aux_data, tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();
    let zk_pok = fhevm_engine_common::utils::safe_serialize(&the_list);
    let zk_id = ZK_PROOF_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    insert_proof(&pool, zk_id, &zk_pok, &zk_data).await?;

    info!(zk_id, count, "waiting for verification...");
    let handles = wait_for_verification_and_handle(&pool, zk_id, 5000).await?;
    info!(handles = ?handles.iter().map(hex::encode), count = handles.len(), "received handles");

    Ok(handles)
}

pub async fn get_inputs_vector(
    in_type: Inputs,
    contract_address: &String,
    user_address: &String,
) -> Result<Vec<Option<Handle>>, Box<dyn std::error::Error>> {
    if in_type == Inputs::NA {
        return Ok(vec![]);
    }
    if in_type == Inputs::NewInputs {
        return Ok(vec![None; 16]);
    }

    let contract_inputs = CONTRACT_INPUTS
        .read()
        .await
        .get(&(contract_address.to_owned(), user_address.to_owned()))
        .cloned();

    if let Some(contract_inputs) = contract_inputs {
        Ok(contract_inputs.to_owned())
    } else {
        let count = 16;
        info!(count, "No cached inputs found, generating new ones");

        let inputs = generate_random_handle_vec(count, contract_address, user_address)
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
    count: u32,
    batch_size: u8,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    for _ in 0..count {
        generate_random_handle_vec(batch_size, contract_address, user_address).await?;
    }
    Ok((next_random_handle(DEF_TYPE), next_random_handle(DEF_TYPE)))
}
