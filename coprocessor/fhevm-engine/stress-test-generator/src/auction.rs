use std::collections::BTreeMap;
use std::sync::{Arc, OnceLock};

use fhevm_engine_common::types::AllowEvents;
use host_listener::contracts::{TfheContract, TfheContract::TfheContractEvents};
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle, ScalarByte,
};
use tokio::sync::RwLock;
use tracing::info;

use crate::utils::{
    allow_handle, generate_trivial_encrypt, insert_tfhe_event, new_transaction_id,
    next_random_handle, tfhe_event, Context, FheType, DEF_TYPE,
};

#[derive(Clone, Copy)]
pub struct BidEntry {
    pub e_amount: Handle,
    pub e_paid: Handle,
    pub price: u64,
}

pub struct ContractState {
    pub bids_submitted: Vec<BidEntry>,
    pub e_total_requested_amount_by_price: BTreeMap<u64, Handle>,
    pub e_requested_amount_by_token_and_price: BTreeMap<u64, Handle>,
}

pub static CONTRACT_STATE: OnceLock<Arc<RwLock<ContractState>>> = OnceLock::new();

#[allow(clippy::too_many_arguments)]
pub async fn batch_submit_encrypted_bids(
    ctx: &Context,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    listener_event_to_db: &ListenerDatabase,
    transaction_id: Option<Handle>,
    user_address: &str,
    payment_token_address: &str,
    bids: &[Option<Handle>],
) -> Result<Handle, Box<dyn std::error::Error>> {
    let caller = user_address.parse().unwrap();
    let transaction_id = transaction_id.unwrap_or(new_transaction_id());

    let _state = CONTRACT_STATE.get_or_init(|| {
        Arc::new(RwLock::new(ContractState {
            bids_submitted: Vec::new(),
            e_total_requested_amount_by_price: BTreeMap::new(),
            e_requested_amount_by_token_and_price: BTreeMap::new(),
        }))
    });

    // euint64 eTotalPaymentValue = FHE.asEuint64(0);
    let mut e_total_payment_value = generate_trivial_encrypt(
        tx,
        user_address,
        user_address,
        transaction_id,
        listener_event_to_db,
        Some(DEF_TYPE),
        Some(0),
        false,
    )
    .await?;

    let mut user_submitted_bids = vec![];

    for e_amount in bids.iter() {
        let bid_price = 1;

        let (e_paid, e_amount, price) = process_bid_entry(
            ctx,
            tx,
            e_amount.expect("should be a valid bid"),
            bid_price,
            transaction_id,
            listener_event_to_db,
            user_address,
        )
        .await?;

        /*
        eTotalPaymentValue = FHE.add(
               eTotalPaymentValue,
               _processBidEntry(eAmount, _inputBids[i].price, _paymentTokenAddress)
           );
        */
        let result_handle = next_random_handle(DEF_TYPE);
        let event = tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller,
            lhs: e_total_payment_value,
            rhs: e_paid,
            result: result_handle,
            scalarByte: ScalarByte::from(false as u8),
        }));
        insert_tfhe_event(tx, listener_event_to_db, transaction_id, event, false).await?;
        e_total_payment_value = result_handle;

        user_submitted_bids.push(BidEntry {
            e_amount,
            e_paid,
            price,
        });
    }

    let e_is_payment_confirmed = process_batch_payment(
        ctx,
        tx,
        transaction_id,
        listener_event_to_db,
        user_address,
        payment_token_address,
        e_total_payment_value,
    )
    .await?;

    // Confirm and finalize each bid based on the payment result
    for bid_entry in user_submitted_bids.iter() {
        confirm_and_finalize_bid(
            tx,
            transaction_id,
            listener_event_to_db,
            bid_entry,
            user_address,
            e_is_payment_confirmed,
            payment_token_address,
        )
        .await?;
    }

    Ok(e_is_payment_confirmed)
}
//
//    eAmount = FHE.select(
//        FHE.le(eAmount, FHE.asEuint64(auctionConfig.zamaTokenSupply)),
//        eAmount,
//        FHE.asEuint64(0),
//    );
//   ePaid = FHE.mul(eAmount, FHE.asEuint64(price));
//
#[allow(clippy::too_many_arguments)]
pub async fn process_bid_entry(
    _ctx: &Context,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mut e_amount: Handle,
    price: u64,
    transaction_id: Handle,
    listener_event_to_db: &ListenerDatabase,
    user_address: &str,
) -> Result<(Handle, Handle, u64), Box<dyn std::error::Error>> {
    let caller = user_address.parse().unwrap();
    info!(target: "tool", "Process Bid Entry: tx_id: {:?}", transaction_id);

    let total_supply = generate_trivial_encrypt(
        tx,
        user_address,
        user_address,
        transaction_id,
        listener_event_to_db,
        Some(DEF_TYPE),
        Some(1_000_000),
        false,
    )
    .await?;

    let less_than_total_supply = next_random_handle(FheType::FheBool);
    let event = tfhe_event(TfheContractEvents::FheLe(TfheContract::FheLe {
        caller,
        lhs: e_amount,
        rhs: total_supply,
        result: less_than_total_supply,
        scalarByte: ScalarByte::from(false as u8),
    }));
    insert_tfhe_event(tx, listener_event_to_db, transaction_id, event, false).await?;

    let zero = generate_trivial_encrypt(
        tx,
        user_address,
        user_address,
        transaction_id,
        listener_event_to_db,
        Some(DEF_TYPE),
        Some(0),
        false,
    )
    .await?;

    let result_handle = next_random_handle(DEF_TYPE);
    let event = tfhe_event(TfheContractEvents::FheIfThenElse(
        TfheContract::FheIfThenElse {
            caller,
            control: less_than_total_supply,
            ifTrue: e_amount,
            ifFalse: zero,
            result: result_handle,
        },
    ));

    insert_tfhe_event(tx, listener_event_to_db, transaction_id, event, false).await?;
    e_amount = result_handle;

    let e_price = generate_trivial_encrypt(
        tx,
        user_address,
        user_address,
        transaction_id,
        listener_event_to_db,
        Some(DEF_TYPE),
        Some(price as u128),
        false,
    )
    .await?;

    let e_paid = next_random_handle(DEF_TYPE);

    let event = tfhe_event(TfheContractEvents::FheMul(TfheContract::FheMul {
        caller,
        lhs: e_amount,
        rhs: e_price,
        result: e_paid,
        scalarByte: ScalarByte::from(false as u8),
    }));
    insert_tfhe_event(tx, listener_event_to_db, transaction_id, event, false).await?;

    Ok((e_paid, e_amount, price))
}

// euint64 eTotalPaid = IERC7984(paymentTokenAddress).confidentialTransferFrom(
//           msg.sender,
//             address(this),
//           eTotalValue
//       );
//     FHE.allow(eTotalPaid, auctionConfig.complianceAddress);
//     eIsPaymentConfirmed = FHE.eq(eTotalPaid, eTotalValue);
pub async fn process_batch_payment(
    ctx: &Context,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    transaction_id: Handle,
    listener_event_to_db: &ListenerDatabase,
    user_address: &str,
    payment_token_address: &str,
    e_total_value: Handle,
) -> Result<Handle, Box<dyn std::error::Error>> {
    let caller = user_address.parse().unwrap();
    info!(target: "tool", "Process Batch Payment: tx_id: {:?}", transaction_id);

    let e_total_paid = crate::erc7984::confidential_transfer_from(
        ctx,
        tx,
        transaction_id,
        listener_event_to_db,
        e_total_value,
        user_address,
    )
    .await?;

    allow_handle(
        tx,
        &e_total_paid.to_vec(),
        AllowEvents::AllowedAccount,
        payment_token_address.to_string(),
        transaction_id,
    )
    .await?;

    let e_is_payment_confirmed = next_random_handle(FheType::FheBool);
    let event = tfhe_event(TfheContractEvents::FheEq(TfheContract::FheEq {
        caller,
        lhs: e_total_paid,
        rhs: e_total_value,
        result: e_is_payment_confirmed,
        scalarByte: ScalarByte::from(false as u8),
    }));
    insert_tfhe_event(tx, listener_event_to_db, transaction_id, event, false).await?;

    Ok(e_is_payment_confirmed)
}

pub async fn confirm_and_finalize_bid(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    transaction_id: Handle,
    listener_event_to_db: &ListenerDatabase,
    bid_entry: &BidEntry,
    user_address: &str,
    e_is_payment_confirmed: Handle,
    _payment_token_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bid_entry = *bid_entry;
    let caller = user_address.parse().unwrap();
    info!(target: "tool", "Confirm and Finalize Bid: tx_id: {:?}", transaction_id);

    let zero = generate_trivial_encrypt(
        tx,
        user_address,
        user_address,
        transaction_id,
        listener_event_to_db,
        Some(DEF_TYPE),
        Some(0),
        false,
    )
    .await?;

    /*
     Bid storage _bid = _bids[bidId];
    _bid.eAmount = FHE.select(eIsPaymentConfirmed, _bid.eAmount, FHE.asEuint64(0));
    _bid.ePaid = FHE.select(eIsPaymentConfirmed, _bid.ePaid, FHE.asEuint64(0));
    */

    let result_handle = next_random_handle(DEF_TYPE);
    let event = tfhe_event(TfheContractEvents::FheIfThenElse(
        TfheContract::FheIfThenElse {
            caller,
            control: e_is_payment_confirmed,
            ifTrue: bid_entry.e_amount,
            ifFalse: zero,
            result: result_handle,
        },
    ));
    insert_tfhe_event(tx, listener_event_to_db, transaction_id, event, true).await?;
    bid_entry.e_amount = result_handle;

    let result_handle = next_random_handle(DEF_TYPE);
    let event = tfhe_event(TfheContractEvents::FheIfThenElse(
        TfheContract::FheIfThenElse {
            caller,
            control: e_is_payment_confirmed,
            ifTrue: bid_entry.e_paid,
            ifFalse: zero,
            result: result_handle,
        },
    ));
    insert_tfhe_event(tx, listener_event_to_db, transaction_id, event, true).await?;
    bid_entry.e_paid = result_handle;

    // Update contract state

    //    FHE.allowThis(updatedTotalAmountByTokenPrice);
    {
        let mut state_guard = CONTRACT_STATE.get().unwrap().write().await;

        //  euint64 updatedTotalAmount = _eTotalRequestedAmountByPrice[_bid.price];
        // updatedTotalAmount = FHE.isInitialized(updatedTotalAmount)
        //    ? FHE.add(updatedTotalAmount, _bid.eAmount)
        //    : _bid.eAmount;
        // _eTotalRequestedAmountByPrice[_bid.price] = updatedTotalAmount;

        let mut event = None;
        state_guard
            .e_total_requested_amount_by_price
            .entry(bid_entry.price)
            .and_modify(|updated_total_amount| {
                let result_handle = next_random_handle(DEF_TYPE);
                event = Some(tfhe_event(TfheContractEvents::FheAdd(
                    TfheContract::FheAdd {
                        caller,
                        lhs: *updated_total_amount,
                        rhs: bid_entry.e_amount,
                        result: result_handle,
                        scalarByte: ScalarByte::from(false as u8),
                    },
                )));

                info!(target: "tool", "modify e_total_requested_amount_by_price, handle: {:?}", hex::encode(result_handle));

                *updated_total_amount = result_handle;
            })
            .or_insert_with(|| bid_entry.e_amount);

        if let Some(event) = event {
            insert_tfhe_event(tx, listener_event_to_db, transaction_id, event, true)
                .await
                .unwrap();
        }

        let updated_total_amount = state_guard
            .e_total_requested_amount_by_price
            .get(&bid_entry.price)
            .expect("should be valid handle")
            .to_vec();

        //    FHE.allowThis(updatedTotalAmount);
        allow_handle(
            tx,
            &updated_total_amount,
            AllowEvents::AllowedAccount,
            user_address.to_string(),
            transaction_id,
        )
        .await?;

        //   Tracks the amount of tokens per payment token and price level to determine the amount to send to the treasury
        //   euint64 updatedTotalAmountByTokenPrice = _eRequestedAmountByTokenAndPrice[paymentTokenAddress][_bid.price];
        //   updatedTotalAmountByTokenPrice = FHE.isInitialized(updatedTotalAmountByTokenPrice)
        //      ? FHE.add(updatedTotalAmountByTokenPrice, _bid.eAmount)
        //     : _bid.eAmount;
        //  _eRequestedAmountByTokenAndPrice[paymentTokenAddress][_bid.price] = updatedTotalAmountByTokenPrice;

        let mut event = None;
        state_guard
            .e_requested_amount_by_token_and_price
            .entry(bid_entry.price)
            .and_modify(|updated_total_amount_by_token_price| {
                let result_handle = next_random_handle(DEF_TYPE);
                event = Some(tfhe_event(TfheContractEvents::FheAdd(
                    TfheContract::FheAdd {
                        caller,
                        lhs: *updated_total_amount_by_token_price,
                        rhs: bid_entry.e_amount,
                        result: result_handle,
                        scalarByte: ScalarByte::from(false as u8),
                    },
                )));

                info!(target: "tool", "modify e_requested_amount_by_token_and_price, handle: {:?}", hex::encode(result_handle));

                *updated_total_amount_by_token_price = result_handle;
            })
            .or_insert_with(|| bid_entry.e_amount);

        if let Some(event) = event {
            insert_tfhe_event(tx, listener_event_to_db, transaction_id, event, true)
                .await
                .unwrap();
        }

        let updated_total_amount_by_token_price = state_guard
            .e_requested_amount_by_token_and_price
            .get(&bid_entry.price)
            .expect("should be valid handle")
            .to_vec();

        // FHE.allowThis(updatedTotalAmountByTokenPrice);
        allow_handle(
            tx,
            &updated_total_amount_by_token_price,
            AllowEvents::AllowedAccount,
            user_address.to_string(),
            transaction_id,
        )
        .await?;
    }

    /*
       FHE.allow(_bid.eAmount, msg.sender);
       FHE.allow(_bid.ePaid, msg.sender);
       FHE.allow(_bid.eAmount, auctionConfig.complianceAddress);
       FHE.allow(_bid.ePaid, auctionConfig.complianceAddress);
    */
    allow_handle(
        tx,
        bid_entry.e_amount.to_vec().as_ref(),
        AllowEvents::AllowedAccount,
        user_address.to_string(),
        transaction_id,
    )
    .await?;

    allow_handle(
        tx,
        bid_entry.e_paid.to_vec().as_ref(),
        AllowEvents::AllowedAccount,
        user_address.to_string(),
        transaction_id,
    )
    .await?;

    Ok(())
}
