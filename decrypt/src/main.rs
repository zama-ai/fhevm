use std::env;
use tfhe::shortint::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let secret_key_file = &args[1];
    let ciphertext_file = &args[2];
    let expected_result = args[3].parse::<u64>().unwrap();

    let bytes = std::fs::read(&secret_key_file).unwrap();
    let hex_cks = hex::decode(&bytes).unwrap();
    let cks: ClientKey = bincode::deserialize(&hex_cks).unwrap();

    let ct_bytes = std::fs::read(&ciphertext_file).unwrap();
    let ct: Ciphertext = bincode::deserialize(&ct_bytes).unwrap();

    let balance = cks.decrypt(&ct);

    println!("Decrypted balance: {}", balance);
    assert_eq!(balance, expected_result);
}
