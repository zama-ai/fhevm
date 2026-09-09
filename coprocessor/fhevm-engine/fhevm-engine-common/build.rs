use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=BUILD_CONSENSUS_VERSION");
    println!("cargo:rerun-if-env-changed=BUILD_STACK_VERSION");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // tonic 0.14 moved prost codegen out of tonic-build into tonic-prost-build.
    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("common_descriptor.bin"))
        .compile_protos(&["../../proto/common.proto"], &["../../proto"])
        .unwrap();
}
