//! # Minimal User Decryption
//!
//! Decrypt an encrypted value using KMS client api.
//! This example is taken from a centralized KMS configuration.
//!
//! ```bash
//! cargo run --example minimal-user-decryption
//! ```

use alloy::primitives::U256;
use fhevm_sdk::{
    Result, blockchain::bindings::Decryption::CtHandleContractPair,
    decryption::user::process_user_decryption, utils::validate_address_from_str,
};
use std::str::FromStr;
fn main() -> Result<()> {
    println!("🔓 Processing user decryption...");

    let handle_pair = CtHandleContractPair {
        ctHandle: U256::from_str(
            "0xf2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200",
        )?
        .into(),
        contractAddress: validate_address_from_str("0xa3f4D50ebfea1237316b4377F0fff4831F2D1c46")?,
    };

    let json_response = r#"{
        "response": [{
            "payload": "29000000000000002100000000000000029395c8ff9ca2d768dd40bf9fb6dfc834874487da26218ee57a929228b807ff2b20000000000000002a92056afa790a38b17237730d08ef686c04a2e0dac55aec05b97f26c79a95a50100000000000000020000001501000000000000c5000000000000003b62b10c9abb1f9c4caef03543917fa093758c0b6eb22444293172d287415966f72a4bb1c352aacf7c0003652653aefedb05871dbf068643e8f57baa56a631b579ea0d062921c178e9a73ca341d8a983687a84cd1690af7f4679642a5e3209f8d902c9fcde4a18d8c2dc5bd06d30cdae4ae26c838d35199db8497d454fa4dfc6ec43315254b901d4262fb07f0a039b9523ea0aa658ea583ed29fe54ce22d9fa361502be74746c993e814e6685e7ba723cfcd7b590fa394efbd9068156dfc17d9d3c8c5fa7f1800000000000000717abaaaeb83db7e49cac2168789d3184de51040f7205936200000000000000031604bdf7defdf92477633d530e37899aa12b94dcf132fb6d717aad48b8b625d2000000000000000f2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200010000000100000000000000",
            "signature": "70ec9d960d08632518ba9591f028edeb3c55345c35f0b383596a13e8a7d773582af5f1f2c1897b73d05333d39ab8c9d5bef64e7c14bc636d4a176c30ba3842ee1b"
        }]
    }"#;

    let result = process_user_decryption(
        &["0x67F6A11ADf13CEDdB8319Fe12705809563611703".to_string()],
        "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
        54321,
        "0xc9bAE822fE6793e3B456144AdB776D5A318CB71e",
        "791e8a06dab85d960745c4c5dea65fdc250e0d42cbfbd2037ae221d2baa980c062f8b46f023c11bba8ba49c17e9e73a8ce0556040c567849b62b675678c3bc071c",
        "2000000000000000750f4e54713eae622dfeb01809290183a447e2b277e89d2c6a681af1aa5b2c2b",
        "2000000000000000321387e7b579a16d9bcb17d14625dc2841314c05f7c266418a9576091178902d",
        &[handle_pair],
        json_response,
    );

    match result {
        Ok(_) => println!("✅ Decryption completed! (Expected result: 42)"),
        Err(e) => println!("⚠️ Decryption failed (expected in test env): {}", e),
    }

    Ok(())
}
