use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;

use crate::types::CoprocessorError;
use sqlx::{query, Postgres};

/// Returns tenant id upon valid authorization request
pub async fn check_if_api_key_is_valid<T>(
    req: &tonic::Request<T>,
    pool: &sqlx::Pool<Postgres>,
) -> Result<i32, CoprocessorError> {
    match req.metadata().get("authorization") {
        Some(auth) => {
            let auth_header = String::from_utf8(auth.as_bytes().to_owned())
                .map_err(|_| CoprocessorError::Unauthorized)?
                .to_lowercase();

            let prefix = "bearer ";
            if !auth_header.starts_with(prefix) {
                return Err(CoprocessorError::Unauthorized);
            }

            let tail = &auth_header[prefix.len()..];
            let api_key = tail.trim();
            let api_key = match sqlx::types::Uuid::from_str(api_key) {
                Ok(uuid) => uuid,
                Err(_) => return Err(CoprocessorError::Unauthorized),
            };

            let tenant = query!(
                "SELECT tenant_id FROM tenants WHERE tenant_api_key = $1",
                api_key
            )
            .fetch_all(pool)
            .await
            .map_err(Into::<CoprocessorError>::into)?;

            if tenant.is_empty() {
                return Err(CoprocessorError::Unauthorized);
            }

            return Ok(tenant[0].tenant_id);
        }
        None => {
            return Err(CoprocessorError::Unauthorized);
        }
    }
}

/// Returns ciphertext types
pub async fn check_if_ciphertexts_exist_in_db(
    mut cts: BTreeSet<Vec<u8>>,
    tenant_id: i32,
    pool: &sqlx::Pool<Postgres>,
) -> Result<HashMap<Vec<u8>, i16>, CoprocessorError> {
    let handles_to_check_in_db_vec = cts.iter().cloned().collect::<Vec<_>>();
    let ciphertexts = query!(
        "
            SELECT handle, ciphertext_type
            FROM ciphertexts
            WHERE handle = ANY($1::BYTEA[])
            AND tenant_id = $2
        ",
        &handles_to_check_in_db_vec,
        tenant_id,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::<CoprocessorError>::into)?;

    let mut result = HashMap::with_capacity(cts.len());
    for ct in ciphertexts {
        assert!(cts.remove(&ct.handle), "any ciphertext selected must exist");
        assert!(result
            .insert(ct.handle.clone(), ct.ciphertext_type)
            .is_none());
    }

    if !cts.is_empty() {
        return Err(CoprocessorError::UnexistingInputCiphertextsFound(
            cts.into_iter()
                .map(|i| format!("0x{}", hex::encode(i)))
                .collect(),
        ));
    }

    Ok(result)
}
