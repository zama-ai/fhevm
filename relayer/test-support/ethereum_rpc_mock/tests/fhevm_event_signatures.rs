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

    println!("2. userDecryptionRequestCall");
    println!(
        "   Selector: 0x{}",
        hex::encode(Decryption::userDecryptionRequestCall::SELECTOR)
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

    println!("1. PublicDecryptionRequest");
    println!(
        "   Signature: {}",
        Decryption::PublicDecryptionRequest::SIGNATURE
    );
    println!(
        "   Hash:      0x{}\n",
        hex::encode(Decryption::PublicDecryptionRequest::SIGNATURE_HASH)
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

    println!("3. UserDecryptionRequest");
    println!(
        "   Signature: {}",
        Decryption::UserDecryptionRequest::SIGNATURE
    );
    println!(
        "   Hash:      0x{}\n",
        hex::encode(Decryption::UserDecryptionRequest::SIGNATURE_HASH)
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
