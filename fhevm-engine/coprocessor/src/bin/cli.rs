use std::str::FromStr;

use clap::Parser;
use coprocessor::server::{
    common::FheOperation,
    coprocessor::{
        fhevm_coprocessor_client::FhevmCoprocessorClient, AsyncComputation, AsyncComputationInput,
        AsyncComputeRequest, GetCiphertextBatch, TrivialEncryptBatch, TrivialEncryptRequestSingle,
    },
};
use rand::Rng;
use sqlx::types::Uuid;
use tonic::metadata::MetadataValue;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Inserts tenant into specified database
    InsertTenant {
        /// PKS file path
        #[arg(long)]
        pks_file: String,
        /// SKS file path (compressed SKS if GPU)
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
    /// Coprocessor smoke test
    SmokeTest {
        /// Tenant api key
        #[arg(long)]
        tenant_api_key: String,
        /// Coprocessor grpc url
        #[arg(long)]
        coprocessor_url: String,
    },
}

fn main() {
    let args = Args::parse();
    match args {
        Args::InsertTenant {
            pks_file,
            sks_file,
            public_params_file,
            tenant_api_key,
            acl_contract_address,
            verifying_contract_address,
            chain_id,
        } => {
            insert_tenant(
                pks_file,
                sks_file,
                public_params_file,
                tenant_api_key,
                acl_contract_address,
                verifying_contract_address,
                chain_id,
            );
        }
        Args::SmokeTest {
            tenant_api_key,
            coprocessor_url,
        } => {
            smoke_test(tenant_api_key, coprocessor_url);
        }
    }
}

fn smoke_test(tenant_api_key: String, coprocessor_url: String) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let mut client = FhevmCoprocessorClient::connect(coprocessor_url)
                .await.expect("Can't connect to coprocessor server");

            let api_key_header = format!("bearer {}", tenant_api_key);
            let handle_a = rand::rng().random::<u64>().to_be_bytes().to_vec();
            let handle_b = rand::rng().random::<u64>().to_be_bytes().to_vec();
            let output_handle = rand::rng().random::<u64>().to_be_bytes().to_vec();
            let num_a = rand::rng().random::<u32>().to_be_bytes().to_vec();
            let num_b = rand::rng().random::<u32>().to_be_bytes().to_vec();

            println!(
                "Trivially encrypting numbers 0x{} and 0x{} with handles 0x{} and 0x{}",
                hex::encode(&num_a), hex::encode(&num_b),
                hex::encode(&handle_a), hex::encode(&handle_b)
            );

            // trivial encryption
            let mut encrypt_request = tonic::Request::new(
                TrivialEncryptBatch {
                    values: vec![
                        TrivialEncryptRequestSingle {
                            handle: handle_a.clone(),
                            be_value: num_a,
                            output_type: 4, // 32 bit
                        },
                        TrivialEncryptRequestSingle {
                            handle: handle_b.clone(),
                            be_value: num_b,
                            output_type: 4, // 32 bit
                        },
                    ]
                }
            );
            encrypt_request.metadata_mut().append(
                "authorization",
                MetadataValue::from_str(&api_key_header).unwrap(),
            );
            let _res = client.trivial_encrypt_ciphertexts(encrypt_request)
                .await.expect("error while sending trivial encrypt request to coprocessor");

            // schedule computation
            println!(
                "Scheduling FheAdd computation with output handle of 0x{}",
                hex::encode(&output_handle),
            );
            let mut compute_request = tonic::Request::new(
                AsyncComputeRequest {
                    computations: vec![
                        AsyncComputation {
                            operation: FheOperation::FheAdd.into(),
                            output_handle: output_handle.clone(),
                            inputs: vec![
                                AsyncComputationInput {
                                    input: Some(coprocessor::server::coprocessor::async_computation_input::Input::InputHandle(handle_a.clone())),
                                },
                                AsyncComputationInput {
                                    input: Some(coprocessor::server::coprocessor::async_computation_input::Input::InputHandle(handle_b.clone())),
                                },
                            ]
                        },
                    ]
                }
            );
            compute_request.metadata_mut().append(
                "authorization",
                MetadataValue::from_str(&api_key_header).unwrap(),
            );
            let _res = client.async_compute(compute_request)
                .await.expect("error while scheduling computation in coprocessor");

            let wait_ms = 5000;
            println!("Waiting for computation to finish in {}ms", wait_ms);
            tokio::time::sleep(tokio::time::Duration::from_millis(wait_ms)).await;

            // retrieve ciphertext
            println!("Retrieving ciphertext with handle 0x{}", hex::encode(&output_handle));
            let mut get_ciphertext_request = tonic::Request::new(
                GetCiphertextBatch {
                    handles: vec![
                        output_handle,
                    ]
                }
            );
            get_ciphertext_request.metadata_mut().append(
                "authorization",
                MetadataValue::from_str(&api_key_header).unwrap(),
            );
            let res = client.get_ciphertexts(get_ciphertext_request)
                .await.expect("error while fetching ciphertexts from coprocessor");

            for resp in &res.get_ref().responses {
                match &resp.ciphertext {
                    Some(ct) => {
                        println!(
                            "Retrieved ciphertext with handle of 0x{} and value of 0x{}",
                            hex::encode(&resp.handle), hex::encode(&ct.ciphertext_bytes)
                        );
                    }
                    None => {
                        panic!(
                            "No ciphertext with handle of 0x{} exists",
                            hex::encode(&resp.handle),
                        );
                    }
                }
            }
        });
}

fn insert_tenant(
    pks_file: String,
    sks_file: String,
    public_params_file: String,
    tenant_api_key: String,
    acl_contract_address: String,
    verifying_contract_address: String,
    chain_id: u32,
) {
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is undefined");
    let pks_file = std::fs::read(&pks_file).expect("Can't read pks file");
    let sks_file = std::fs::read(&sks_file).expect("Can't read sks file (or csks if GPU)");
    let public_params_file =
        std::fs::read(&public_params_file).expect("Can't read public params file");
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
                .await
                .expect("Can't connect to postgres instance");

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
}
