use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=BUILD_CONSENSUS_VERSION");
    println!("cargo:rerun-if-env-changed=BUILD_STACK_VERSION");
    println!("cargo:rerun-if-env-changed=FHEVM_GPU_COMPUTE_CAPABILITY");
    if let Ok(capability) = env::var("FHEVM_GPU_COMPUTE_CAPABILITY") {
        if capability.len() < 2
            || !capability.bytes().all(|byte| byte.is_ascii_digit())
            || capability.parse::<u16>().is_err()
        {
            panic!(
                "FHEVM_GPU_COMPUTE_CAPABILITY must be a numeric CUDA capability such as 90, got {capability:?}"
            );
        }
        println!("cargo:rustc-env=FHEVM_GPU_COMPUTE_CAPABILITY={capability}");
    }
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // tonic 0.14 moved prost codegen out of tonic-build into tonic-prost-build.
    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("common_descriptor.bin"))
        .compile_protos(&["../../proto/common.proto"], &["../../proto"])
        .unwrap();
}
