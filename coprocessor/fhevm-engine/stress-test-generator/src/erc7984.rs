use crate::utils::{
    allow_handle, generate_trivial_encrypt, insert_tfhe_event, next_random_handle, pool,
    tfhe_event, Context, FheType, DEF_TYPE,
};
use alloy_primitives::Address;
use fhevm_engine_common::types::AllowEvents;
use host_listener::{
    contracts::TfheContract::{self, TfheContractEvents},
    database::tfhe_event_propagate::{Database as ListenerDatabase, Handle, ScalarByte},
};

/// Implements ERC-7984's confidential transfer function
/// see also: github.com/OpenZeppelin/openzeppelin-confidential-contracts/blob/master/contracts/token/ERC7984/ERC7984.sol
pub async fn confidential_transfer_from(
    ctx: &Context,
    transaction_id: Handle,
    db: &mut ListenerDatabase,
    e_amount: Handle,
    user_address: &str,
) -> Result<Handle, Box<dyn std::error::Error>> {
    let caller: Address = user_address.parse().unwrap();
    let pool = pool(db).await;

    let balance_from = ctx
        .inputs_pool
        .first()
        .unwrap()
        .expect("should be at least one input available");

    let balance_to = ctx
        .inputs_pool
        .get(1)
        .unwrap()
        .expect("should be at least two inputs available");

    /*
       euint64 fromBalance = _balances[from];
       require(FHE.isInitialized(fromBalance), ERC7984ZeroBalance(from));
       (success, ptr) = FHESafeMath.tryDecrease(fromBalance, amount);
       FHE.allowThis(ptr);
       FHE.allow(ptr, from);
       _balances[from] = ptr;
    */

    let (success, ptr) = try_decrease(db, caller, transaction_id, balance_from, e_amount).await?;

    allow_handle(
        &ptr.to_vec(),
        AllowEvents::AllowedAccount,
        user_address.to_string(),
        transaction_id,
        &pool,
    )
    .await?;

    let zero = generate_trivial_encrypt(
        user_address,
        user_address,
        transaction_id,
        db,
        Some(DEF_TYPE),
        Some(0),
        false,
    )
    .await?;

    // transferred = FHE.select(success, amount, FHE.asEuint64(0));

    let transferred = next_random_handle(DEF_TYPE);
    let event = tfhe_event(TfheContractEvents::FheIfThenElse(
        TfheContract::FheIfThenElse {
            caller,
            control: success,
            ifTrue: e_amount,
            ifFalse: zero,
            result: transferred,
        },
    ));
    insert_tfhe_event(db, transaction_id, event, true).await?;

    /*

        ptr = FHE.add(_balances[to], transferred);
        FHE.allowThis(ptr);
        FHE.allow(ptr, to);
        _balances[to] = ptr;
    */

    let ptr = next_random_handle(DEF_TYPE);
    let event = tfhe_event(TfheContractEvents::FheAdd(TfheContract::FheAdd {
        caller,
        lhs: balance_to,
        rhs: transferred,
        result: ptr,
        scalarByte: ScalarByte::from(false as u8),
    }));
    insert_tfhe_event(db, transaction_id, event, true).await?;

    allow_handle(
        &ptr.to_vec(),
        AllowEvents::AllowedForDecryption,
        user_address.to_string(),
        transaction_id,
        &pool,
    )
    .await?;

    /*
    if (from != address(0)) FHE.allow(transferred, from);
       if (to != address(0)) FHE.allow(transferred, to);
       FHE.allowThis(transferred);
       emit ConfidentialTransfer(from, to, transferred);
    */

    allow_handle(
        &transferred.to_vec(),
        AllowEvents::AllowedAccount,
        user_address.to_string(),
        transaction_id,
        &pool,
    )
    .await?;

    Ok(transferred)
}

/*
 function tryDecrease(euint64 oldValue, euint64 delta) internal returns (ebool success, euint64 updated) {
    if (!FHE.isInitialized(oldValue)) {
        if (!FHE.isInitialized(delta)) {
            return (FHE.asEbool(true), oldValue);
        }
        return (FHE.eq(delta, 0), FHE.asEuint64(0));
    }
    success = FHE.ge(oldValue, delta);
    updated = FHE.select(success, FHE.sub(oldValue, delta), oldValue);
}
*/
pub async fn try_decrease(
    db: &mut ListenerDatabase,
    caller: Address,
    transaction_id: Handle,
    old_value: Handle,
    delta: Handle,
) -> Result<(Handle, Handle), Box<dyn std::error::Error>> {
    let success = next_random_handle(FheType::FheBool);
    let event = tfhe_event(TfheContractEvents::FheGe(TfheContract::FheGe {
        caller,
        lhs: old_value,
        rhs: delta,
        result: success,
        scalarByte: ScalarByte::from(false as u8),
    }));
    insert_tfhe_event(db, transaction_id, event, false).await?;

    let result_handle = next_random_handle(DEF_TYPE);

    let event = tfhe_event(TfheContractEvents::FheSub(TfheContract::FheSub {
        caller,
        lhs: old_value,
        rhs: delta,
        result: result_handle,
        scalarByte: ScalarByte::from(false as u8),
    }));
    insert_tfhe_event(db, transaction_id, event, false).await?;

    let updated = next_random_handle(DEF_TYPE);
    let event = tfhe_event(TfheContractEvents::FheIfThenElse(
        TfheContract::FheIfThenElse {
            caller,
            control: success,
            ifTrue: result_handle,
            ifFalse: old_value,
            result: updated,
        },
    ));
    insert_tfhe_event(db, transaction_id, event, true).await?;

    Ok((success, updated))
}
