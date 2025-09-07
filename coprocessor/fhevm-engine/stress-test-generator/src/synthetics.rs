use crate::utils::{
    allow_handle, generate_trivial_encrypt, next_random_handle, tfhe_event, EnvConfig, FheType,
    DEF_TYPE,
};
use crate::zk_gen::generate_random_handle_amount_if_none;
use fhevm_engine_common::types::AllowEvents;
use host_listener::contracts::{TfheContract, TfheContract::TfheContractEvents};
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle, ScalarByte,
};
use sqlx::Postgres;
use std::io::prelude::*;

#[allow(clippy::too_many_arguments)]
pub async fn add_chain_transaction(
    counter: Option<Handle>,
    amount: Option<Handle>,
    length: u32,
    transaction_id: Option<Handle>,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let caller = user_address.parse().unwrap();
    let transaction_id = transaction_id.unwrap_or_else(|| next_random_handle(DEF_TYPE));
    let mut counter =
        generate_random_handle_amount_if_none(counter, contract_address, user_address).await?;

    let amount = match amount {
        Some(amount) => amount,
        None => {
            generate_trivial_encrypt(
                contract_address,
                contract_address,
                transaction_id,
                listener_event_to_db,
                Some(DEF_TYPE),
                None,
            )
            .await?
        }
    };

    for _ in 0..length {
        let new_counter = next_random_handle(FheType::FheUint64);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: counter,
                rhs: amount,
                result: new_counter,
                scalarByte: ScalarByte::from(false as u8),
            })),
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_id),
            transaction_index: Some(0),
            log_index: None,
            removed: false,
        };
        listener_event_to_db.insert_tfhe_event(&log).await?;
        counter = new_counter;
    }
    allow_handle(
        &counter.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    Ok((counter, counter))
}

#[allow(clippy::too_many_arguments)]
pub async fn mul_chain_transaction(
    counter: Option<Handle>,
    amount: Option<Handle>,
    length: u32,
    transaction_id: Option<Handle>,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let caller = user_address.parse().unwrap();
    let transaction_id = transaction_id.unwrap_or_else(|| next_random_handle(DEF_TYPE));
    let mut counter =
        generate_random_handle_amount_if_none(counter, contract_address, user_address).await?;

    let amount = match amount {
        Some(amount) => amount,
        None => {
            generate_trivial_encrypt(
                contract_address,
                contract_address,
                transaction_id,
                listener_event_to_db,
                Some(DEF_TYPE),
                None,
            )
            .await?
        }
    };

    for _ in 0..length {
        let new_counter = next_random_handle(FheType::FheUint64);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::FheMul(TfheContract::FheMul {
                caller,
                lhs: counter,
                rhs: amount,
                result: new_counter,
                scalarByte: ScalarByte::from(false as u8),
            })),
            block_hash: None,
            block_number: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_id),
            transaction_index: Some(0),
            log_index: None,
            removed: false,
        };
        listener_event_to_db.insert_tfhe_event(&log).await?;
        counter = new_counter;
    }
    allow_handle(
        &counter.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    Ok((counter, counter))
}

#[allow(clippy::too_many_arguments)]
pub async fn generate_pub_decrypt_handles_types(
    min_type: u8,
    max_type: u8,
    transaction_id: Option<Handle>,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
    contract_address: &str,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let ecfg = EnvConfig::new();
    let mut out_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(ecfg.output_handles_for_pub_decryption)
        .unwrap();
    let transaction_id = transaction_id.unwrap_or_else(|| next_random_handle(DEF_TYPE));
    let mut handle = next_random_handle(DEF_TYPE);
    for type_num in min_type..=max_type {
        handle = generate_trivial_encrypt(
            contract_address,
            user_address,
            transaction_id,
            listener_event_to_db,
            Some(type_num.into()),
            Some(type_num.into()),
        )
        .await?;
        allow_handle(
            &handle.to_vec(),
            AllowEvents::AllowedForDecryption,
            user_address.to_string(),
            pool,
        )
        .await?;
        writeln!(out_file, "{}", "0x".to_owned() + &hex::encode(handle))?;
    }
    Ok((handle, handle))
}

#[allow(clippy::too_many_arguments)]
pub async fn generate_user_decrypt_handles_types(
    min_type: u8,
    max_type: u8,
    transaction_id: Option<Handle>,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
    contract_address: &str,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let ecfg = EnvConfig::new();
    let mut out_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&ecfg.output_handles_for_usr_decryption)
        .unwrap();
    let transaction_id = transaction_id.unwrap_or_else(|| next_random_handle(DEF_TYPE));
    let mut handle = next_random_handle(DEF_TYPE);
    for type_num in min_type..=max_type {
        handle = generate_trivial_encrypt(
            contract_address,
            user_address,
            transaction_id,
            listener_event_to_db,
            Some(type_num.into()),
            Some(type_num.into()),
        )
        .await?;
        allow_handle(
            &handle.to_vec(),
            AllowEvents::AllowedAccount,
            contract_address.to_string(),
            pool,
        )
        .await?;
        allow_handle(
            &handle.to_vec(),
            AllowEvents::AllowedAccount,
            user_address.to_string(),
            pool,
        )
        .await?;
        writeln!(out_file, "{}", "0x".to_owned() + &hex::encode(handle))?;
    }
    Ok((handle, handle))
}
