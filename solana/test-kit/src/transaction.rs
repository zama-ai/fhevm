//! Transaction envelope used by every FHE consumer: one open, application calls,
//! one final close. TransientStore account contents are always created by the real host.

use anchor_lang::prelude::Instructions;
use mollusk_svm::{
    result::{types::TransactionResult, Check, InstructionResult},
    Mollusk,
};
use solana_sdk::{
    account::Account, instruction::Instruction, pubkey::Pubkey, sysvar::SysvarId,
    transaction::TransactionError,
};

use crate::{anchor_ix, empty_system_account, ensure_system_accounts, Ctx};

pub fn fhe_transaction(
    payer: Pubkey,
    body: impl IntoIterator<Item = Instruction>,
) -> Vec<Instruction> {
    let transient_store = zama_host::transient_store_address(payer).0;
    let open = anchor_ix(
        zama_host::ID,
        zama_host::accounts::OpenTransientStore {
            payer,
            transient_store,
            instructions: Instructions::id(),
            system_program: anchor_lang::system_program::ID,
        },
        zama_host::instruction::OpenTransientStore {},
    );
    let close = anchor_ix(
        zama_host::ID,
        zama_host::accounts::CloseTransientStore {
            transient_store,
            refund: payer,
            instructions: Instructions::id(),
        },
        zama_host::instruction::CloseTransientStore {},
    );
    std::iter::once(open)
        .chain(body)
        .chain(std::iter::once(close))
        .collect()
}

/// Keeps the existing instruction assertions and CPI oracle, but runs the body in
/// a real shared transaction. CU/time cover the full envelope; CPIs cover the body.
/// An envelope failure must not satisfy a body-error test.
fn body_result(result: TransactionResult) -> InstructionResult {
    let raw_result = result.raw_result.map_err(|error| match error {
        TransactionError::InstructionError(index, error) => {
            assert_eq!(
                index, 1,
                "the transient_store envelope failed before/after the tested instruction: {error:?}"
            );
            error
        }
        other => panic!("transaction setup failed: {other:?}"),
    });
    InstructionResult {
        compute_units_consumed: result.compute_units_consumed,
        execution_time: result.execution_time,
        program_result: raw_result.clone().into(),
        raw_result,
        return_data: result.return_data,
        resulting_accounts: result.resulting_accounts,
        inner_instructions: result
            .inner_instructions
            .into_iter()
            .nth(1)
            .unwrap_or_default(),
        message: result.message,
    }
}

pub fn process_fhe_instruction(
    context: &Ctx,
    payer: Pubkey,
    instruction: &Instruction,
    checks: &[Check],
) -> InstructionResult {
    ensure_system_accounts(context, &[zama_host::transient_store_address(payer).0]);
    let result = body_result(
        context.process_transaction_instructions(&fhe_transaction(payer, [instruction.clone()])),
    );
    assert!(result.run_checks(checks, &context.mollusk.config, &context.mollusk));
    result
}

pub fn check_fhe_instruction(
    mollusk: &Mollusk,
    payer: Pubkey,
    instruction: &Instruction,
    accounts: &[(Pubkey, Account)],
    checks: &[Check],
) -> InstructionResult {
    let mut accounts = accounts.to_vec();
    let transient_store = zama_host::transient_store_address(payer).0;
    if !accounts
        .iter()
        .any(|(address, _)| *address == transient_store)
    {
        accounts.push((transient_store, empty_system_account()));
    }
    let result = body_result(mollusk.process_transaction_instructions(
        &fhe_transaction(payer, [instruction.clone()]),
        &accounts,
    ));
    assert!(result.run_checks(checks, &mollusk.config, mollusk));
    result
}
