use std::{fs::read, sync::Arc};

use tfhe::{
    generate_keys, set_server_key,
    shortint::{
        parameters::{
            v1_0::compact_public_key_only::p_fail_2_minus_128::ks_pbs::V1_0_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128,
            v1_0::key_switching::p_fail_2_minus_128::ks_pbs::V1_0_PARAM_KEYSWITCH_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128,
            v1_0::list_compression::V1_0_COMP_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128,
            CompactPublicKeyEncryptionParameters, CompressionParameters,
            ShortintKeySwitchingParameters, PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128,
        },
        ClassicPBSParameters,
    },
    zk::CompactPkeCrs,
    ClientKey, CompactPublicKey, Config, ConfigBuilder, ServerKey,
};

use crate::utils::{safe_deserialize_key, safe_serialize_key};

pub const TFHE_PARAMS: ClassicPBSParameters = PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
pub const TFHE_COMPRESSION_PARAMS: CompressionParameters =
    V1_0_COMP_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
pub const TFHE_COMPACT_PK_ENCRYPTION_PARAMS: CompactPublicKeyEncryptionParameters =
    V1_0_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
pub const TFHE_KS_PARAMS: ShortintKeySwitchingParameters =
    V1_0_PARAM_KEYSWITCH_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;

pub const MAX_BITS_TO_PROVE: usize = 2048;

#[derive(Clone)]
pub struct FhevmKeys {
    pub server_key: ServerKey,
    pub client_key: Option<ClientKey>,
    pub compact_public_key: CompactPublicKey,
    pub public_params: Arc<CompactPkeCrs>,
}

pub struct SerializedFhevmKeys {
    pub server_key: Vec<u8>,
    pub client_key: Option<Vec<u8>>,
    pub compact_public_key: Vec<u8>,
    pub public_params: Vec<u8>,
}

impl FhevmKeys {
    pub fn new() -> Self {
        println!("Generating keys...");
        let config = Self::new_config();
        let (client_key, server_key) = generate_keys(config);
        let compact_public_key = CompactPublicKey::new(&client_key);
        let crs = CompactPkeCrs::from_config(config, MAX_BITS_TO_PROVE).expect("CRS creation");
        FhevmKeys {
            server_key,
            client_key: Some(client_key),
            compact_public_key,
            public_params: Arc::new(crs.clone()),
        }
    }

    pub fn new_config() -> Config {
        ConfigBuilder::with_custom_parameters(TFHE_PARAMS)
            .enable_compression(TFHE_COMPRESSION_PARAMS)
            .use_dedicated_compact_public_key_parameters((
                TFHE_COMPACT_PK_ENCRYPTION_PARAMS,
                TFHE_KS_PARAMS,
            ))
            .build()
    }

    pub fn set_server_key_for_current_thread(&self) {
        set_server_key(self.server_key.clone());
    }
}

impl SerializedFhevmKeys {
    const DIRECTORY: &'static str = "../fhevm-keys";
    const SKS: &'static str = "../fhevm-keys/sks";
    const CKS: &'static str = "../fhevm-keys/cks";
    const PKS: &'static str = "../fhevm-keys/pks";
    const PUBLIC_PARAMS: &'static str = "../fhevm-keys/pp";

    // generating keys is only for testing, so it is okay these are hardcoded
    pub fn save_to_disk(self) {
        println!("Creating directory {}", Self::DIRECTORY);
        std::fs::create_dir_all(Self::DIRECTORY).expect("create keys directory");

        println!("Creating file {}", Self::SKS);
        std::fs::write(format!("{}", Self::SKS), self.server_key).expect("write sks");

        if self.client_key.is_some() {
            println!("Creating file {}", Self::CKS);
            std::fs::write(format!("{}", Self::CKS), self.client_key.unwrap()).expect("write cks");
        }

        println!("Creating file {}", Self::PKS);
        std::fs::write(format!("{}", Self::PKS), self.compact_public_key).expect("write pks");

        println!("Creating file {}", Self::PUBLIC_PARAMS);
        std::fs::write(format!("{}", Self::PUBLIC_PARAMS), self.public_params)
            .expect("write public params");
    }

    pub fn load_from_disk(keys_directory: &str) -> Self {
        let keys_dir = std::path::Path::new(&keys_directory);
        let server_key = read(keys_dir.join("sks")).expect("read server key");
        let client_key = read(keys_dir.join("cks")).ok();
        let compact_public_key = read(keys_dir.join("pks")).expect("read compact public key");
        let public_params = read(keys_dir.join("pp")).expect("read public params");
        SerializedFhevmKeys {
            server_key,
            client_key,
            compact_public_key,
            public_params,
        }
    }
}

impl From<FhevmKeys> for SerializedFhevmKeys {
    fn from(f: FhevmKeys) -> Self {
        SerializedFhevmKeys {
            server_key: safe_serialize_key(&f.server_key),
            client_key: f.client_key.map(|c| safe_serialize_key(&c)),
            compact_public_key: safe_serialize_key(&f.compact_public_key),
            public_params: safe_serialize_key(f.public_params.as_ref()),
        }
    }
}

impl From<SerializedFhevmKeys> for FhevmKeys {
    fn from(f: SerializedFhevmKeys) -> Self {
        FhevmKeys {
            server_key: safe_deserialize_key(&f.server_key).expect("deserialize server key"),
            client_key: f
                .client_key
                .map(|c| safe_deserialize_key(&c).expect("deserialize client key")),
            compact_public_key: safe_deserialize_key(&f.compact_public_key)
                .expect("deserialize compact public key"),
            public_params: Arc::new(
                safe_deserialize_key(&f.public_params).expect("deserialize public params"),
            ),
        }
    }
}
