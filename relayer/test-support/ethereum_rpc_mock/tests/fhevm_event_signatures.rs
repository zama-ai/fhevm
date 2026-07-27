use alloy::sol_types::{SolCall, SolEvent};
use ethereum_rpc_mock::fhevm::{Decryption, InputVerification};

#[test]
fn print_all_fhevm_event_signatures() {
    println!("\n=== FHEVM FUNCTION SELECTORS ===\n");

    println!("1. publicDecryptionRequestCall");
    println!(
        "   Selector: 0x{}",
        hex::encode(Decryption::publicDecryptionRequestCall::SELECTOR)
    );

    println!("2a. userDecryptionRequest_0Call (unified HandleEntry[] overload)");
    println!(
        "    Selector: 0x{}",
        hex::encode(Decryption::userDecryptionRequest_0Call::SELECTOR)
    );

    println!("2b. userDecryptionRequest_1Call (legacy CtHandleContractPair[] overload)");
    println!(
        "    Selector: 0x{}",
        hex::encode(Decryption::userDecryptionRequest_1Call::SELECTOR)
    );

    println!("3. userDecryptionResponseCall");
    println!(
        "   Selector: 0x{}",
        hex::encode(Decryption::userDecryptionResponseCall::SELECTOR)
    );

    println!("4. publicDecryptionResponseCall");
    println!(
        "   Selector: 0x{}",
        hex::encode(Decryption::publicDecryptionResponseCall::SELECTOR)
    );

    println!("5. verifyProofRequestCall");
    println!(
        "   Selector: 0x{}",
        hex::encode(InputVerification::verifyProofRequestCall::SELECTOR)
    );

    println!("5. verifyProofResponseCall");
    println!(
        "   Selector: 0x{}",
        hex::encode(InputVerification::verifyProofResponseCall::SELECTOR)
    );

    println!("6. rejectProofResponseCall");
    println!(
        "   Selector: 0x{}",
        hex::encode(InputVerification::rejectProofResponseCall::SELECTOR)
    );
    println!();

    println!("=== FHEVM EVENT SIGNATURES ===\n");

    println!("1. PublicDecryptionRequest_0");
    println!(
        "   Signature: {}",
        Decryption::PublicDecryptionRequest_0::SIGNATURE
    );
    println!(
        "   Hash:      0x{}\n",
        hex::encode(Decryption::PublicDecryptionRequest_0::SIGNATURE_HASH)
    );

    println!("2. PublicDecryptionResponse");
    println!(
        "   Signature: {}",
        Decryption::PublicDecryptionResponse::SIGNATURE
    );
    println!(
        "   Hash:      0x{}\n",
        hex::encode(Decryption::PublicDecryptionResponse::SIGNATURE_HASH)
    );

    println!("3a. UserDecryptionRequest_0 (legacy event)");
    println!(
        "    Signature: {}",
        Decryption::UserDecryptionRequest_0::SIGNATURE
    );
    println!(
        "    Hash:      0x{}\n",
        hex::encode(Decryption::UserDecryptionRequest_0::SIGNATURE_HASH)
    );

    println!("3b. UserDecryptionRequest_1 (unified event)");
    println!(
        "    Signature: {}",
        Decryption::UserDecryptionRequest_1::SIGNATURE
    );
    println!(
        "    Hash:      0x{}\n",
        hex::encode(Decryption::UserDecryptionRequest_1::SIGNATURE_HASH)
    );

    println!("4. UserDecryptionResponse");
    println!(
        "   Signature: {}",
        Decryption::UserDecryptionResponse::SIGNATURE
    );
    println!(
        "   Hash:      0x{}\n",
        hex::encode(Decryption::UserDecryptionResponse::SIGNATURE_HASH)
    );

    println!("5. VerifyProofRequest");
    println!(
        "   Signature: {}",
        InputVerification::VerifyProofRequest::SIGNATURE
    );
    println!(
        "   Hash:      0x{}\n",
        hex::encode(InputVerification::VerifyProofRequest::SIGNATURE_HASH)
    );

    println!("6. VerifyProofResponse");
    println!(
        "   Signature: {}",
        InputVerification::VerifyProofResponse::SIGNATURE
    );
    println!(
        "   Hash:      0x{}\n",
        hex::encode(InputVerification::VerifyProofResponse::SIGNATURE_HASH)
    );

    println!("7. RejectProofResponse");
    println!(
        "   Signature: {}",
        InputVerification::RejectProofResponse::SIGNATURE
    );
    println!(
        "   Hash:      0x{}\n",
        hex::encode(InputVerification::RejectProofResponse::SIGNATURE_HASH)
    );

    println!("===========================\n");
}
