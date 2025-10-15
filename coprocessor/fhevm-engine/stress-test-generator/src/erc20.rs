use fhevm_engine_common::types::AllowEvents;
use host_listener::contracts::{TfheContract, TfheContract::TfheContractEvents};
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle, ScalarByte,
};
use sqlx::Postgres;
use tracing::{error, info};

use crate::utils::{
    allow_handle, insert_tfhe_event, next_random_handle, tfhe_event, Context, ERCTransferVariant,
    FheType, DEF_TYPE,
};
use crate::zk_gen::generate_random_handle_amount_if_none;

#[allow(clippy::too_many_arguments)]
pub async fn erc20_transaction(
    ctx: &Context,
    source: Option<Handle>,
    destination: Option<Handle>,
    amount: Option<Handle>,
    transaction_id: Option<Handle>,
    listener_event_to_db: &mut ListenerDatabase,
    pool: &sqlx::Pool<Postgres>,
    variant: ERCTransferVariant,
    contract_address: &String,
    user_address: &String,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let caller = user_address.parse().unwrap();
    let transaction_id = transaction_id.unwrap_or(next_random_handle(DEF_TYPE));

    info!(target: "tool", "ERC20 Transaction: tx_id: {:?}", transaction_id);

    let source =
        generate_random_handle_amount_if_none(ctx, source, contract_address, user_address).await?;

    info!(target: "tool", source = %source, "ERC20 Transfer");

    let destination =
        generate_random_handle_amount_if_none(ctx, destination, contract_address, user_address)
            .await?;

    info!(target: "tool", destination = %destination, "ERC20 Transfer");

    let amount =
        generate_random_handle_amount_if_none(ctx, amount, contract_address, user_address).await?;

    info!(target: "tool", "ERC20 Transfer: {} -> {}: {}", source, destination, amount);

    let has_enough_funds = next_random_handle(FheType::FheBool);
    let event = tfhe_event(TfheContractEvents::FheGe(TfheContract::FheGe {
        caller,
        lhs: source,
        rhs: amount,
        result: has_enough_funds,
        scalarByte: ScalarByte::from(false as u8),
    }));
    insert_tfhe_event(listener_event_to_db, transaction_id, event, false).await?;
    let new_source = next_random_handle(DEF_TYPE);
    let new_destination = next_random_handle(DEF_TYPE);
    match variant {
        ERCTransferVariant::Whitepaper => {
            let new_destination_target = next_random_handle(DEF_TYPE);
            let event = tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: destination,
                rhs: amount,
                result: new_destination_target,
                scalarByte: ScalarByte::from(false as u8),
            }));
            insert_tfhe_event(listener_event_to_db, transaction_id, event, false).await?;
            let event = tfhe_event(TfheContractEvents::FheIfThenElse(
                TfheContract::FheIfThenElse {
                    caller,
                    control: has_enough_funds,
                    ifTrue: new_destination_target,
                    ifFalse: destination,
                    result: new_destination,
                },
            ));
            insert_tfhe_event(listener_event_to_db, transaction_id, event, true).await?;
            allow_handle(
                &new_destination.to_vec(),
                AllowEvents::AllowedForDecryption,
                contract_address.to_string(),
                transaction_id,
                pool,
            )
            .await?;
            let new_source_target = next_random_handle(DEF_TYPE);
            let event = tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs: source,
                rhs: amount,
                result: new_source_target,
                scalarByte: ScalarByte::from(false as u8),
            }));
            insert_tfhe_event(listener_event_to_db, transaction_id, event, false).await?;
            let event = tfhe_event(TfheContractEvents::FheIfThenElse(
                TfheContract::FheIfThenElse {
                    caller,
                    control: has_enough_funds,
                    ifTrue: new_source_target,
                    ifFalse: source,
                    result: new_source,
                },
            ));
            insert_tfhe_event(listener_event_to_db, transaction_id, event, true).await?;
            allow_handle(
                &new_source.to_vec(),
                AllowEvents::AllowedForDecryption,
                contract_address.to_string(),
                transaction_id,
                pool,
            )
            .await?;
        }
        ERCTransferVariant::NoCMUX => {
            let cast_has_enough_funds = next_random_handle(DEF_TYPE);
            let event = tfhe_event(TfheContractEvents::Cast(TfheContract::Cast {
                caller,
                ct: has_enough_funds,
                toType: 5u8,
                result: cast_has_enough_funds,
            }));
            insert_tfhe_event(listener_event_to_db, transaction_id, event, false).await?;
            let select_amount = next_random_handle(DEF_TYPE);
            let event = tfhe_event(TfheContractEvents::FheMul(TfheContract::FheMul {
                caller,
                lhs: amount,
                rhs: cast_has_enough_funds,
                result: select_amount,
                scalarByte: ScalarByte::from(false as u8),
            }));
            insert_tfhe_event(listener_event_to_db, transaction_id, event, false).await?;
            let event = tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: destination,
                rhs: select_amount,
                result: new_destination,
                scalarByte: ScalarByte::from(false as u8),
            }));
            insert_tfhe_event(listener_event_to_db, transaction_id, event, true).await?;
            allow_handle(
                &new_destination.to_vec(),
                AllowEvents::AllowedForDecryption,
                contract_address.to_string(),
                transaction_id,
                pool,
            )
            .await?;
            let event = tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs: source,
                rhs: select_amount,
                result: new_source,
                scalarByte: ScalarByte::from(false as u8),
            }));
            insert_tfhe_event(listener_event_to_db, transaction_id, event, true).await?;
            allow_handle(
                &new_source.to_vec(),
                AllowEvents::AllowedForDecryption,
                contract_address.to_string(),
                transaction_id,
                pool,
            )
            .await?;
        }
        ERCTransferVariant::NA => {
            error!("ERC should have a variant");
        }
    }
    Ok((new_source, new_destination))
}
