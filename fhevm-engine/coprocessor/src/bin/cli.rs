use std::str::FromStr;

use clap::Parser;
use sqlx::types::Uuid;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Inserts tenant into specified database
    InsertTenant {
        /// PKS file path
        #[arg(long)]
        pks_file: String,
        /// SKS file path
        #[arg(long)]
        sks_file: String,
        /// Public params file path
        #[arg(long)]
        public_params_file: String,
        /// Tenant api key
        #[arg(long)]
        tenant_api_key: String,
        /// ACL contract address
        #[arg(long)]
        acl_contract_address: String,
        /// Input verifier address
        #[arg(long)]
        verifying_contract_address: String,
        /// Chain id
        #[arg(long)]
        chain_id: u32,
    },
}

fn main() {
    let args = Args::parse();
    match args {
        Args::InsertTenant { pks_file, sks_file, public_params_file, tenant_api_key, acl_contract_address, verifying_contract_address, chain_id } => {
            let db_url = std::env::var("DATABASE_URL")
                .expect("DATABASE_URL environment variable is undefined");
            let pks_file = std::fs::read(&pks_file)
                .expect("Can't read pks file");
            let sks_file = std::fs::read(&sks_file)
                .expect("Can't read pks file");
            let public_params_file = std::fs::read(&public_params_file)
                .expect("Can't read public params file");
            let _ = alloy::primitives::Address::from_str(&acl_contract_address)
                .expect("Can't parse acl contract adddress");
            let _ = alloy::primitives::Address::from_str(&verifying_contract_address)
                .expect("Can't parse input verifier adddress");
            let tenant_api_key = Uuid::from_str(&tenant_api_key).expect("Can't parse tenant api key");

            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    let pool = sqlx::postgres::PgPoolOptions::new()
                        .max_connections(1)
                        .connect(&db_url)
                        .await.expect("Can't connect to postgres instance");


                    sqlx::query!(
                        "
                            INSERT INTO tenants(
                                tenant_api_key,
                                chain_id,
                                acl_contract_address,
                                verifying_contract_address,
                                pks_key,
                                sks_key,
                                public_params
                            )
                            VALUES (
                                $1,
                                $2,
                                $3,
                                $4,
                                $5,
                                $6,
                                $7
                            )
                        ",
                        tenant_api_key,
                        chain_id as i32,
                        &acl_contract_address,
                        &verifying_contract_address,
                        &pks_file,
                        &sks_file,
                        &public_params_file
                    )
                    .execute(&pool)
                    .await
                    .expect("Can't insert new tenant");
                });
        },
    }
}