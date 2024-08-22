use std::fs::read;

use tfhe::{
    generate_keys,
    shortint::parameters::{
        list_compression::COMP_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64,
        PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64,
    },
    ClientKey, CompactPublicKey, ConfigBuilder, ServerKey,
};

pub struct FhevmKeys {
    pub server_key: ServerKey,
    pub client_key: Option<ClientKey>,
    pub compact_public_key: Option<CompactPublicKey>,
}

pub struct SerializedFhevmKeys {
    pub server_key: Vec<u8>,
    pub client_key: Option<Vec<u8>>,
    pub compact_public_key: Option<Vec<u8>>,
}

impl FhevmKeys {
    pub fn new() -> Self {
        println!("Generating keys...");
        let config =
            ConfigBuilder::with_custom_parameters(PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64)
                .enable_compression(COMP_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64)
                .build();
        let (client_key, server_key) = generate_keys(config);
        let compact_public_key = CompactPublicKey::new(&client_key);
        FhevmKeys {
            server_key,
            client_key: Some(client_key),
            compact_public_key: Some(compact_public_key),
        }
    }
}

impl SerializedFhevmKeys {
    const DIRECTORY: &'static str = "fhevm-keys";
    const SKS: &'static str = "fhevm-keys/sks";
    const CKS: &'static str = "fhevm-keys/cks";
    const PKS: &'static str = "fhevm-keys/pks";

    pub fn save_to_disk(self) {
        println!("Creating directory {}", Self::DIRECTORY);
        std::fs::create_dir_all(Self::DIRECTORY).expect("create keys directory");

        println!("Creating file {}", Self::SKS);
        std::fs::write(format!("{}", Self::SKS), self.server_key).expect("write sks");

        if self.client_key.is_some() {
            println!("Creating file {}", Self::CKS);
            std::fs::write(format!("{}", Self::CKS), self.client_key.unwrap()).expect("write cks");
        }

        if self.compact_public_key.is_some() {
            println!("Creating file {}", Self::PKS);
            std::fs::write(format!("{}", Self::PKS), self.compact_public_key.unwrap())
                .expect("write pks");
        }
    }

    pub fn load_from_disk() -> Self {
        let server_key = read(Self::SKS).expect("read server key");
        let client_key = read(Self::CKS);
        let compact_public_key = read(Self::PKS);
        SerializedFhevmKeys {
            server_key,
            client_key: client_key.ok(),
            compact_public_key: compact_public_key.ok(),
        }
    }
}

impl From<FhevmKeys> for SerializedFhevmKeys {
    fn from(f: FhevmKeys) -> Self {
        SerializedFhevmKeys {
            server_key: bincode::serialize(&f.server_key).expect("serialize server key"),
            client_key: f
                .client_key
                .map(|c| bincode::serialize(&c).expect("serialize client key")),
            compact_public_key: f
                .compact_public_key
                .map(|p| bincode::serialize(&p).expect("serialize compact public key")),
        }
    }
}

impl From<SerializedFhevmKeys> for FhevmKeys {
    fn from(f: SerializedFhevmKeys) -> Self {
        FhevmKeys {
            server_key: bincode::deserialize(&f.server_key).expect("deserialize server key"),
            client_key: f
                .client_key
                .map(|c| bincode::deserialize(&c).expect("deserialize client key")),
            compact_public_key: f
                .compact_public_key
                .map(|p| bincode::deserialize(&p).expect("deserialize compact public key")),
        }
    }
}
