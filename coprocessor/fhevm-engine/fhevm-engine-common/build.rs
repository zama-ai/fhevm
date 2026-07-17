use std::{env, path::PathBuf};

fn main() {
    // Default BUILD_STACK_VERSION so the feature's env!() never fails to compile when unset.
    let stack_version = env::var("BUILD_STACK_VERSION")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "0.14.0".to_string());
    println!("cargo:rustc-env=BUILD_STACK_VERSION={stack_version}");
    println!("cargo:rerun-if-env-changed=BUILD_STACK_VERSION");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("common_descriptor.bin"))
        .compile_protos(&["../../proto/common.proto"], &["../../proto"])
        .unwrap();
}
