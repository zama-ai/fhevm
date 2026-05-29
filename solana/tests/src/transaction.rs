use anchor_lang::{InstructionData, ToAccountMetas};
use anchor_litesvm::Program;
use litesvm::LiteSVM;
use solana_sdk::{
    instruction::Instruction,
    message::{Message, VersionedMessage},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};

pub fn anchor_ix<A, D>(program_id: Pubkey, accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    Program::new(program_id)
        .accounts(accounts)
        .args(args)
        .instruction()
        .unwrap()
}

pub fn send(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) {
    try_send(svm, payer, ix).unwrap();
}

pub fn send_with_meta(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> (litesvm::types::TransactionMetadata, Vec<Pubkey>) {
    let (tx, account_keys) = build_legacy_tx(svm, &payer.pubkey(), &[ix], &[payer]);
    (svm.send_transaction(tx).unwrap(), account_keys)
}

pub fn send_with_meta_and_signature(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> (
    litesvm::types::TransactionMetadata,
    Vec<Pubkey>,
    solana_sdk::signature::Signature,
) {
    let (tx, account_keys) = build_legacy_tx(svm, &payer.pubkey(), &[ix], &[payer]);
    let signature = tx.signatures[0];
    (svm.send_transaction(tx).unwrap(), account_keys, signature)
}

pub fn try_send(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> litesvm::types::TransactionResult {
    send_with_signers(svm, &payer.pubkey(), ix, &[payer])
}

pub fn try_send_with_meta(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> (litesvm::types::TransactionResult, Vec<Pubkey>) {
    let (tx, account_keys) = build_legacy_tx(svm, &payer.pubkey(), &[ix], &[payer]);
    (svm.send_transaction(tx), account_keys)
}

pub fn send_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ix: Instruction,
    signers: &[&Keypair],
) -> litesvm::types::TransactionResult {
    let (tx, _) = build_legacy_tx(svm, payer, &[ix], signers);
    svm.send_transaction(tx)
}

pub fn send_many_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ixs: Vec<Instruction>,
    signers: &[&Keypair],
) -> litesvm::types::TransactionResult {
    let (tx, _) = build_legacy_tx(svm, payer, &ixs, signers);
    svm.send_transaction(tx)
}

/// Build a signed legacy transaction and capture its account keys.
fn build_legacy_tx(
    svm: &LiteSVM,
    payer: &Pubkey,
    ixs: &[Instruction],
    signers: &[&Keypair],
) -> (VersionedTransaction, Vec<Pubkey>) {
    let message = Message::new_with_blockhash(ixs, Some(payer), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    (tx, account_keys)
}
