use tfhe::{
    generate_keys,
    shortint::parameters::{
        list_compression::COMP_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64,
        PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64,
    },
    CompactPublicKey, ConfigBuilder,
};

pub mod tfhe_ops;
pub mod types;

pub struct FhevmKeys {
    pub server_key: Vec<u8>,
    pub client_key: Vec<u8>,
    pub compact_public_key: Vec<u8>,
}

pub fn generate_fhe_keys() -> FhevmKeys {
    let config =
        ConfigBuilder::with_custom_parameters(PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64)
            .enable_compression(COMP_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64)
            .build();
    let (client_key, server_key) = generate_keys(config);
    let public_key = CompactPublicKey::new(&client_key);

    let client_key = bincode::serialize(&client_key).unwrap();
    let server_key = bincode::serialize(&server_key).unwrap();
    let compact_public_key = bincode::serialize(&public_key).unwrap();
    FhevmKeys {
        server_key,
        client_key,
        compact_public_key,
    }
}
