pub mod types;
pub mod tfhe_ops;

pub struct FhevmKeys {
    pub server_key: Vec<u8>,
    pub client_key: Vec<u8>,
    pub compact_public_key: Vec<u8>,
}

pub fn generate_fhe_keys() -> FhevmKeys {
    let (client_key, server_key) = tfhe::generate_keys(tfhe::ConfigBuilder::default().build());
    let compact_key = tfhe::CompactPublicKey::new(&client_key);
    let client_key = bincode::serialize(&client_key).unwrap();
    let server_key = bincode::serialize(&server_key).unwrap();
    let compact_public_key = bincode::serialize(&compact_key).unwrap();
    FhevmKeys { server_key, client_key, compact_public_key }
}