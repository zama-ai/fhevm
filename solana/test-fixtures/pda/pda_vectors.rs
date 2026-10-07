use anchor_lang::prelude::Pubkey;
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::PathBuf};

pub struct PdaVectors {
    path: PathBuf,
    pub fixture: Value,
}

impl PdaVectors {
    pub fn load() -> Self {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../test-fixtures/pda/pda_v1.json");
        let fixture = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        Self { path, fixture }
    }

    pub fn key(&self, name: &str) -> Pubkey {
        self.fixture["inputs"][name]
            .as_str()
            .unwrap()
            .parse()
            .unwrap()
    }

    pub fn check(mut self, program: &str, id: Pubkey, pdas: &[(&str, (Pubkey, u8))]) {
        let pdas: BTreeMap<_, _> = pdas
            .iter()
            .map(|(name, (address, bump))| {
                (
                    *name,
                    json!({ "address": address.to_string(), "bump": bump }),
                )
            })
            .collect();
        let actual = json!({ "id": id.to_string(), "pdas": pdas });
        if std::env::var_os("ZAMA_UPDATE_PDA_VECTORS").is_some() {
            self.fixture["programs"][program] = actual;
            std::fs::write(
                &self.path,
                serde_json::to_string_pretty(&self.fixture).unwrap() + "\n",
            )
            .unwrap();
        } else {
            assert_eq!(self.fixture["programs"][program], actual);
        }
    }
}
