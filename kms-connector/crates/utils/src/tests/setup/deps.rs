use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::LazyLock};

pub static ROOT_CARGO_TOML: LazyLock<CargoToml> =
    LazyLock::new(|| toml::from_str(include_str!("../../../../../Cargo.toml")).unwrap());

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoToml {
    pub workspace: Workspace,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub dependencies: HashMap<String, DependencyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencyValue {
    String(String),
    Object {
        version: Option<String>,
        tag: Option<String>,
        rev: Option<String>,
    },
}

impl CargoToml {
    pub fn get_gateway_bindings_version(&self) -> String {
        self.workspace
            .dependencies
            .get("fhevm_gateway_rust_bindings")
            .unwrap()
            .get_version()
    }

    pub fn get_kms_grpc_version(&self) -> String {
        self.workspace
            .dependencies
            .get("kms-grpc")
            .unwrap()
            .get_version()
    }
}

impl DependencyValue {
    pub fn get_version(&self) -> String {
        match self {
            DependencyValue::String(version) => version.clone(),
            DependencyValue::Object { version, tag, rev } => {
                if let Some(version) = version {
                    version.clone()
                } else if let Some(tag) = tag {
                    tag.clone()
                } else if let Some(rev) = rev {
                    rev[0..7].to_string()
                } else {
                    panic!("Either `version`, `tag` or `rev` should be configured");
                }
            }
        }
    }
}
