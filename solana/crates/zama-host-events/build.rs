use serde_json::Value;
use std::{collections::HashMap, env, fs, path::PathBuf};

fn idl_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../coprocessor/fhevm-engine/host-listener/idl/zama_host.json")
}

fn main() {
    let idl_path = idl_path();
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

    let mut output = String::from(
        r#"// Generated from host-listener/idl/zama_host.json by zama-host-events/build.rs.
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

    for event in events {
        let event_name = event["name"].as_str().expect("IDL event must have a name");
        let fields = fields_for_event(&types, event_name);
        output.push_str("#[derive(Clone, Debug, PartialEq, Eq)]\n");
        output.push_str("pub struct ");
        output.push_str(event_name);
        output.push_str(" {\n");
        for field in fields {
            output.push_str("    pub ");
            output.push_str(field["name"].as_str().expect("field must have a name"));
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
        r#"pub fn decode_anchor_cpi_event(data: &[u8]) -> Option<ZamaHostEvent> {
    let data = data.strip_prefix(&ANCHOR_EVENT_IX_TAG_LE)?;
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
        output.push_str("> {\n    let mut cursor = Cursor::new(payload);\n    let event = ");
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
        output.push_str("    };\n    (event.version == EVENT_VERSION && cursor.is_finished()).then_some(event)\n}\n\n");
    }

    output.push_str(
        "fn read_fhe_binary_op_code(cursor: &mut Cursor<'_>) -> Option<FheBinaryOpCode> {\n",
    );
    output.push_str("    match cursor.read_u8()? {\n");
    for (index, variant) in op_variants.iter().enumerate() {
        output.push_str("        ");
        output.push_str(&index.to_string());
        output.push_str(" => Some(FheBinaryOpCode::");
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

    fn read_array<const N: usize>(&mut self) -> Option<[u8; N]> {
        let end = self.offset.checked_add(N)?;
        let bytes = self.bytes.get(self.offset..end)?;
        self.offset = end;
        bytes.try_into().ok()
    }
}
"#,
    );

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"))
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
        .unwrap_or_else(|| panic!("IDL event type {event_name} must define fields"))
}

fn rust_type(idl_type: &Value) -> String {
    if let Some(primitive) = idl_type.as_str() {
        return match primitive {
            "u8" => "u8".to_string(),
            "bool" => "bool".to_string(),
            other => panic!("unsupported IDL primitive type {other}"),
        };
    }
    if let Some(array) = idl_type["array"].as_array() {
        let element = array[0].as_str().expect("array element must be primitive");
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
            "bool" => "cursor.read_bool()".to_string(),
            other => panic!("unsupported IDL primitive type {other}"),
        };
    }
    if let Some(array) = idl_type["array"].as_array() {
        let element = array[0].as_str().expect("array element must be primitive");
        let len = array[1].as_u64().expect("array length must be integer");
        if element != "u8" {
            panic!("unsupported IDL array element type {element}");
        }
        return format!("cursor.read_array::<{len}>()");
    }
    if let Some(defined) = idl_type["defined"]["name"].as_str() {
        return match defined {
            "FheBinaryOpCode" => "read_fhe_binary_op_code(&mut cursor)".to_string(),
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
