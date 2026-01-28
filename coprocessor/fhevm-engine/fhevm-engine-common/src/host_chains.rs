use anyhow::Result;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

#[derive(Clone)]
pub struct HostChain {
    pub chain_id: i64,
    pub name: String,
    pub acl_contract_address: String,
}

#[derive(Clone)]
pub struct HostChainsCache {
    map: HashMap<i64, HostChain>,
}

impl HostChainsCache {
    pub async fn load(pool: &PgPool) -> Result<Self> {
        let rows = sqlx::query("SELECT chain_id, name, acl_contract_address FROM host_chains")
            .fetch_all(pool)
            .await?;

        let mut map = HashMap::with_capacity(rows.len());

        for row in rows {
            let chain = HostChain {
                chain_id: row.try_get("chain_id")?,
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

    pub fn get_chain(&self, chain_id: i64) -> Option<&HostChain> {
        self.map.get(&chain_id)
    }
}
