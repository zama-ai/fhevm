use std::str::FromStr;

use alloy::primitives::Address;
use fhevm_engine_common::tfhe_ops::{current_ciphertext_version, try_expand_ciphertext_list};
use fhevm_engine_common::utils::safe_serialize;
use serial_test::serial;
use sha3::{Digest, Keccak256};
use tfhe::integer::{bigint::StaticUnsignedBigInt, U256};

use crate::db_queries::query_tenant_keys;
use crate::tests::utils::{decrypt_ciphertexts, setup_test_app};

fn derive_handle(
    blob_hash: &[u8],
    ct_idx: usize,
    acl_address: &Address,
    chain_id: u64,
    ct_type: i16,
    ciphertext_version: i16,
) -> Vec<u8> {
    let mut handle_hash = Keccak256::new();
    handle_hash.update(blob_hash);
    handle_hash.update([ct_idx as u8]);
    handle_hash.update(acl_address.as_slice());
    handle_hash.update(chain_id.to_be_bytes());
    let mut handle = handle_hash.finalize().to_vec();
    handle[29] = ct_idx as u8;
    handle[30] = ct_type as u8;
    handle[31] = ciphertext_version as u8;
    handle
}

#[tokio::test]
#[serial(db)]
async fn test_compact_input_list_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let keys = query_tenant_keys(vec![1], &pool)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e })?;
    let keys = &keys[0];
    tfhe::set_server_key(keys.sks.clone());

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
    let the_list = builder
        .push(false)
        .push(1u8)
        .push(2u16)
        .push(3u32)
        .push(4u64)
        .push(5u128)
        .push(U256::from(7u32))
        .push(StaticUnsignedBigInt::<8>::from(8u32))
        .push(StaticUnsignedBigInt::<16>::from(9u32))
        .push(StaticUnsignedBigInt::<32>::from(10u32))
        .build_with_proof_packed(&keys.public_params, &[], tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();

    let serialized = safe_serialize(&the_list);
    let blob_hash = Keccak256::digest(&serialized).to_vec();
    let expanded = try_expand_ciphertext_list(&serialized, keys.public_params.as_ref()).unwrap();
    assert_eq!(expanded.len(), 10);

    let cipher_version = current_ciphertext_version();
    let acl_address = Address::from_str(&keys.acl_contract_address)?;
    let chain_id = keys.chain_id as u64;
    let tenant_id = keys.tenant_id;

    let mut tx = pool.begin().await?;
    sqlx::query(
        "
            INSERT INTO input_blobs(tenant_id, blob_hash, blob_data, blob_ciphertext_count)
            VALUES($1, $2, $3, $4)
            ON CONFLICT (tenant_id, blob_hash) DO NOTHING
        ",
    )
    .bind(tenant_id)
    .bind(&blob_hash)
    .bind(&serialized)
    .bind(expanded.len() as i32)
    .execute(tx.as_mut())
    .await?;

    let mut handles = Vec::with_capacity(expanded.len());
    for (ct_idx, the_ct) in expanded.into_iter().enumerate() {
        let (ct_type, ct_bytes) = the_ct.compress();
        let handle = derive_handle(
            &blob_hash,
            ct_idx,
            &acl_address,
            chain_id,
            ct_type,
            cipher_version,
        );
        sqlx::query(
            "
                INSERT INTO ciphertexts(
                    tenant_id,
                    handle,
                    ciphertext,
                    ciphertext_version,
                    ciphertext_type,
                    input_blob_hash,
                    input_blob_index
                )
                VALUES($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (tenant_id, handle, ciphertext_version) DO NOTHING
            ",
        )
        .bind(tenant_id)
        .bind(&handle)
        .bind(&ct_bytes)
        .bind(cipher_version)
        .bind(ct_type)
        .bind(&blob_hash)
        .bind(ct_idx as i32)
        .execute(tx.as_mut())
        .await?;
        handles.push(handle);
    }
    tx.commit().await?;

    let resp = decrypt_ciphertexts(&pool, tenant_id, handles).await?;
    assert_eq!(resp.len(), 10);

    assert_eq!(resp[0].output_type, 0);
    assert_eq!(resp[0].value, "false");
    assert_eq!(resp[1].output_type, 2);
    assert_eq!(resp[1].value, "1");
    assert_eq!(resp[2].output_type, 3);
    assert_eq!(resp[2].value, "2");
    assert_eq!(resp[3].output_type, 4);
    assert_eq!(resp[3].value, "3");
    assert_eq!(resp[4].output_type, 5);
    assert_eq!(resp[4].value, "4");
    assert_eq!(resp[5].output_type, 6);
    assert_eq!(resp[5].value, "5");
    assert_eq!(resp[6].output_type, 8);
    assert_eq!(resp[6].value, "7");
    assert_eq!(resp[7].output_type, 9);
    assert_eq!(resp[7].value, "8");
    assert_eq!(resp[8].output_type, 10);
    assert_eq!(resp[8].value, "9");
    assert_eq!(resp[9].output_type, 11);
    assert_eq!(resp[9].value, "10");

    Ok(())
}
