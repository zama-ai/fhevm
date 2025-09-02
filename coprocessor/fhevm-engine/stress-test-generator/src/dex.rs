use alloy_primitives::Address;
use fhevm_engine_common::types::AllowEvents;
use host_listener::contracts::{TfheContract, TfheContract::TfheContractEvents};
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle, ScalarByte,
};
use sqlx::Postgres;

use crate::erc20::erc20_transaction;
use crate::utils::{
    allow_handle, generate_trivial_encrypt, next_random_handle, tfhe_event, ERCTransferVariant,
    DEF_TYPE,
};
use crate::zk_gen::generate_random_handle_amount_if_none;

#[allow(clippy::too_many_arguments)]
async fn dex_swap_request_update_dex_balance(
    from_balance: Option<Handle>,
    current_dex_balance: Option<Handle>,
    amount: Option<Handle>,
    transaction_id: Handle,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
    variant: ERCTransferVariant,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let caller: Address = user_address.parse().unwrap();
    let from_balance =
        generate_random_handle_amount_if_none(from_balance, contract_address, user_address).await?;
    let current_dex_balance =
        generate_random_handle_amount_if_none(current_dex_balance, contract_address, user_address)
            .await?;
    let amount =
        generate_random_handle_amount_if_none(amount, contract_address, user_address).await?;
    let (_, new_current_balance) = erc20_transaction(
        Some(from_balance),
        Some(current_dex_balance),
        Some(amount),
        Some(transaction_id),
        listener_event_to_db,
        pool,
        variant,
        contract_address,
        user_address,
    )
    .await?;
    let sent_amount = next_random_handle(DEF_TYPE);
    let log = alloy::rpc::types::Log {
        inner: tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
            caller,
            lhs: new_current_balance,
            rhs: current_dex_balance,
            result: sent_amount,
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
    Ok((sent_amount, new_current_balance))
}

#[allow(clippy::too_many_arguments)]
async fn dex_swap_request_finalize(
    to_balance: Option<Handle>,
    total_dex_token_in: Option<Handle>,
    sent: Option<Handle>,
    transaction_id: Handle,
    listener_event_to_db: &mut ListenerDatabase,
    _pool: &sqlx::Pool<Postgres>,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let caller: Address = user_address.parse().unwrap();
    let to_balance =
        generate_random_handle_amount_if_none(to_balance, contract_address, user_address).await?;
    let total_dex_token_in =
        generate_random_handle_amount_if_none(total_dex_token_in, contract_address, user_address)
            .await?;
    let sent = generate_random_handle_amount_if_none(sent, contract_address, user_address).await?;
    let pending_in = next_random_handle(DEF_TYPE);
    let log = alloy::rpc::types::Log {
        inner: tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller,
            lhs: to_balance,
            rhs: sent,
            result: pending_in,
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
    let pending_total_token_in = next_random_handle(DEF_TYPE);
    let log = alloy::rpc::types::Log {
        inner: tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller,
            lhs: total_dex_token_in,
            rhs: sent,
            result: pending_total_token_in,
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
    Ok((pending_in, pending_total_token_in))
}

#[allow(clippy::too_many_arguments)]
pub async fn dex_swap_request_transaction(
    from_balance_0: Option<Handle>,
    from_balance_1: Option<Handle>,
    current_balance_0: Option<Handle>,
    current_balance_1: Option<Handle>,
    to_balance_0: Option<Handle>,
    to_balance_1: Option<Handle>,
    total_token_0: Option<Handle>,
    total_token_1: Option<Handle>,
    amount_0: Option<Handle>,
    amount_1: Option<Handle>,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
    variant: ERCTransferVariant,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let transaction_id = next_random_handle(DEF_TYPE);
    let from_balance_0 =
        generate_random_handle_amount_if_none(from_balance_0, contract_address, user_address)
            .await?;
    let from_balance_1 =
        generate_random_handle_amount_if_none(from_balance_1, contract_address, user_address)
            .await?;
    let current_balance_0 =
        generate_random_handle_amount_if_none(current_balance_0, contract_address, user_address)
            .await?;
    let current_balance_1 =
        generate_random_handle_amount_if_none(current_balance_1, contract_address, user_address)
            .await?;
    let to_balance_0 =
        generate_random_handle_amount_if_none(to_balance_0, contract_address, user_address).await?;
    let to_balance_1 =
        generate_random_handle_amount_if_none(to_balance_1, contract_address, user_address).await?;
    let total_token_0 =
        generate_random_handle_amount_if_none(total_token_0, contract_address, user_address)
            .await?;
    let total_token_1 =
        generate_random_handle_amount_if_none(total_token_1, contract_address, user_address)
            .await?;
    let amount_0 =
        generate_random_handle_amount_if_none(amount_0, contract_address, user_address).await?;
    let amount_1 =
        generate_random_handle_amount_if_none(amount_1, contract_address, user_address).await?;

    let (sent_0, new_current_balance_0) = dex_swap_request_update_dex_balance(
        Some(from_balance_0),
        Some(current_balance_0),
        Some(amount_0),
        transaction_id,
        listener_event_to_db,
        pool,
        variant.to_owned(),
        contract_address,
        user_address,
    )
    .await?;
    let (sent_1, new_current_balance_1) = dex_swap_request_update_dex_balance(
        Some(from_balance_1),
        Some(current_balance_1),
        Some(amount_1),
        transaction_id,
        listener_event_to_db,
        pool,
        variant.to_owned(),
        contract_address,
        user_address,
    )
    .await?;

    let (pending_in_0, pending_total_token_in_0) = dex_swap_request_finalize(
        Some(to_balance_0),
        Some(total_token_0),
        Some(sent_0),
        transaction_id,
        listener_event_to_db,
        pool,
        contract_address,
        user_address,
    )
    .await?;
    let (pending_in_1, pending_total_token_in_1) = dex_swap_request_finalize(
        Some(to_balance_1),
        Some(total_token_1),
        Some(sent_1),
        transaction_id,
        listener_event_to_db,
        pool,
        contract_address,
        user_address,
    )
    .await?;
    allow_handle(
        &pending_in_0.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    allow_handle(
        &pending_in_1.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    allow_handle(
        &pending_total_token_in_0.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    allow_handle(
        &pending_total_token_in_1.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    allow_handle(
        &new_current_balance_0.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    allow_handle(
        &new_current_balance_1.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    Ok((new_current_balance_0, new_current_balance_1))
}

#[allow(clippy::too_many_arguments)]
async fn dex_swap_claim_prepare(
    pending_0_in: Option<Handle>,
    pending_1_in: Option<Handle>,
    total_dex_token_0_in: u64,
    total_dex_token_1_in: u64,
    total_dex_token_0_out: u64,
    total_dex_token_1_out: u64,
    transaction_id: Handle,
    listener_event_to_db: &mut ListenerDatabase,
    _pool: &sqlx::Pool<Postgres>,
    _variant: ERCTransferVariant,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let caller: Address = user_address.parse().unwrap();
    let pending_0_in =
        generate_random_handle_amount_if_none(pending_0_in, contract_address, user_address).await?;
    let pending_1_in =
        generate_random_handle_amount_if_none(pending_1_in, contract_address, user_address).await?;
    let mut amount_0_out = pending_1_in;
    let mut amount_1_out = pending_0_in;
    if total_dex_token_1_in != 0 {
        let big_pending_1_in = next_random_handle(crate::utils::FheType::FheUint128);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::Cast(TfheContract::Cast {
                caller,
                ct: pending_1_in,
                toType: crate::utils::FheType::FheUint128 as u8,
                result: big_pending_1_in,
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
        let total_dex_token_0_out_te = generate_trivial_encrypt(
            contract_address,
            user_address,
            transaction_id,
            listener_event_to_db,
            Some(crate::utils::FheType::FheUint128),
            Some(total_dex_token_0_out.into()),
        )
        .await?;
        let big_amount_0_out_mul = next_random_handle(crate::utils::FheType::FheUint128);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::FheMul(TfheContract::FheMul {
                caller,
                lhs: big_pending_1_in,
                rhs: total_dex_token_0_out_te,
                result: big_amount_0_out_mul,
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
        let total_dex_token_1_in_te = generate_trivial_encrypt(
            contract_address,
            user_address,
            transaction_id,
            listener_event_to_db,
            Some(crate::utils::FheType::FheUint128),
            Some(total_dex_token_1_in.into()),
        )
        .await?;
        let big_amount_0_out_div = next_random_handle(crate::utils::FheType::FheUint128);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::FheDiv(TfheContract::FheDiv {
                caller,
                lhs: big_amount_0_out_mul,
                rhs: total_dex_token_1_in_te,
                result: big_amount_0_out_div,
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
        amount_0_out = next_random_handle(crate::utils::FheType::FheUint64);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::Cast(TfheContract::Cast {
                caller,
                ct: big_amount_0_out_div,
                toType: crate::utils::FheType::FheUint64 as u8,
                result: amount_0_out,
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
    }
    if total_dex_token_0_in != 0 {
        let big_pending_0_in = next_random_handle(crate::utils::FheType::FheUint128);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::Cast(TfheContract::Cast {
                caller,
                ct: pending_0_in,
                toType: crate::utils::FheType::FheUint128 as u8,
                result: big_pending_0_in,
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
        let total_dex_token_1_out_te = generate_trivial_encrypt(
            contract_address,
            user_address,
            transaction_id,
            listener_event_to_db,
            Some(crate::utils::FheType::FheUint128),
            Some(total_dex_token_1_out.into()),
        )
        .await?;
        let big_amount_1_out_mul = next_random_handle(crate::utils::FheType::FheUint128);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::FheMul(TfheContract::FheMul {
                caller,
                lhs: big_pending_0_in,
                rhs: total_dex_token_1_out_te,
                result: big_amount_1_out_mul,
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
        let total_dex_token_0_in_te = generate_trivial_encrypt(
            contract_address,
            user_address,
            transaction_id,
            listener_event_to_db,
            Some(crate::utils::FheType::FheUint128),
            Some(total_dex_token_0_in.into()),
        )
        .await?;
        let big_amount_1_out_div = next_random_handle(crate::utils::FheType::FheUint128);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::FheDiv(TfheContract::FheDiv {
                caller,
                lhs: big_amount_1_out_mul,
                rhs: total_dex_token_0_in_te,
                result: big_amount_1_out_div,
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
        amount_1_out = next_random_handle(crate::utils::FheType::FheUint64);
        let log = alloy::rpc::types::Log {
            inner: tfhe_event(TfheContractEvents::Cast(TfheContract::Cast {
                caller,
                ct: big_amount_1_out_div,
                toType: crate::utils::FheType::FheUint64 as u8,
                result: amount_1_out,
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
    }
    Ok((amount_0_out, amount_1_out))
}

#[allow(clippy::too_many_arguments)]
async fn dex_swap_claim_update_dex_balance(
    amount_out: Option<Handle>,
    total_dex_other_token_in: u64,
    old_balance: Option<Handle>,
    current_dex_balance: Option<Handle>,
    transaction_id: Handle,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
    variant: ERCTransferVariant,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let amount_out =
        generate_random_handle_amount_if_none(amount_out, contract_address, user_address).await?;
    let old_balance =
        generate_random_handle_amount_if_none(old_balance, contract_address, user_address).await?;
    let current_dex_balance =
        generate_random_handle_amount_if_none(current_dex_balance, contract_address, user_address)
            .await?;
    let mut new_balance = old_balance;
    let mut new_dex_balance = current_dex_balance;
    if total_dex_other_token_in != 0 {
        (new_dex_balance, new_balance) = erc20_transaction(
            Some(current_dex_balance),
            Some(old_balance),
            Some(amount_out),
            Some(transaction_id),
            listener_event_to_db,
            pool,
            variant,
            contract_address,
            user_address,
        )
        .await?;
    }
    Ok((new_dex_balance, new_balance))
}

#[allow(clippy::too_many_arguments)]
pub async fn dex_swap_claim_transaction(
    pending_0_in: Option<Handle>,
    pending_1_in: Option<Handle>,
    total_token_0_in: u64,
    total_token_1_in: u64,
    total_token_0_out: u64,
    total_token_1_out: u64,
    old_balance_0: Option<Handle>,
    old_balance_1: Option<Handle>,
    current_balance_0: Option<Handle>,
    current_balance_1: Option<Handle>,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
    variant: ERCTransferVariant,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let transaction_id = next_random_handle(DEF_TYPE);
    let pending_0_in =
        generate_random_handle_amount_if_none(pending_0_in, contract_address, user_address).await?;
    let pending_1_in =
        generate_random_handle_amount_if_none(pending_1_in, contract_address, user_address).await?;
    let old_balance_0 =
        generate_random_handle_amount_if_none(old_balance_0, contract_address, user_address)
            .await?;
    let old_balance_1 =
        generate_random_handle_amount_if_none(old_balance_1, contract_address, user_address)
            .await?;
    let current_balance_0 =
        generate_random_handle_amount_if_none(current_balance_0, contract_address, user_address)
            .await?;
    let current_balance_1 =
        generate_random_handle_amount_if_none(current_balance_1, contract_address, user_address)
            .await?;

    let (amount_0_out, amount_1_out) = dex_swap_claim_prepare(
        Some(pending_0_in),
        Some(pending_1_in),
        total_token_0_in,
        total_token_1_in,
        total_token_0_out,
        total_token_1_out,
        transaction_id,
        listener_event_to_db,
        pool,
        variant.to_owned(),
        contract_address,
        user_address,
    )
    .await?;

    let (new_dex_balance_0, new_balance_0) = dex_swap_claim_update_dex_balance(
        Some(amount_0_out),
        total_token_1_in,
        Some(old_balance_0),
        Some(current_balance_0),
        transaction_id,
        listener_event_to_db,
        pool,
        variant.to_owned(),
        contract_address,
        user_address,
    )
    .await?;
    let (new_dex_balance_1, new_balance_1) = dex_swap_claim_update_dex_balance(
        Some(amount_1_out),
        total_token_0_in,
        Some(old_balance_1),
        Some(current_balance_1),
        transaction_id,
        listener_event_to_db,
        pool,
        variant.to_owned(),
        contract_address,
        user_address,
    )
    .await?;
    allow_handle(
        &new_balance_0.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    allow_handle(
        &new_balance_1.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    allow_handle(
        &new_dex_balance_0.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    allow_handle(
        &new_dex_balance_1.to_vec(),
        AllowEvents::AllowedForDecryption,
        contract_address.to_string(),
        pool,
    )
    .await?;
    Ok((new_dex_balance_0, new_dex_balance_1))
}
