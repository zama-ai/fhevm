use crate::chain_id::ChainId;
use anyhow::Result;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

#[derive(Clone)]
pub struct HostChain {
    pub chain_id: ChainId,
    pub name: String,
    pub acl_contract_address: String,
}

#[derive(Clone)]
pub struct HostChainsCache {
    map: HashMap<ChainId, HostChain>,
}

impl HostChainsCache {
    pub async fn load(pool: &PgPool) -> Result<Self> {
        let rows = sqlx::query("SELECT chain_id, name, acl_contract_address FROM host_chains")
            .fetch_all(pool)
            .await?;

        let mut map = HashMap::with_capacity(rows.len());

        for row in rows {
            let chain_id_raw: i64 = row.try_get("chain_id")?;
            let chain = HostChain {
                chain_id: ChainId::try_from(chain_id_raw)?,
                name: row.try_get("name")?,
                acl_contract_address: row.try_get("acl_contract_address")?,
            };
            map.insert(chain.chain_id, chain);
        }

        Ok(Self { map })
    }

    pub fn all(&self) -> Vec<&HostChain> {
        self.map.values().collect()
    }

    pub fn get_chain(&self, chain_id: ChainId) -> Option<&HostChain> {
        self.map.get(&chain_id)
    }
}
