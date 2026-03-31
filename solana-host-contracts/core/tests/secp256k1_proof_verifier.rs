use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};
use solana_host_contracts_core::{
    ContextUserInputs, EvmAddress, ExecutorState, Handle, InputVerifierSession, InputVerifierState,
    KmsVerifierState, Pubkey, PublicDecryptVerification, Secp256k1ProofVerifier,
};

fn signing_key(byte: u8) -> SigningKey {
    let mut secret = [0_u8; 32];
    secret[31] = byte;
    SigningKey::from_bytes((&secret).into()).unwrap()
}

fn evm_address(signing_key: &SigningKey) -> EvmAddress {
    let encoded = signing_key.verifying_key().to_encoded_point(false);
    let pubkey = &encoded.as_bytes()[1..65];
    let hash = Keccak256::digest(pubkey);
    let mut address = [0_u8; 20];
    address.copy_from_slice(&hash[12..]);
    EvmAddress::new(address)
}

fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
    let (signature, recovery_id) = signing_key
        .sign_digest_recoverable(Keccak256::new_with_prefix(message))
        .unwrap();
    let mut bytes = signature.to_bytes().to_vec();
    bytes.push(recovery_id.to_byte());
    bytes
}

fn pubkey(byte: u8) -> Pubkey {
    Pubkey::new([byte; 32])
}

#[test]
fn real_input_proof_verifier_accepts_signed_payload() {
    let signer1 = signing_key(1);
    let signer2 = signing_key(2);
    let source_contract = EvmAddress::new([7; 20]);
    let source_chain_id = 54321;
    let verifier = Secp256k1ProofVerifier;
    let mut input_verifier = InputVerifierState::new(
        source_contract,
        source_chain_id,
        vec![evm_address(&signer1), evm_address(&signer2)],
        2,
    )
    .unwrap();

    let host_chain_id = 111;
    let acl_program = pubkey(3);
    let handles =
        ExecutorState::compute_input_handles(b"ciphertext", &[8, 16], acl_program, host_chain_id)
            .unwrap();
    let context = ContextUserInputs {
        user_address: EvmAddress::new([4; 20]),
        contract_address: EvmAddress::new([5; 20]),
    };
    let extra_data = vec![0xAA, 0xBB, 0xCC];
    let payload = solana_host_contracts_core::CiphertextVerification {
        ct_handles: handles.clone(),
        user_address: context.user_address,
        contract_address: context.contract_address,
        contract_chain_id: host_chain_id,
        extra_data: extra_data.clone(),
    };

    let message = Secp256k1ProofVerifier::input_verification_message(
        &payload,
        source_chain_id,
        source_contract,
    );
    let signature1 = sign_message(&signer1, &message);
    let signature2 = sign_message(&signer2, &message);

    let mut proof = Vec::new();
    proof.push(handles.len() as u8);
    proof.push(2);
    for handle in &handles {
        proof.extend_from_slice(handle.as_bytes());
    }
    proof.extend_from_slice(&signature1);
    proof.extend_from_slice(&signature2);
    proof.extend_from_slice(&extra_data);

    let mut session = InputVerifierSession::default();
    let verified = input_verifier
        .verify_input(
            context,
            handles[1],
            &proof,
            &mut session,
            &verifier,
            host_chain_id,
        )
        .unwrap();
    assert_eq!(verified, handles[1]);
}

#[test]
fn real_kms_proof_verifier_accepts_signed_payload() {
    let signer1 = signing_key(11);
    let signer2 = signing_key(12);
    let source_contract = EvmAddress::new([8; 20]);
    let source_chain_id = 65432;
    let verifier = Secp256k1ProofVerifier;
    let kms = KmsVerifierState::new(
        source_contract,
        source_chain_id,
        vec![evm_address(&signer1), evm_address(&signer2)],
        2,
    )
    .unwrap();

    let payload = PublicDecryptVerification {
        ct_handles: vec![Handle::new([0x11; 32]), Handle::new([0x22; 32])],
        decrypted_result: vec![1, 2, 3, 4],
        extra_data: vec![0x00],
    };

    let message =
        Secp256k1ProofVerifier::decryption_message(&payload, source_chain_id, source_contract);
    let signature1 = sign_message(&signer1, &message);
    let signature2 = sign_message(&signer2, &message);

    let mut proof = Vec::new();
    proof.push(2);
    proof.extend_from_slice(&signature1);
    proof.extend_from_slice(&signature2);
    proof.extend_from_slice(&payload.extra_data);

    assert!(kms
        .verify_decryption_signatures(
            payload.ct_handles.clone(),
            payload.decrypted_result.clone(),
            &proof,
            &verifier,
        )
        .unwrap());
}
