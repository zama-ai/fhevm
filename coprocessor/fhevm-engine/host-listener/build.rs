use foundry_compilers::{
    multi::MultiCompiler,
    solc::{Solc, SolcCompiler},
    Project, ProjectPathsConfig,
};
use semver::Version;
use serde_json::Value;
use std::{collections::HashMap, env, fs, path::Path, process::Command};

fn generate_zama_host_events() {
    generate_anchor_events(
        "zama_host.json",
        "zama_host_events.rs",
        "ZamaHostEvent",
    );
}

fn generate_confidential_token_events() {
    generate_anchor_events(
        "confidential_token.json",
        "confidential_token_events.rs",
        "ConfidentialTokenEvent",
    );
}

fn generate_anchor_events(
    idl_file: &str,
    output_file: &str,
    event_enum_name: &str,
) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let idl_path = manifest_dir.join("idl").join(idl_file);
    println!("cargo:rerun-if-changed={}", idl_path.display());

    let idl = fs::read_to_string(&idl_path).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", idl_path.display())
    });
    let idl: Value = serde_json::from_str(&idl).unwrap_or_else(|err| {
        panic!("failed to parse {}: {err}", idl_path.display())
    });

    let types = idl["types"]
        .as_array()
        .expect("Anchor IDL must contain types")
        .iter()
        .map(|ty| (ty["name"].as_str().expect("IDL type must have a name"), ty))
        .collect::<HashMap<_, _>>();

    let events = idl["events"]
        .as_array()
        .expect("Anchor IDL must contain events");
    let event_enum_types = event_enum_types(&types, events);

    let mut output = format!(
        r#"// Generated from `host-listener/idl/{idl_file}` by `host-listener/build.rs`.
// Do not edit by hand.

use sha2::{{Digest, Sha256}};

pub const EVENT_VERSION: u8 = 1;
pub const ANCHOR_EVENT_IX_TAG_LE: [u8; 8] = 0x1d9acb512ea545e4_u64.to_le_bytes();

"#,
    );

    for (enum_name, variants) in &event_enum_types {
        output.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n");
        output.push_str("pub enum ");
        output.push_str(enum_name);
        output.push_str(" {\n");
        for variant in variants {
            output.push_str("    ");
            output.push_str(variant);
            output.push_str(",\n");
        }
        output.push_str("}\n\n");
    }

    for event in events {
        let event_name =
            event["name"].as_str().expect("IDL event must have a name");
        let fields = fields_for_event(&types, event_name);
        output.push_str("#[derive(Clone, Debug, PartialEq, Eq)]\n");
        output.push_str("pub struct ");
        output.push_str(event_name);
        output.push_str(" {\n");
        for field in fields {
            output.push_str("    pub ");
            output.push_str(
                field["name"].as_str().expect("field must have a name"),
            );
            output.push_str(": ");
            output.push_str(&rust_type(&field["type"]));
            output.push_str(",\n");
        }
        output.push_str("}\n\n");
    }

    output.push_str("#[derive(Clone, Debug, PartialEq, Eq)]\n");
    output.push_str("pub enum ");
    output.push_str(event_enum_name);
    output.push_str(" {\n");
    for event in events {
        let event_name = event["name"].as_str().unwrap();
        output.push_str("    ");
        output.push_str(event_name.trim_end_matches("Event"));
        output.push('(');
        output.push_str(event_name);
        output.push_str("),\n");
    }
    output.push_str("}\n\n");

    output.push_str("pub fn decode_anchor_event(data: &[u8]) -> Option<");
    output.push_str(event_enum_name);
    output.push_str(
        r#"> {
    if data.len() < 8 {
        return None;
    }
    let (discriminator, payload) = data.split_at(8);

"#,
    );
    for event in events {
        let event_name = event["name"].as_str().unwrap();
        let discriminator = event["discriminator"]
            .as_array()
            .expect("IDL event must contain discriminator");
        output.push_str("    if discriminator == ");
        output.push_str(&byte_array(discriminator));
        output.push_str(" {\n        return decode_");
        output.push_str(&snake_case(event_name));
        output.push_str("(payload).map(");
        output.push_str(event_enum_name);
        output.push_str("::");
        output.push_str(event_name.trim_end_matches("Event"));
        output.push_str(");\n    }\n");
    }
    output.push_str("\n    None\n}\n\n");

    output.push_str("pub fn decode_anchor_cpi_event(data: &[u8]) -> Option<");
    output.push_str(event_enum_name);
    output.push_str(
        r#"> {
    decode_anchor_event(data.strip_prefix(&ANCHOR_EVENT_IX_TAG_LE)?)
}

"#,
    );

    output.push_str(
        r#"pub fn anchor_event_discriminator(name: &str) -> [u8; 8] {
    let digest = Sha256::digest(format!("event:{name}"));
    digest[..8].try_into().expect("slice has 8 bytes")
}

"#,
    );

    for event in events {
        let event_name = event["name"].as_str().unwrap();
        let fields = fields_for_event(&types, event_name);
        output.push_str("fn decode_");
        output.push_str(&snake_case(event_name));
        output.push_str("(payload: &[u8]) -> Option<");
        output.push_str(event_name);
        output.push_str(
            "> {\n    let mut cursor = Cursor::new(payload);\n    let event = ",
        );
        output.push_str(event_name);
        output.push_str(" {\n");
        for field in fields {
            let field_name = field["name"].as_str().unwrap();
            output.push_str("        ");
            output.push_str(field_name);
            output.push_str(": ");
            output.push_str(&read_expr(&field["type"]));
            output.push_str("?,\n");
        }
        output.push_str(
            "    };\n    cursor.is_finished().then_some(event)\n}\n\n",
        );
    }

    for (enum_name, variants) in &event_enum_types {
        output.push_str("fn read_");
        output.push_str(&snake_case(enum_name));
        output.push_str("(cursor: &mut Cursor<'_>) -> Option<");
        output.push_str(enum_name);
        output.push_str("> {\n");
        output.push_str("    match cursor.read_u8()? {\n");
        for (index, variant) in variants.iter().enumerate() {
            output.push_str("        ");
            output.push_str(&index.to_string());
            output.push_str(" => Some(");
            output.push_str(enum_name);
            output.push_str("::");
            output.push_str(variant);
            output.push_str("),\n");
        }
        output.push_str("        _ => None,\n    }\n}\n\n");
    }

    output.push_str(
        r#"struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn is_finished(&self) -> bool {
        self.offset == self.bytes.len()
    }

    fn read_u8(&mut self) -> Option<u8> {
        let byte = *self.bytes.get(self.offset)?;
        self.offset += 1;
        Some(byte)
    }

    fn read_bool(&mut self) -> Option<bool> {
        match self.read_u8()? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    // Used only by `read_vec_array`; unused in event modules without a `Vec<[u8; N]>` field.
    #[allow(dead_code)]
    fn read_u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.read_array::<4>()?))
    }

    fn read_u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.read_array::<8>()?))
    }

    fn read_array<const N: usize>(&mut self) -> Option<[u8; N]> {
        let end = self.offset.checked_add(N)?;
        let bytes = self.bytes.get(self.offset..end)?;
        self.offset = end;
        bytes.try_into().ok()
    }

    // Borsh `Vec<[u8; N]>`: u32 little-endian length, then `len` fixed-size arrays.
    // Unused in event modules without a `Vec<[u8; N]>` field (e.g. confidential_token).
    #[allow(dead_code)]
    fn read_vec_array<const N: usize>(&mut self) -> Option<Vec<[u8; N]>> {
        let len = self.read_u32()? as usize;
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            out.push(self.read_array::<N>()?);
        }
        Some(out)
    }
}
"#,
    );

    let out_path =
        Path::new(&env::var("OUT_DIR").expect("OUT_DIR must be set"))
            .join(output_file);
    fs::write(&out_path, output).unwrap_or_else(|err| {
        panic!("failed to write {}: {err}", out_path.display())
    });
}

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

fn event_enum_types<'a>(
    types: &'a HashMap<&str, &'a Value>,
    events: &[Value],
) -> Vec<(&'a str, Vec<&'a str>)> {
    let mut enum_names = Vec::<&str>::new();
    for event in events {
        let event_name = event["name"].as_str().expect("event must have name");
        for field in fields_for_event(types, event_name) {
            collect_defined_event_enums(types, &field["type"], &mut enum_names);
        }
    }
    enum_names
        .into_iter()
        .map(|name| {
            let enum_type = types
                .get(name)
                .unwrap_or_else(|| panic!("IDL must define enum {name}"));
            let variants = enum_type["type"]["variants"]
                .as_array()
                .unwrap_or_else(|| panic!("IDL type {name} must be an enum"))
                .iter()
                .map(|variant| {
                    if variant.get("fields").is_some() {
                        panic!("event enum {name} must use fieldless variants")
                    }
                    variant["name"]
                        .as_str()
                        .expect("enum variant must have a name")
                })
                .collect();
            (name, variants)
        })
        .collect()
}

fn collect_defined_event_enums<'a>(
    types: &'a HashMap<&str, &'a Value>,
    idl_type: &'a Value,
    enum_names: &mut Vec<&'a str>,
) {
    if let Some(defined) = idl_type["defined"]["name"].as_str() {
        let ty = types
            .get(defined)
            .unwrap_or_else(|| panic!("IDL must define {defined}"));
        if ty["type"]["kind"].as_str() == Some("enum")
            && !enum_names.contains(&defined)
        {
            enum_names.push(defined);
        }
    }
}

fn fields_for_event<'a>(
    types: &'a HashMap<&str, &'a Value>,
    event_name: &str,
) -> &'a Vec<Value> {
    types
        .get(event_name)
        .and_then(|ty| ty["type"]["fields"].as_array())
        .unwrap_or_else(|| {
            panic!("IDL event type {event_name} must define fields")
        })
}

fn rust_type(idl_type: &Value) -> String {
    if let Some(primitive) = idl_type.as_str() {
        return match primitive {
            "u8" => "u8".to_string(),
            "u64" => "u64".to_string(),
            "bool" => "bool".to_string(),
            "pubkey" => "[u8; 32]".to_string(),
            other => panic!("unsupported IDL primitive type {other}"),
        };
    }
    if let Some(array) = idl_type["array"].as_array() {
        let element =
            array[0].as_str().expect("array element must be primitive");
        let len = array[1].as_u64().expect("array length must be integer");
        if element != "u8" {
            panic!("unsupported IDL array element type {element}");
        }
        return format!("[u8; {len}]");
    }
    if let Some(vec_inner) = idl_type.get("vec") {
        if let Some(array) = vec_inner["array"].as_array() {
            let element = array[0]
                .as_str()
                .expect("vec array element must be primitive");
            let len =
                array[1].as_u64().expect("vec array length must be integer");
            if element != "u8" {
                panic!("unsupported IDL vec array element type {element}");
            }
            return format!("Vec<[u8; {len}]>");
        }
        if let Some(defined) = vec_inner["defined"]["name"].as_str() {
            return format!("Vec<{defined}>");
        }
        panic!("unsupported IDL vec inner type {vec_inner}");
    }
    if let Some(defined) = idl_type["defined"]["name"].as_str() {
        return defined.to_string();
    }
    panic!("unsupported IDL type {idl_type}");
}

fn read_expr(idl_type: &Value) -> String {
    if let Some(primitive) = idl_type.as_str() {
        return match primitive {
            "u8" => "cursor.read_u8()".to_string(),
            "u64" => "cursor.read_u64()".to_string(),
            "bool" => "cursor.read_bool()".to_string(),
            "pubkey" => "cursor.read_array::<32>()".to_string(),
            other => panic!("unsupported IDL primitive type {other}"),
        };
    }
    if let Some(array) = idl_type["array"].as_array() {
        let element =
            array[0].as_str().expect("array element must be primitive");
        let len = array[1].as_u64().expect("array length must be integer");
        if element != "u8" {
            panic!("unsupported IDL array element type {element}");
        }
        return format!("cursor.read_array::<{len}>()");
    }
    if let Some(vec_inner) = idl_type.get("vec") {
        if let Some(array) = vec_inner["array"].as_array() {
            let element = array[0]
                .as_str()
                .expect("vec array element must be primitive");
            let len =
                array[1].as_u64().expect("vec array length must be integer");
            if element != "u8" {
                panic!("unsupported IDL vec array element type {element}");
            }
            return format!("cursor.read_vec_array::<{len}>()");
        }
        if let Some(defined) = vec_inner["defined"]["name"].as_str() {
            return format!("cursor.read_vec(read_{})", snake_case(defined));
        }
        panic!("unsupported IDL vec inner type {vec_inner}");
    }
    if let Some(defined) = idl_type["defined"]["name"].as_str() {
        return format!("read_{}(&mut cursor)", snake_case(defined));
    }
    panic!("unsupported IDL type {idl_type}");
}

fn byte_array(values: &[Value]) -> String {
    let bytes = values
        .iter()
        .map(|value| {
            value
                .as_u64()
                .expect("discriminator byte must be integer")
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{bytes}]")
}

fn snake_case(name: &str) -> String {
    let mut out = String::new();
    for (index, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if index != 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
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

/// zama-host instructions whose raw (discriminator + borsh args) data we decode
/// off-chain to reconstruct events without relying on `emit_cpi!`/`emit!`. Curated
/// so the generator never trips over arg types used only by unrelated instructions;
/// a listed instruction with an unexpected arg type fails the build loudly.
///
/// Empty: the MMR/EncryptedValue rewrite removed every instruction this once
/// decoded (`trivial_encrypt_and_bind`, `fhe_binary_op*`, `fhe_ternary_op*`,
/// `fhe_rand*`, `allow_for_decryption`, `allow_acl_subjects`,
/// `commit_handle_material`). `fhe_eval` is decoded directly from its own
/// program type in `solana_reconstruct.rs`, not through this allowlist;
/// `EncryptedValue` instructions are decoded via their own discriminators.
const ZAMA_HOST_INSTRUCTION_ALLOWLIST: &[&str] = &[];

fn generate_zama_host_instructions() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let idl_path = manifest_dir.join("idl").join("zama_host.json");
    println!("cargo:rerun-if-changed={}", idl_path.display());
    let idl = fs::read_to_string(&idl_path).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", idl_path.display())
    });
    let idl: Value = serde_json::from_str(&idl).unwrap_or_else(|err| {
        panic!("failed to parse {}: {err}", idl_path.display())
    });

    let types = idl["types"]
        .as_array()
        .expect("Anchor IDL must contain types")
        .iter()
        .map(|ty| (ty["name"].as_str().expect("IDL type must have a name"), ty))
        .collect::<HashMap<_, _>>();

    let instructions = idl["instructions"]
        .as_array()
        .expect("Anchor IDL must contain instructions")
        .iter()
        .filter(|ins| {
            ZAMA_HOST_INSTRUCTION_ALLOWLIST
                .contains(&ins["name"].as_str().unwrap_or_default())
        })
        .collect::<Vec<_>>();
    assert_eq!(
        instructions.len(),
        ZAMA_HOST_INSTRUCTION_ALLOWLIST.len(),
        "every allowlisted zama-host instruction must exist in the IDL"
    );

    // Defined enums + structs referenced by the allowlisted args (in first-seen order).
    let mut enums = Vec::<&str>::new();
    let mut structs = Vec::<&str>::new();
    for ins in &instructions {
        for arg in ins["args"].as_array().expect("instruction args") {
            collect_defined_types(
                &types,
                &arg["type"],
                &mut enums,
                &mut structs,
            );
        }
    }
    let needs_bool = instructions.iter().any(|ins| {
        ins["args"]
            .as_array()
            .expect("instruction args")
            .iter()
            .any(|arg| idl_type_uses_primitive(&types, &arg["type"], "bool"))
    });
    let needs_u64 = instructions.iter().any(|ins| {
        ins["args"]
            .as_array()
            .expect("instruction args")
            .iter()
            .any(|arg| idl_type_uses_primitive(&types, &arg["type"], "u64"))
    });

    let mut output = String::from(
        "// Generated from `host-listener/idl/zama_host.json` by `host-listener/build.rs`.\n// Do not edit by hand.\n\n",
    );

    for name in &enums {
        output.push_str(
            "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum ",
        );
        output.push_str(name);
        output.push_str(" {\n");
        for variant in enum_variants(&types, name) {
            output.push_str("    ");
            output.push_str(variant);
            output.push_str(",\n");
        }
        output.push_str("}\n\n");
    }

    for name in &structs {
        output.push_str("#[derive(Clone, Debug, PartialEq, Eq)]\npub struct ");
        output.push_str(name);
        output.push_str(" {\n");
        for field in fields_for_event(&types, name) {
            output.push_str("    pub ");
            output.push_str(field["name"].as_str().expect("field name"));
            output.push_str(": ");
            output.push_str(&rust_type(&field["type"]));
            output.push_str(",\n");
        }
        output.push_str("}\n\n");
    }

    for ins in &instructions {
        let pascal = pascal_case(ins["name"].as_str().unwrap());
        output.push_str("#[derive(Clone, Debug, PartialEq, Eq)]\npub struct ");
        output.push_str(&pascal);
        output.push_str("Args {\n");
        for arg in ins["args"].as_array().unwrap() {
            output.push_str("    pub ");
            output.push_str(arg["name"].as_str().expect("arg name"));
            output.push_str(": ");
            output.push_str(&rust_type(&arg["type"]));
            output.push_str(",\n");
        }
        output.push_str("}\n\n");
    }

    output.push_str("#[derive(Clone, Debug, PartialEq, Eq)]\npub enum ZamaHostInstruction {\n");
    for ins in &instructions {
        let pascal = pascal_case(ins["name"].as_str().unwrap());
        output.push_str("    ");
        output.push_str(&pascal);
        output.push('(');
        output.push_str(&pascal);
        output.push_str("Args),\n");
    }
    output.push_str("}\n\n");

    output.push_str(
        "pub fn decode_zama_host_instruction(data: &[u8]) -> Option<ZamaHostInstruction> {\n    if data.len() < 8 {\n        return None;\n    }\n    let (discriminator, payload) = data.split_at(8);\n\n",
    );
    for ins in &instructions {
        let name = ins["name"].as_str().unwrap();
        let pascal = pascal_case(name);
        let discriminator = ins["discriminator"]
            .as_array()
            .expect("instruction must contain discriminator");
        output.push_str("    if discriminator == ");
        output.push_str(&byte_array(discriminator));
        output.push_str(" {\n        return decode_");
        output.push_str(name);
        output.push_str("_args(payload).map(ZamaHostInstruction::");
        output.push_str(&pascal);
        output.push_str(");\n    }\n");
    }
    output.push_str("\n    None\n}\n\n");

    for ins in &instructions {
        let name = ins["name"].as_str().unwrap();
        let pascal = pascal_case(name);
        output.push_str("fn decode_");
        output.push_str(name);
        output.push_str("_args(payload: &[u8]) -> Option<");
        output.push_str(&pascal);
        output.push_str("Args> {\n    let mut cursor = Cursor::new(payload);\n    let args = ");
        output.push_str(&pascal);
        output.push_str("Args {\n");
        for arg in ins["args"].as_array().unwrap() {
            output.push_str("        ");
            output.push_str(arg["name"].as_str().unwrap());
            output.push_str(": ");
            output.push_str(&read_expr(&arg["type"]));
            output.push_str("?,\n");
        }
        output.push_str(
            "    };\n    cursor.is_finished().then_some(args)\n}\n\n",
        );
    }

    for name in &enums {
        output.push_str("fn read_");
        output.push_str(&snake_case(name));
        output.push_str("(cursor: &mut Cursor<'_>) -> Option<");
        output.push_str(name);
        output.push_str("> {\n    match cursor.read_u8()? {\n");
        for (index, variant) in enum_variants(&types, name).iter().enumerate() {
            output.push_str("        ");
            output.push_str(&index.to_string());
            output.push_str(" => Some(");
            output.push_str(name);
            output.push_str("::");
            output.push_str(variant);
            output.push_str("),\n");
        }
        output.push_str("        _ => None,\n    }\n}\n\n");
    }

    for name in &structs {
        output.push_str("fn read_");
        output.push_str(&snake_case(name));
        output.push_str("(cursor: &mut Cursor<'_>) -> Option<");
        output.push_str(name);
        output.push_str("> {\n    Some(");
        output.push_str(name);
        output.push_str(" {\n");
        for field in fields_for_event(&types, name) {
            output.push_str("        ");
            output.push_str(field["name"].as_str().unwrap());
            output.push_str(": ");
            output.push_str(&read_expr(&field["type"]));
            output.push_str("?,\n");
        }
        output.push_str("    })\n}\n\n");
    }

    output.push_str(
        r#"struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn is_finished(&self) -> bool {
        self.offset == self.bytes.len()
    }

    fn read_u8(&mut self) -> Option<u8> {
        let byte = *self.bytes.get(self.offset)?;
        self.offset += 1;
        Some(byte)
    }

"#,
    );
    if needs_bool {
        output.push_str(
            r#"    fn read_bool(&mut self) -> Option<bool> {
        match self.read_u8()? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

"#,
        );
    }
    output.push_str(
        r#"    fn read_u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.read_array::<4>()?))
    }

"#,
    );
    if needs_u64 {
        output.push_str(
            r#"    fn read_u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.read_array::<8>()?))
    }

"#,
        );
    }
    output.push_str(
        r#"    fn read_array<const N: usize>(&mut self) -> Option<[u8; N]> {
        let end = self.offset.checked_add(N)?;
        let bytes = self.bytes.get(self.offset..end)?;
        self.offset = end;
        bytes.try_into().ok()
    }

    // Borsh `Vec<T>`: u32 little-endian length, then `len` elements decoded by `f`.
    fn read_vec<T>(
        &mut self,
        mut f: impl FnMut(&mut Cursor<'a>) -> Option<T>,
    ) -> Option<Vec<T>> {
        let len = self.read_u32()? as usize;
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            out.push(f(self)?);
        }
        Some(out)
    }
}
"#,
    );

    let out_path =
        Path::new(&env::var("OUT_DIR").expect("OUT_DIR must be set"))
            .join("zama_host_instructions.rs");
    fs::write(&out_path, output).unwrap_or_else(|err| {
        panic!("failed to write {}: {err}", out_path.display())
    });
}

/// Collects defined enums and structs referenced by an IDL type (recursing into
/// `vec` inners and struct fields), classifying each by its IDL `kind`.
fn collect_defined_types<'a>(
    types: &'a HashMap<&str, &'a Value>,
    idl_type: &'a Value,
    enums: &mut Vec<&'a str>,
    structs: &mut Vec<&'a str>,
) {
    if let Some(vec_inner) = idl_type.get("vec") {
        collect_defined_types(types, vec_inner, enums, structs);
        return;
    }
    if let Some(defined) = idl_type["defined"]["name"].as_str() {
        let ty = types
            .get(defined)
            .unwrap_or_else(|| panic!("IDL must define {defined}"));
        match ty["type"]["kind"].as_str() {
            Some("enum") => {
                if !enums.contains(&defined) {
                    enums.push(defined);
                }
            }
            Some("struct") => {
                if !structs.contains(&defined) {
                    structs.push(defined);
                    for field in fields_for_event(types, defined) {
                        collect_defined_types(
                            types,
                            &field["type"],
                            enums,
                            structs,
                        );
                    }
                }
            }
            other => {
                panic!("unsupported IDL defined kind {other:?} for {defined}")
            }
        }
    }
}

fn idl_type_uses_primitive(
    types: &HashMap<&str, &Value>,
    idl_type: &Value,
    primitive: &str,
) -> bool {
    if idl_type.as_str() == Some(primitive) {
        return true;
    }
    if let Some(vec_inner) = idl_type.get("vec") {
        return idl_type_uses_primitive(types, vec_inner, primitive);
    }
    if let Some(array) = idl_type["array"].as_array() {
        return array[0].as_str() == Some(primitive);
    }
    if let Some(defined) = idl_type["defined"]["name"].as_str() {
        let ty = types
            .get(defined)
            .unwrap_or_else(|| panic!("IDL must define {defined}"));
        return match ty["type"]["kind"].as_str() {
            Some("struct") => {
                fields_for_event(types, defined).iter().any(|field| {
                    idl_type_uses_primitive(types, &field["type"], primitive)
                })
            }
            Some("enum") => false,
            other => {
                panic!("unsupported IDL defined kind {other:?} for {defined}")
            }
        };
    }
    false
}

fn enum_variants<'a>(
    types: &'a HashMap<&str, &'a Value>,
    name: &str,
) -> Vec<&'a str> {
    types[name]["type"]["variants"]
        .as_array()
        .unwrap_or_else(|| panic!("IDL type {name} must be an enum"))
        .iter()
        .map(|variant| {
            if variant.get("fields").is_some() {
                panic!("enum {name} must use fieldless variants")
            }
            variant["name"]
                .as_str()
                .expect("enum variant must have a name")
        })
        .collect()
}

/// `allow_for_decryption` -> `AllowForDecryption`.
fn pascal_case(snake: &str) -> String {
    let mut out = String::new();
    let mut upper = true;
    for ch in snake.chars() {
        if ch == '_' {
            upper = true;
        } else if upper {
            out.push(ch.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

fn main() {
    println!("cargo::warning=build.rs run ...");
    generate_zama_host_events();
    generate_confidential_token_events();
    generate_zama_host_instructions();
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
