use foundry_compilers::{
    multi::MultiCompiler,
    solc::{Solc, SolcCompiler},
    Project, ProjectPathsConfig,
};
use semver::Version;
use serde_json::Value;
use std::{env, fs, path::Path};

fn generate_solana_abi_schema_hashes() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = manifest_dir.join("idl/solana_abi_golden.json");
    println!("cargo:rerun-if-changed={}", manifest_path.display());
    let manifest = fs::read_to_string(&manifest_path).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", manifest_path.display())
    });
    let manifest: Value =
        serde_json::from_str(&manifest).unwrap_or_else(|err| {
            panic!("failed to parse {}: {err}", manifest_path.display())
        });
    let schemas = manifest["schemas"]
        .as_array()
        .expect("Solana ABI manifest must contain schemas");
    let versions = manifest["event_versions"]
        .as_object()
        .expect("Solana ABI manifest must contain event_versions");

    let mut output = String::from(
        r#"// Generated from `host-listener/idl/solana_abi_golden.json` by `host-listener/build.rs`.
// Do not edit by hand.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaAbiSchema {
    pub program: &'static str,
    pub category: &'static str,
    pub name: &'static str,
    pub schema_hash_hex: &'static str,
    pub fixture_hex: Option<&'static str>,
    pub fixture_len: Option<usize>,
}

"#,
    );
    output.push_str("pub const SOLANA_EVENT_VERSIONS: &[(&str, u8)] = &[\n");
    for (program, version) in versions {
        output.push_str("    (");
        output.push_str(&format!("{program:?}"));
        output.push_str(", ");
        output.push_str(
            &version
                .as_u64()
                .expect("event version must be an integer")
                .to_string(),
        );
        output.push_str("),\n");
    }
    output.push_str("];\n\n");
    output.push_str("pub const SOLANA_ABI_SCHEMAS: &[SolanaAbiSchema] = &[\n");
    for schema in schemas {
        output.push_str("    SolanaAbiSchema {\n");
        for key in ["program", "category", "name"] {
            output.push_str("        ");
            output.push_str(key);
            output.push_str(": ");
            output.push_str(&format!(
                "{:?}",
                schema[key].as_str().expect("schema field must be string")
            ));
            output.push_str(",\n");
        }
        output.push_str("        schema_hash_hex: ");
        output.push_str(&format!(
            "{:?}",
            schema["schema_hash"]
                .as_str()
                .expect("schema hash must be string")
        ));
        output.push_str(",\n");
        if let Some(fixture_hex) = schema["fixture_hex"].as_str() {
            output.push_str("        fixture_hex: Some(");
            output.push_str(&format!("{fixture_hex:?}"));
            output.push_str("),\n");
            output.push_str("        fixture_len: Some(");
            output.push_str(
                &schema["fixture_len"]
                    .as_u64()
                    .expect("fixture_len must be an integer")
                    .to_string(),
            );
            output.push_str("),\n");
        } else {
            output.push_str("        fixture_hex: None,\n");
            output.push_str("        fixture_len: None,\n");
        }
        output.push_str("    },\n");
    }
    output.push_str("];\n");

    let out_path =
        Path::new(&env::var("OUT_DIR").expect("OUT_DIR must be set"))
            .join("solana_abi_schema_hashes.rs");
    fs::write(&out_path, output).unwrap_or_else(|err| {
        panic!("failed to write {}: {err}", out_path.display())
    });
}

// Compiles the Anvil mock contracts under contracts/ for the integration
// tests. The event/type sources they import are symlinks into
// host-contracts/contracts/; the Rust event bindings themselves are NOT
// generated here — they live in the committed host-contracts/rust_bindings
// crate, refreshed with `make -C host-contracts update-bindings` (CI's
// check-bindings target catches drift).
fn compile_test_contracts() {
    println!("cargo:rerun-if-env-changed=HOST_LISTENER_SKIP_TEST_CONTRACTS");
    // Image builds compile bins, not tests. The Anvil mocks symlink into
    // host-contracts/, which Docker no longer copies, so skip solc there.
    if env::var_os("HOST_LISTENER_SKIP_TEST_CONTRACTS").is_some() {
        println!(
            "cargo::warning=skipping test-contract solc (HOST_LISTENER_SKIP_TEST_CONTRACTS is set)"
        );
        return;
    }
    // Also skip when the symlink targets are absent (fs::metadata follows
    // symlinks), so builds without host-contracts sources don't panic on
    // dangling links even when the env var above was forgotten.
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let symlinked_sources = [
        "contracts/ACLEvents.sol",
        "contracts/FHEEvents.sol",
        "contracts/shared/FheType.sol",
    ];
    for source in symlinked_sources {
        if fs::metadata(manifest_dir.join(source)).is_err() {
            println!(
                "cargo::warning=skipping test-contract solc ({source} does not resolve; host-contracts sources are absent)"
            );
            return;
        }
    }

    let paths =
        ProjectPathsConfig::hardhat(Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();
    // Use a specific version due to an issue with libc and libstdc++ in the
    // rust Docker image we use to run it.
    let solc = Solc::find_or_install(&Version::new(0, 8, 28)).unwrap();
    let project = Project::builder()
        .paths(paths)
        .build(
            MultiCompiler::new(Some(SolcCompiler::Specific(solc)), None)
                .unwrap(),
        )
        .unwrap();
    let output = project.compile().unwrap();
    if output.has_compiler_errors() {
        eprintln!("{output}");
    }
    assert!(!output.has_compiler_errors());
}

fn main() {
    println!("cargo::warning=build.rs run ...");
    generate_solana_abi_schema_hashes();
    compile_test_contracts();
}
