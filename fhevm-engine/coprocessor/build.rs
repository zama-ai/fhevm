use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("coprocessor_descriptor.bin"))
        .compile(&["../../proto/coprocessor.proto"], &["../../proto"])
        .unwrap();
}
