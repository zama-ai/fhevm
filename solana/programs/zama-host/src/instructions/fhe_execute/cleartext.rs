//! Cleartext build: after the production execution, evaluates its steps on plaintexts and writes
//! each value next to the handle the execution recorded.
//!
//! Operand values are read before any is written, so a step reading a slot that an effect of the
//! same execution replaces sees the previous plaintext, as the handle check saw the previous handle.

use super::*;
use crate::cleartext::{self, layout, CleartextError, Value};

pub(super) fn record_execution<'info>(
    table: &mut ExecutionAccountTable<'_, 'info>,
    transient_store: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    call_start: usize,
    args: &FheExecuteArgs,
    random_seeds: &[FheExecuteRandomSeed],
) -> Result<()> {
    if transient_store.data_len() == TransientStore::SPACE {
        grow_account_if_needed(
            payer,
            transient_store,
            system_program,
            layout::TRANSIENT_ACCOUNT_SIZE,
        )?;
        layout::init_transient_tail(&mut transient_store.try_borrow_mut_data()?)?;
    }

    let values = {
        let data = transient_store.try_borrow_data()?;
        let results: &TransientStore = bytemuck::from_bytes(&data[8..TransientStore::SPACE]);
        cleartext::evaluate_steps(
            args,
            |operand, produced| {
                resolve_operand(table, results, &data, call_start, args, operand, produced)
            },
            |step_index| {
                random_seeds
                    .iter()
                    .find(|seed| seed.step_index == step_index)
                    .map(|seed| seed.seed)
                    .ok_or_else(|| error!(ZamaHostError::FheExecuteRandNonceMissing))
            },
        )?
    };

    let mut data = transient_store.try_borrow_mut_data()?;
    for (offset, value) in values.iter().enumerate() {
        layout::set_transient_value(&mut data, call_start + offset, *value)?;
    }
    let results: &TransientStore = bytemuck::from_bytes(&data[8..TransientStore::SPACE]);

    for effect in &args.effects {
        // As in production, an effect that only grants leaves its store untouched, and the store
        // may be read-only.
        if effect.slot.is_none() && effect.allow_indexes.is_empty() && !effect.make_public {
            continue;
        }
        let step = usize::from(effect.result.step_index);
        let handle = results
            .result(call_start + step)
            .ok_or(ZamaHostError::InvalidReturnSelection)?
            .handle;
        let slot_index = match &effect.slot {
            Some(slot) => {
                let key = dictionary_bytes(&args.dictionary, slot.key_index)?;
                Some(slot_index(table.state(effect.store_index.into())?, key)?)
            }
            None => None,
        };
        let mut store = table
            .account(effect.store_index.into())?
            .try_borrow_mut_data()?;
        if let Some(index) = slot_index {
            layout::set_store_value(&mut store, index, values[step])?;
        }
        layout::record_store_history(&mut store, handle, values[step])?;
    }
    Ok(())
}

fn resolve_operand(
    table: &mut ExecutionAccountTable<'_, '_>,
    results: &TransientStore,
    transient_data: &[u8],
    call_start: usize,
    args: &FheExecuteArgs,
    operand: &FheExecuteOperand,
    produced: &[Value],
) -> Result<Value> {
    match operand {
        FheExecuteOperand::StoreSlot {
            handle_index,
            store_index,
            key_index,
        } => {
            let handle = dictionary_bytes(&args.dictionary, *handle_index)?;
            let key = dictionary_bytes(&args.dictionary, *key_index)?;
            let index = slot_index(table.state((*store_index).into())?, key)?;
            let data = table.account((*store_index).into())?.try_borrow_data()?;
            layout::store_value(&data, index, handle)
        }
        FheExecuteOperand::TransientResult { handle_index, .. } => {
            let handle = dictionary_bytes(&args.dictionary, *handle_index)?;
            let index = (0..results.len())
                .find(|index| results.result(*index).map(|result| result.handle) == Some(handle))
                .ok_or_else(|| error!(CleartextError::ValueUnknown))?;
            match index.checked_sub(call_start) {
                Some(step) => produced
                    .get(step)
                    .copied()
                    .ok_or_else(|| error!(CleartextError::ValueUnknown)),
                None => layout::transient_value(transient_data, index, handle),
            }
        }
        FheExecuteOperand::VerifiedInput { .. }
        | FheExecuteOperand::EarlierStep { .. }
        | FheExecuteOperand::Scalar { .. } => {
            err!(ZamaHostError::InvalidFheExecuteAccount)
        }
    }
}

fn slot_index(state: &EncryptedStore, key: [u8; 32]) -> Result<usize> {
    state
        .slots
        .iter()
        .position(|slot| slot.key == key)
        .ok_or_else(|| error!(CleartextError::ValueUnknown))
}
