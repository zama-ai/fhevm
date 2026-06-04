use {
    anchor_lang::{
        prelude::Pubkey, solana_program::instruction::Instruction, AccountDeserialize,
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_write_data() {
    let program_id = geyser::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/geyser.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let (data_account, _bump) =
        Pubkey::find_program_address(&[b"data", payer.pubkey().as_ref()], &program_id);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &geyser::instruction::WriteData {
            value: 42,
            message: "hello-from-geyser".to_string(),
        }
        .data(),
        geyser::accounts::WriteData {
            authority: payer.pubkey(),
            data_account,
            system_program: anchor_lang::solana_program::system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer.insecure_clone()])
        .unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok(), "write_data failed: {res:?}");

    // Read the PDA back and confirm the program persisted our data.
    let account = svm.get_account(&data_account).expect("PDA should exist");
    let decoded = geyser::DataAccount::try_deserialize(&mut account.data.as_slice())
        .expect("PDA should hold a DataAccount");
    assert_eq!(decoded.authority, payer.pubkey());
    assert_eq!(decoded.value, 42);
    assert_eq!(decoded.message, "hello-from-geyser");
}
