//! Called from each program's `build.rs`. Reads `solana/environments/<PROGRAM_ENVIRONMENT>.json`
//! and writes the program's `declare_id!` line to `OUT_DIR`, which `lib.rs` includes. Under
//! `solana/`, `.cargo/config.toml` forces the variable to `localnet`; elsewhere it defaults to
//! `localnet` when unset. See DESIGN_DECISIONS.md DD-053.

use std::path::{Path, PathBuf};

const DEFAULT_ENVIRONMENT: &str = "localnet";

/// `program` is the key under `"programs"` in the environment file, e.g. `zama_host`.
pub fn declare_program_id(program: &str) {
    println!("cargo:rerun-if-env-changed=PROGRAM_ENVIRONMENT");
    let name =
        std::env::var("PROGRAM_ENVIRONMENT").unwrap_or_else(|_| DEFAULT_ENVIRONMENT.to_owned());
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../environments")
        .join(format!("{name}.json"));
    println!("cargo:rerun-if-changed={}", path.display());
    if name != DEFAULT_ENVIRONMENT {
        println!("cargo:warning={program}: compiling the {name} program id");
    }
    let file = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "PROGRAM_ENVIRONMENT={name}: cannot read {}: {err}",
            path.display()
        )
    });
    let environment: serde_json::Value =
        serde_json::from_str(&file).unwrap_or_else(|err| panic!("{}: {err}", path.display()));
    let id = environment["programs"][program]
        .as_str()
        .unwrap_or_else(|| panic!("{} has no programs.{program}", path.display()));
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("program_id.rs");
    std::fs::write(&out, format!("anchor_lang::declare_id!(\"{id}\");\n"))
        .unwrap_or_else(|err| panic!("write {}: {err}", out.display()));
}
