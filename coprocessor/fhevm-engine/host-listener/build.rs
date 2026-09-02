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

fn main() {
    generate_solana_abi_schema_hashes();
}
