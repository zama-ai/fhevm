use crate::{config::Config, decryption::extract_id_from_receipt};
use alloy::{primitives::U256, rpc::types::TransactionReceipt, sol_types::SolEvent};
use anyhow::anyhow;
use fhevm_gateway_rust_bindings::decryption::Decryption;
use gateway_sdk::{FhevmSdk, signature::Eip712Result};
use std::sync::Arc;

#[allow(dead_code)]
fn extract_user_decryption_id_from_receipt(receipt: &TransactionReceipt) -> anyhow::Result<U256> {
    extract_id_from_receipt(
        receipt,
        Decryption::UserDecryptionRequest::SIGNATURE_HASH,
        |log| {
            Decryption::UserDecryptionRequest::decode_log_data(log)
                .map(|event| event.decryptionId)
                .map_err(|e| anyhow!("Failed to decode event data {e}"))
        },
    )
}

pub fn generate_eip712(
    sdk: Arc<FhevmSdk>,
    config: &Config,
    timestamp: u64,
) -> anyhow::Result<Eip712Result> {
    let allowed_contract = config.allowed_contract.clone();
    let private_key = config.private_key.clone().unwrap();

    // Spawn in new thread otherwise panic because it blocks the async runtime
    std::thread::spawn(move || {
        sdk.create_eip712_signature_builder()
            .with_public_key(RAND_PUBLIC_KEY)
            .with_contract(&allowed_contract)
            .unwrap()
            .with_verification(true)
            .with_validity_period(timestamp, DURATION_DAYS)
            .with_private_key(&private_key)
            .with_extra_data(EXTRA_DATA.to_vec())
            .generate_and_sign()
            .inspect(|r| println!("{r:?}"))
            .unwrap()
    })
    .join()
    .map_err(|e| anyhow!("{e:?}"))
}

pub const DURATION_DAYS: u64 = 10;
pub const EXTRA_DATA: [u8; 1] = [0];
pub const RAND_PUBLIC_KEY: &str = "0x0300000000000000302e35000000000300000000000000302e311300000000000000556e69666965645075626c6963456e634b65790000000000000000200300000000000014ea3e413024ff622b2a29974a7418a0c0b5d3fb5d93f83a414384386701e7895646cc83ff289216007c4a7821ec4291d4e92e5fa3a8e96cbd11491696407336d6a03bb971a4207f90b35958191ac69599d8337fa114cb607552e4113253c875f824c027e15e5ee2c73f5c4885d66f5324915f94c1dd1a0c4d226f9967c56db99bf81c3bfbb273e1fcc41f01639f754f13f8c62ec4cb6af14c373bc2d97955b1f50b66124ea38757b41c21191135c304b2b9223e2067cd4691c5ee840bbc3a382037c0ea611f8e0a6f6cb22afdf0a61f6aa11dc28691cb41d9b5a13eb94a3aea4e8e7c07c9f467949acac6bca98f720754485de7b95568495e0c90c9e73175db850a63d3cf45189846d20894e633636621b8107dbc132b9a281dbf382231697f67249a6b177db34c5b4639bc605424ab23cc186b00d0430e9efba3b6e8c07c615df7e0cf2c8082ffe640f6d4173b2c0b42f0c57e0a7b29a886b0808427f46cd0359e68a54bc783a684b95fe1859373d636e812524ee42a4ac8c8bab03a421cb8c8734ede914d8d221a4e258485f5415fe15d8aebcf0225ce87451402170d97a21313789750f794ca064129b1b3c8fb452c2b1d8d7a5bc514638362b952362c06536906000b08551e3087336869c2cb2190741a58c6171e6a562e83389042cb844cd3c2bf8365e36168fbe1bcfc2b1b281863dbe62fbb620f4683c3dcd34f22647f6d583950f834bf69cb3a9c790c049a7ec2cdfcfcaefd2b0281969913ca2ed134acbb917f36e3bf8fa8818a8a146a518ef00346d91469de68b495200a50d205735a781c650319f953c62255c8f93cd931387da3b2eb474f3bd9431641c2f62b6ddae95e34d01dc8510382e925610228508b30d5271ec74a01b9b0c6ceb07250565055369af2577bbae08b9bb61c38acac6ac6bb65494d2b5846fe345556252deecb6c8c2629e8a67585fcbb9c992266f6a739fc8d45269acb0c5250f4990f899982493a50d23d9228cdbf726bdcf250d19705f5991ee966077e58b04cb0023a9984431828e9085b34c959cfeb838d4507d4d1b8704c5f6ce7b1298c89a984196fec71df209828e693510719ef86d5740906b94a86aa52b57b955b4ffb9c1cc5";
