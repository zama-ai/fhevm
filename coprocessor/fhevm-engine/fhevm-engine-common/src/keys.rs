use std::sync::Arc;

#[cfg(feature = "gpu")]
use tfhe::core_crypto::gpu::get_number_of_gpus;
#[cfg(feature = "gpu")]
use tfhe::shortint::parameters::v1_6::meta::cpu::V1_6_META_PARAM_CPU_2_2_KS_PBS_PKE_TO_SMALL_ZKV2_TUNIFORM_2M128 as gpu_meta_parameters;
use tfhe::shortint::AtomicPatternParameters;
use tfhe::{
    set_server_key,
    shortint::parameters::{
        meta::DedicatedCompactPublicKeyParameters,
        v1_6::meta::cpu::V1_6_META_PARAM_CPU_2_2_KS_PBS_PKE_TO_SMALL_ZKV2_TUNIFORM_2M128 as cpu_meta_parameters,
        CompressionParameters, MetaNoiseSquashingParameters, ReRandomizationParameters,
    },
    zk::CompactPkeCrs,
    ClientKey, CompactPublicKey, CompressedServerKey, Config, ConfigBuilder, ServerKey,
};

use crate::utils::safe_serialize_key;

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
/// Re-randomization mode carried by the meta parameter set. The v1_6 sets
/// resolve to [`ReRandomizationParameters::DerivedCPKWithoutKeySwitch`]: the
/// zeros are encrypted under a compact public key derived from the compute
/// key, so no keyswitch from the dedicated-CPK domain is needed (and no
/// re-randomization keyswitching key is generated). Not a `const` because
/// `MetaParameters::rerandomization_parameters` is not a `const fn`.
pub fn tfhe_re_randomization_params() -> ReRandomizationParameters {
    cpu_meta_parameters
        .rerandomization_parameters()
        .expect("Missing rerandomisation configuration")
}

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
    pub server_key_without_ns: ServerKey,
    pub client_key: Option<ClientKey>,
    pub compact_public_key: CompactPublicKey,
    pub public_params: Arc<CompactPkeCrs>,
    #[cfg(feature = "gpu")]
    pub gpu_server_key: Vec<tfhe::CudaServerKey>,
}

pub struct SerializedFhevmKeys {
    pub server_key: Vec<u8>,
    pub server_key_without_ns: Vec<u8>,
    pub client_key: Option<Vec<u8>>,
    pub compact_public_key: Vec<u8>,
    pub public_params: Vec<u8>,
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
        #[cfg(not(feature = "gpu"))]
        let server_key = compressed_server_key.decompress();
        #[cfg(feature = "gpu")]
        let server_key = compressed_server_key.clone().decompress();
        let (
            sks,
            kskm,
            compression_key,
            decompression_key,
            _noise_squashing_key,
            _noise_squashing_compression_key,
            re_randomization_key,
            _oprf_key,
            tag,
        ) = server_key.clone().into_raw_parts();
        let server_key_without_ns = ServerKey::from_raw_parts(
            sks,
            kskm,
            compression_key,
            decompression_key,
            None, // noise squashing key excluded
            None, // noise squashing compression key excluded
            re_randomization_key,
            None, // oprf key excluded
            tag,
        );

        FhevmKeys {
            server_key,
            server_key_without_ns,
            client_key: Some(client_key),
            compact_public_key,
            public_params: Arc::new(crs),
            #[cfg(feature = "gpu")]
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
            .enable_ciphertext_re_randomization(tfhe_re_randomization_params())
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
    const SKS: &'static str = "../fhevm-keys/sks";
    const CKS: &'static str = "../fhevm-keys/cks";
    const PKS: &'static str = "../fhevm-keys/pks";
    const PUBLIC_PARAMS: &'static str = "../fhevm-keys/pp";
    const FULL_SKS: &'static str = "../fhevm-keys/sns_pk";

    // generating keys is only for testing, so it is okay these are hardcoded
    pub fn save_to_disk(self) {
        println!("Creating directory {}", Self::DIRECTORY);
        std::fs::create_dir_all(Self::DIRECTORY).expect("create keys directory");

        println!("Creating file {}", Self::SKS);
        std::fs::write(Self::SKS, self.server_key_without_ns).expect("write sks");

        println!("Creating file {}", Self::FULL_SKS);
        std::fs::write(Self::FULL_SKS, self.server_key).expect("write sns_pk");

        if let Some(client_key) = self.client_key {
            println!("Creating file {}", Self::CKS);
            std::fs::write(Self::CKS, client_key).expect("write cks");
        }

        println!("Creating file {}", Self::PKS);
        std::fs::write(Self::PKS, self.compact_public_key).expect("write pks");

        println!("Creating file {}", Self::PUBLIC_PARAMS);
        std::fs::write(Self::PUBLIC_PARAMS, self.public_params).expect("write public params");
    }
}

impl From<FhevmKeys> for SerializedFhevmKeys {
    fn from(f: FhevmKeys) -> Self {
        SerializedFhevmKeys {
            client_key: f.client_key.map(|c| safe_serialize_key(&c)),
            compact_public_key: safe_serialize_key(&f.compact_public_key),
            public_params: safe_serialize_key(f.public_params.as_ref()),
            server_key: safe_serialize_key(&f.server_key),
            server_key_without_ns: safe_serialize_key(&f.server_key_without_ns),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The re-randomization mode is fixed by the meta parameter set, and
    /// changing it changes the bytes every coprocessor produces. Pin it here
    /// so a parameter-set bump cannot flip it silently.
    #[test]
    fn re_randomization_is_derived_cpk_without_keyswitch() {
        assert!(matches!(
            tfhe_re_randomization_params(),
            ReRandomizationParameters::DerivedCPKWithoutKeySwitch
        ));
    }

    /// Exercises the keyswitch-free path against the compute and public-key
    /// parameters this crate pins, the way the scheduler calls it: a
    /// `CompactPublicKey` is still handed over (`UseLegacyCPKIfNeeded`), and a
    /// derived-mode server key must ignore it, take the derived key and leave
    /// the plaintext intact.
    #[test]
    fn derived_re_randomization_round_trip() {
        use tfhe::prelude::{FheDecrypt, FheEncrypt, ReRandomize};
        use tfhe::{FheUint64, ReRandomizationContext, ReRandomizationSupport};

        // Compression and noise squashing are irrelevant here and dominate
        // key generation time, so they are left out.
        let config = ConfigBuilder::with_custom_parameters(TFHE_PARAMS)
            .use_dedicated_compact_public_key_parameters((
                TFHE_COMPACT_PK_PARAMS.pke_params,
                TFHE_COMPACT_PK_PARAMS.ksk_params,
            ))
            .enable_ciphertext_re_randomization(tfhe_re_randomization_params())
            .build();

        let client_key = ClientKey::generate(config);
        let compact_public_key = CompactPublicKey::new(&client_key);
        let server_key = ServerKey::new(&client_key);
        assert_eq!(
            server_key.re_randomization_support(),
            ReRandomizationSupport::DerivedCPKWithoutKeySwitch
        );
        set_server_key(server_key);

        let clear = 0xdead_beef_u64;
        let mut ct = FheUint64::encrypt(clear, &client_key);

        let mut context =
            ReRandomizationContext::new(*b"TFHE_Rrd", [b"FheUint64".as_slice()], *b"TFHE_Enc");
        context.add_ciphertext(&ct);
        let mut seed_gen = context.finalize();
        ct.re_randomize(&compact_public_key, seed_gen.next_seed().unwrap())
            .expect("re-randomize under the derived key");

        let decrypted: u64 = ct.decrypt(&client_key);
        assert_eq!(decrypted, clear);
    }
}
