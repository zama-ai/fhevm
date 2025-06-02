use std::{fs::read, sync::Arc};

#[cfg(feature = "gpu")]
use tfhe::core_crypto::gpu::get_number_of_gpus;
#[cfg(feature = "gpu")]
use tfhe::shortint::parameters::v1_4::meta::gpu::V1_4_META_PARAM_GPU_2_2_MULTI_BIT_GROUP_4_KS_PBS_PKE_TO_SMALL_ZKV2_TUNIFORM_2M128 as gpu_meta_parameters;
use tfhe::shortint::AtomicPatternParameters;
use tfhe::{
    set_server_key,
    shortint::parameters::{
        meta::DedicatedCompactPublicKeyParameters,
        v1_4::meta::cpu::V1_4_META_PARAM_CPU_2_2_KS_PBS_PKE_TO_SMALL_ZKV2_TUNIFORM_2M128 as cpu_meta_parameters,
        CompressionParameters, MetaNoiseSquashingParameters, ShortintKeySwitchingParameters,
    },
    zk::CompactPkeCrs,
    ClientKey, CompactPublicKey, CompressedServerKey, Config, ConfigBuilder, ServerKey,
};

use crate::utils::{safe_deserialize_key, safe_serialize_key};

#[cfg(not(feature = "gpu"))]
pub const TFHE_PARAMS: AtomicPatternParameters = cpu_meta_parameters.compute_parameters;
#[cfg(not(feature = "gpu"))]
pub const TFHE_COMPRESSION_PARAMS: CompressionParameters = cpu_meta_parameters
    .compression_parameters
    .expect("Missing compression parameters");

pub const TFHE_COMPACT_PK_PARAMS: DedicatedCompactPublicKeyParameters = cpu_meta_parameters
    .dedicated_compact_public_key_parameters
    .expect("Missing compact public key parameters");
pub const TFHE_NOISE_SQUASHING_PARAMS: MetaNoiseSquashingParameters = cpu_meta_parameters
    .noise_squashing_parameters
    .expect("Missing noise squashing parameters");
pub const TFHE_PKS_RERANDOMIZATION_PARAMS: ShortintKeySwitchingParameters = cpu_meta_parameters
    .re_randomization_parameters
    .expect("Missing rerandomisation parameters");

#[cfg(feature = "gpu")]
pub const TFHE_PARAMS: AtomicPatternParameters = gpu_meta_parameters.compute_parameters;
#[cfg(feature = "gpu")]
pub const TFHE_COMPRESSION_PARAMS: CompressionParameters = gpu_meta_parameters
    .compression_parameters
    .expect("Missing compression parameters");

pub const MAX_BITS_TO_PROVE: usize = 2048;

#[derive(Clone)]
pub struct FhevmKeys {
    pub server_key: ServerKey,
    #[cfg(not(feature = "gpu"))]
    pub server_key_without_ns: ServerKey,
    pub client_key: Option<ClientKey>,
    pub compact_public_key: CompactPublicKey,
    pub public_params: Arc<CompactPkeCrs>,
    #[cfg(feature = "gpu")]
    pub compressed_server_key: CompressedServerKey,
    #[cfg(feature = "gpu")]
    pub gpu_server_key: Vec<tfhe::CudaServerKey>,
}

pub struct SerializedFhevmKeys {
    #[cfg(not(feature = "gpu"))]
    pub server_key: Vec<u8>,
    #[cfg(not(feature = "gpu"))]
    pub server_key_without_ns: Vec<u8>,
    pub client_key: Option<Vec<u8>>,
    pub compact_public_key: Vec<u8>,
    pub public_params: Vec<u8>,
    #[cfg(feature = "gpu")]
    pub compressed_server_key: Vec<u8>,
}

impl Default for FhevmKeys {
    fn default() -> Self {
        Self::new()
    }
}

impl FhevmKeys {
    pub fn new() -> Self {
        println!("Generating keys...");
        let config = Self::new_config();
        let client_key = tfhe::ClientKey::generate(config);
        let compact_public_key = CompactPublicKey::new(&client_key);
        let crs = CompactPkeCrs::from_config(config, MAX_BITS_TO_PROVE).expect("CRS creation");
        let compressed_server_key = CompressedServerKey::new(&client_key);
        let server_key = compressed_server_key.clone().decompress();
        #[cfg(not(feature = "gpu"))]
        let (
            sks,
            kskm,
            compression_key,
            decompression_key,
            _noise_squashing_key,
            _noise_squashing_compression_key,
            re_randomization_keyswitching_key,
            tag,
        ) = server_key.clone().into_raw_parts();
        #[cfg(not(feature = "gpu"))]
        let server_key_without_ns = ServerKey::from_raw_parts(
            sks,
            kskm,
            compression_key,
            decompression_key,
            None, // noise squashing key excluded
            None, // noise squashing compression key excluded
            re_randomization_keyswitching_key,
            tag,
        );

        FhevmKeys {
            server_key,
            #[cfg(not(feature = "gpu"))]
            server_key_without_ns,
            client_key: Some(client_key),
            compact_public_key,
            public_params: Arc::new(crs.clone()),
            #[cfg(feature = "gpu")]
            compressed_server_key: compressed_server_key.clone(),
            #[cfg(feature = "gpu")]
            #[cfg(feature = "latency")]
            gpu_server_key: vec![compressed_server_key.decompress_to_gpu()],
            #[cfg(feature = "gpu")]
            #[cfg(not(feature = "latency"))]
            gpu_server_key: (0..get_number_of_gpus())
                .map(|i| compressed_server_key.decompress_to_specific_gpu(tfhe::GpuIndex::new(i)))
                .collect::<Vec<_>>(),
        }
    }

    pub fn new_config() -> Config {
        ConfigBuilder::with_custom_parameters(TFHE_PARAMS)
            .enable_noise_squashing(TFHE_NOISE_SQUASHING_PARAMS.parameters)
            .enable_noise_squashing_compression(
                TFHE_NOISE_SQUASHING_PARAMS
                    .compression_parameters
                    .expect("Missing noise squahing compression parameters."),
            )
            .enable_compression(TFHE_COMPRESSION_PARAMS)
            .use_dedicated_compact_public_key_parameters((
                TFHE_COMPACT_PK_PARAMS.pke_params,
                TFHE_COMPACT_PK_PARAMS.ksk_params,
            ))
            .enable_ciphertext_re_randomization(TFHE_PKS_RERANDOMIZATION_PARAMS)
            .build()
    }

    pub fn set_server_key_for_current_thread(&self) {
        set_server_key(self.server_key.clone());
    }
    pub fn set_gpu_server_key_for_current_thread(&self) {
        #[cfg(feature = "gpu")]
        set_server_key(self.gpu_server_key[0].clone());
        #[cfg(not(feature = "gpu"))]
        set_server_key(self.server_key.clone());
    }
}

impl SerializedFhevmKeys {
    const DIRECTORY: &'static str = "../fhevm-keys";
    #[cfg(not(feature = "gpu"))]
    const SKS: &'static str = "../fhevm-keys/sks";
    #[cfg(not(feature = "gpu"))]
    const CKS: &'static str = "../fhevm-keys/cks";
    #[cfg(not(feature = "gpu"))]
    const PKS: &'static str = "../fhevm-keys/pks";
    #[cfg(not(feature = "gpu"))]
    const PUBLIC_PARAMS: &'static str = "../fhevm-keys/pp";
    #[cfg(not(feature = "gpu"))]
    const FULL_SKS: &'static str = "../fhevm-keys/sns_pk";

    #[cfg(feature = "gpu")]
    const GPU_CSKS: &'static str = "../fhevm-keys/gpu-csks";
    #[cfg(feature = "gpu")]
    const GPU_CKS: &'static str = "../fhevm-keys/gpu-cks";
    #[cfg(feature = "gpu")]
    const GPU_PKS: &'static str = "../fhevm-keys/gpu-pks";
    #[cfg(feature = "gpu")]
    const GPU_PUBLIC_PARAMS: &'static str = "../fhevm-keys/gpu-pp";

    // generating keys is only for testing, so it is okay these are hardcoded
    pub fn save_to_disk(self) {
        println!("Creating directory {}", Self::DIRECTORY);
        std::fs::create_dir_all(Self::DIRECTORY).expect("create keys directory");
        #[cfg(not(feature = "gpu"))]
        {
            println!("Creating file {}", Self::SKS);
            std::fs::write(Self::SKS, self.server_key_without_ns).expect("write sks");

            println!("Creating file {}", Self::FULL_SKS);
            std::fs::write(Self::FULL_SKS, self.server_key).expect("write sns_pk");

            if self.client_key.is_some() {
                println!("Creating file {}", Self::CKS);
                std::fs::write(Self::CKS, self.client_key.unwrap()).expect("write cks");
            }

            println!("Creating file {}", Self::PKS);
            std::fs::write(Self::PKS, self.compact_public_key).expect("write pks");

            println!("Creating file {}", Self::PUBLIC_PARAMS);
            std::fs::write(Self::PUBLIC_PARAMS, self.public_params).expect("write public params");
        }
        #[cfg(feature = "gpu")]
        {
            println!("Creating file {}", Self::GPU_CSKS);
            std::fs::write(Self::GPU_CSKS, self.compressed_server_key).expect("write gpu csks");

            if self.client_key.is_some() {
                println!("Creating file {}", Self::GPU_CKS);
                std::fs::write(Self::GPU_CKS, self.client_key.unwrap()).expect("write gpu cks");
            }

            println!("Creating file {}", Self::GPU_PKS);
            std::fs::write(Self::GPU_PKS, self.compact_public_key).expect("write gpu pks");

            println!("Creating file {}", Self::GPU_PUBLIC_PARAMS);
            std::fs::write(Self::GPU_PUBLIC_PARAMS, self.public_params)
                .expect("write gpu public params");
        }
    }

    pub fn load_from_disk(keys_directory: &str) -> Self {
        let keys_dir = std::path::Path::new(&keys_directory);
        let (sns_pk, sks, cks, pks, pp) = if !cfg!(feature = "gpu") {
            ("sns_pk", "sks", "cks", "pks", "pp")
        } else {
            ("_unused_", "gpu-csks", "gpu-cks", "gpu-pks", "gpu-pp")
        };
        let server_key = read(keys_dir.join(sns_pk)).expect("read full server key (sns_pk)");
        #[cfg(not(feature = "gpu"))]
        let server_key_without_ns = read(keys_dir.join(sks)).expect("read server key");
        let client_key = read(keys_dir.join(cks)).ok();
        let compact_public_key = read(keys_dir.join(pks)).expect("read compact public key");
        let public_params = read(keys_dir.join(pp)).expect("read public params");
        SerializedFhevmKeys {
            client_key,
            compact_public_key,
            public_params,
            #[cfg(not(feature = "gpu"))]
            server_key,
            #[cfg(not(feature = "gpu"))]
            server_key_without_ns,
            #[cfg(feature = "gpu")]
            compressed_server_key: server_key,
        }
    }
}

impl From<FhevmKeys> for SerializedFhevmKeys {
    fn from(f: FhevmKeys) -> Self {
        SerializedFhevmKeys {
            client_key: f.client_key.map(|c| safe_serialize_key(&c)),
            compact_public_key: safe_serialize_key(&f.compact_public_key),
            public_params: safe_serialize_key(f.public_params.as_ref()),
            #[cfg(not(feature = "gpu"))]
            server_key: safe_serialize_key(&f.server_key),
            #[cfg(not(feature = "gpu"))]
            server_key_without_ns: safe_serialize_key(&f.server_key_without_ns),
            #[cfg(feature = "gpu")]
            compressed_server_key: safe_serialize_key(&f.compressed_server_key),
        }
    }
}

impl From<SerializedFhevmKeys> for FhevmKeys {
    fn from(f: SerializedFhevmKeys) -> Self {
        let client_key = f
            .client_key
            .map(|c| safe_deserialize_key(&c).expect("deserialize client key"));
        #[cfg(feature = "gpu")]
        let compressed_server_key: CompressedServerKey =
            safe_deserialize_key(&f.compressed_server_key)
                .expect("deserialize compressed server key");

        FhevmKeys {
            client_key: client_key.clone(),
            compact_public_key: safe_deserialize_key(&f.compact_public_key)
                .expect("deserialize compact public key"),
            public_params: Arc::new(
                safe_deserialize_key(&f.public_params).expect("deserialize public params"),
            ),
            #[cfg(not(feature = "gpu"))]
            server_key: safe_deserialize_key(&f.server_key)
                .expect("deserialize full server key (sns_pk)"),
            #[cfg(not(feature = "gpu"))]
            server_key_without_ns: safe_deserialize_key(&f.server_key_without_ns)
                .expect("deserialize server key"),
            #[cfg(feature = "gpu")]
            compressed_server_key: compressed_server_key.clone(),
            #[cfg(feature = "gpu")]
            #[cfg(feature = "latency")]
            gpu_server_key: vec![compressed_server_key.decompress_to_gpu()],
            #[cfg(feature = "gpu")]
            #[cfg(not(feature = "latency"))]
            gpu_server_key: (0..get_number_of_gpus())
                .map(|i| compressed_server_key.decompress_to_specific_gpu(tfhe::GpuIndex::new(i)))
                .collect::<Vec<_>>(),
            #[cfg(feature = "gpu")]
            server_key: compressed_server_key.decompress(),
        }
    }
}
