use foundry_compilers::{
    multi::MultiCompiler,
    solc::{Solc, SolcCompiler},
    Project, ProjectPathsConfig,
};
use semver::Version;
use serde_json::Value;
use std::{env, fs, path::Path, process::Command};

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

fn build_contracts() {
    println!(
        "cargo:rerun-if-changed=../../../host-contracts/contracts/ACL.sol"
    );
    println!(
        "cargo:rerun-if-changed=../../../host-contracts/contracts/ACLEvents.sol"
    );
    println!("cargo:rerun-if-changed=../../../host-contracts/contracts/FHEVMExecutor.sol");
    println!("cargo:rerun-if-changed=../../../host-contracts/contracts/KMSGeneration.sol");
    println!(
        "cargo:rerun-if-changed=../../../host-contracts/contracts/bridge/BridgeEvents.sol"
    );
    // Step 1: Copy ../../contracts/.env.example to ../../contracts/.env
    let env_example = Path::new("../../../host-contracts/.env.example");
    let env_dest = Path::new("../../../host-contracts/.env");
    let artefacts = Path::new("../../../host-contracts/artifacts");
    if env_example.exists() {
        // CI build
        if !env_dest.exists() {
            fs::copy(env_example, env_dest)
                .expect("Failed to copy .env.example to .env");
            println!("Copied .env.example to .env");
        }
    } else if artefacts.exists() {
        // Docker build
        println!("Assuming artefacts are up to date.");
        return;
    } else {
        panic!("Error: .env.example not found in contracts directory");
    }

    // Change to the contracts directory for npm commands.
    let contracts_dir = Path::new("../../../host-contracts");
    if !contracts_dir.exists() {
        panic!("Error: contracts directory not found");
    }
    env::set_current_dir(contracts_dir)
        .expect("Failed to change to contracts directory");

    // Step 2: Run `npm ci --include=optional` in ../../contracts
    let npm_ci_status = Command::new("npm")
        .args(["ci", "--include=optional"])
        .status()
        .expect("Failed to run npm ci");
    if !npm_ci_status.success() {
        panic!("Error: npm ci failed");
    }
    println!("Ran npm ci successfully");

    // Step 3: Run `HARDHAT_NETWORK=hardhat npm run deploy:emptyProxies
    // && npx hardhat compile` in ../../contracts
    let npm_run_status = Command::new("npm")
        .env("HARDHAT_NETWORK", "hardhat")
        .args(["run", "deploy:emptyProxies"])
        .status()
        .expect("Failed to run npm run");
    if !npm_run_status.success() {
        panic!("Error: npm tun failed");
    }
    println!("Ran npm run successfully");

    let hardhat_compile_status = Command::new("npx")
        .args(["hardhat", "compile"])
        .status()
        .expect("Failed to run npx hardhat compile");
    if !hardhat_compile_status.success() {
        panic!("Error: npx hardhat compile failed");
    }
    println!("Ran npx hardhat compile successfully");
}

fn main() {
    println!("cargo::warning=build.rs run ...");
    generate_solana_abi_schema_hashes();
    build_contracts();
    // build tests contracts
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
