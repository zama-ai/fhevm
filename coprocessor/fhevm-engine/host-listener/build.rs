use foundry_compilers::{
    multi::MultiCompiler,
    solc::{Solc, SolcCompiler},
    Project, ProjectPathsConfig,
};
use semver::Version;
use serde_json::Value;
use std::{collections::HashMap, env, fs, path::Path, process::Command};

fn generate_zama_host_events() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let idl_path = manifest_dir.join("idl/zama_host.json");
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
    let op_type = types
        .get("FheBinaryOpCode")
        .expect("ZamaHost IDL must define FheBinaryOpCode");
    let op_variants = op_type["type"]["variants"]
        .as_array()
        .expect("FheBinaryOpCode must be an enum")
        .iter()
        .map(|variant| {
            variant["name"]
                .as_str()
                .expect("enum variant must have a name")
        })
        .collect::<Vec<_>>();
    let ternary_op_type = types
        .get("FheTernaryOpCode")
        .expect("ZamaHost IDL must define FheTernaryOpCode");
    let ternary_op_variants = ternary_op_type["type"]["variants"]
        .as_array()
        .expect("FheTernaryOpCode must be an enum")
        .iter()
        .map(|variant| {
            variant["name"]
                .as_str()
                .expect("enum variant must have a name")
        })
        .collect::<Vec<_>>();

    let mut output = String::from(
        r#"// Generated from `host-listener/idl/zama_host.json` by `host-listener/build.rs`.
// Do not edit by hand.

use sha2::{Digest, Sha256};

pub const EVENT_VERSION: u8 = 0;
pub const ANCHOR_EVENT_IX_TAG_LE: [u8; 8] = 0x1d9acb512ea545e4_u64.to_le_bytes();

"#,
    );

    output.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n");
    output.push_str("pub enum FheBinaryOpCode {\n");
    for variant in &op_variants {
        output.push_str("    ");
        output.push_str(variant);
        output.push_str(",\n");
    }
    output.push_str("}\n\n");
    output.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n");
    output.push_str("pub enum FheTernaryOpCode {\n");
    for variant in &ternary_op_variants {
        output.push_str("    ");
        output.push_str(variant);
        output.push_str(",\n");
    }
    output.push_str("}\n\n");

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
    output.push_str("pub enum ZamaHostEvent {\n");
    for event in events {
        let event_name = event["name"].as_str().unwrap();
        output.push_str("    ");
        output.push_str(event_name.trim_end_matches("Event"));
        output.push('(');
        output.push_str(event_name);
        output.push_str("),\n");
    }
    output.push_str("}\n\n");

    output.push_str(
        r#"pub fn decode_anchor_event(data: &[u8]) -> Option<ZamaHostEvent> {
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
        output.push_str("(payload).map(ZamaHostEvent::");
        output.push_str(event_name.trim_end_matches("Event"));
        output.push_str(");\n    }\n");
    }
    output.push_str("\n    None\n}\n\n");

    output.push_str(
        r#"pub fn decode_anchor_cpi_event(data: &[u8]) -> Option<ZamaHostEvent> {
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

    output.push_str("fn read_fhe_binary_op_code(cursor: &mut Cursor<'_>) -> Option<FheBinaryOpCode> {\n");
    output.push_str("    match cursor.read_u8()? {\n");
    for (index, variant) in op_variants.iter().enumerate() {
        output.push_str("        ");
        output.push_str(&index.to_string());
        output.push_str(" => Some(FheBinaryOpCode::");
        output.push_str(variant);
        output.push_str("),\n");
    }
    output.push_str("        _ => None,\n    }\n}\n\n");
    output.push_str("fn read_fhe_ternary_op_code(cursor: &mut Cursor<'_>) -> Option<FheTernaryOpCode> {\n");
    output.push_str("    match cursor.read_u8()? {\n");
    for (index, variant) in ternary_op_variants.iter().enumerate() {
        output.push_str("        ");
        output.push_str(&index.to_string());
        output.push_str(" => Some(FheTernaryOpCode::");
        output.push_str(variant);
        output.push_str("),\n");
    }
    output.push_str("        _ => None,\n    }\n}\n\n");

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

    fn read_u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.read_array::<8>()?))
    }

    fn read_array<const N: usize>(&mut self) -> Option<[u8; N]> {
        let end = self.offset.checked_add(N)?;
        let bytes = self.bytes.get(self.offset..end)?;
        self.offset = end;
        bytes.try_into().ok()
    }
}
"#,
    );

    let out_path =
        Path::new(&env::var("OUT_DIR").expect("OUT_DIR must be set"))
            .join("zama_host_events.rs");
    fs::write(&out_path, output).unwrap_or_else(|err| {
        panic!("failed to write {}: {err}", out_path.display())
    });
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
    if let Some(defined) = idl_type["defined"]["name"].as_str() {
        return match defined {
            "FheBinaryOpCode" => {
                "read_fhe_binary_op_code(&mut cursor)".to_string()
            }
            "FheTernaryOpCode" => {
                "read_fhe_ternary_op_code(&mut cursor)".to_string()
            }
            other => panic!("unsupported IDL defined type {other}"),
        };
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
    generate_zama_host_events();
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
