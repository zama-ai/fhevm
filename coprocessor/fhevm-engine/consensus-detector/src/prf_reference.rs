//! Compares the PRF reference digests of all operators, and writes the agreement or the mismatch.

use crate::s3::S3Service;
use alloy::{
    network::Ethereum,
    primitives::{Address, B256},
    providers::Provider,
};
use anyhow::Context;
use ciphertext_attestation::{
    consensus::Attestation, PrfOutputAttestation, PrfOutputRef, S3_METADATA_ATTESTATION_HEADER,
};
use fhevm_engine_common::prf_reference::{reference_digest, reference_key, reference_outputs};
use sqlx::PgPool;
use std::{collections::HashSet, time::Duration};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    Agreed(B256),
    Mismatch,
    /// An operator has no valid reference objects yet.
    Waiting,
}

/// Compares the new instances at each `interval`, with the members at the latest block.
pub async fn run<P: Provider<Ethereum>>(
    pool: PgPool,
    http: reqwest::Client,
    gateway: S3Service<P>,
    interval: Duration,
    cancel: CancellationToken,
) {
    loop {
        tokio::select! {
            () = cancel.cancelled() => return,
            () = tokio::time::sleep(interval) => {}
        }
        let checked = async {
            if new_instances(&pool).await?.is_empty() {
                return Ok(());
            }
            let members = gateway.get_current_members().await?;
            check(&pool, &http, &members).await
        };
        if let Err(e) = checked.await {
            error!(error = %e, "PRF reference comparison failed");
        }
    }
}

/// Compares each instance without an agreed digest, and writes the result.
pub async fn check(
    pool: &PgPool,
    http: &reqwest::Client,
    members: &[(Address, String)],
) -> anyhow::Result<()> {
    for prf_instance in new_instances(pool).await? {
        match compare(http, members, prf_instance).await {
            Comparison::Agreed(digest) => {
                sqlx::query(
                    "UPDATE prf_lanes SET reference_digest = $2, reference_mismatch_at = NULL
                      WHERE prf_instance = $1 AND reference_digest IS NULL",
                )
                .bind(i32::from(prf_instance))
                .bind(digest.as_slice())
                .execute(pool)
                .await?;
                info!(prf_instance, %digest, "Operators agree on the PRF reference digest");
            }
            Comparison::Mismatch => {
                sqlx::query(
                    "UPDATE prf_lanes
                        SET reference_mismatch_at = COALESCE(reference_mismatch_at, NOW())
                      WHERE prf_instance = $1 AND reference_digest IS NULL",
                )
                .bind(i32::from(prf_instance))
                .execute(pool)
                .await?;
                error!(
                    action = "review",
                    prf_instance, "Operators have other PRF reference digests"
                );
            }
            Comparison::Waiting => {
                debug!(prf_instance, "Waiting for the PRF reference objects");
            }
        }
    }
    Ok(())
}

async fn new_instances(pool: &PgPool) -> sqlx::Result<Vec<u16>> {
    let instances: Vec<i32> =
        sqlx::query_scalar("SELECT prf_instance FROM prf_lanes WHERE reference_digest IS NULL")
            .fetch_all(pool)
            .await?;
    Ok(instances
        .into_iter()
        .map(|i| u16::try_from(i).expect("prf_instance is in range"))
        .collect())
}

/// Two different digests are a mismatch, even if an operator has no digest yet.
pub async fn compare(
    http: &reqwest::Client,
    members: &[(Address, String)],
    prf_instance: u16,
) -> Comparison {
    let digests = futures::future::join_all(
        members
            .iter()
            .map(|(signer, url)| operator_digest(http, url, *signer, prf_instance)),
    )
    .await;
    let found: HashSet<B256> = digests.iter().flatten().copied().collect();
    match (found.len(), digests.iter().any(Option::is_none)) {
        (2.., _) => Comparison::Mismatch,
        (1, false) => Comparison::Agreed(*found.iter().next().expect("one digest")),
        _ => Comparison::Waiting,
    }
}

/// The reference digest of one operator. `None` if an object is missing or not validly signed.
async fn operator_digest(
    http: &reqwest::Client,
    bucket_url: &str,
    signer: Address,
    prf_instance: u16,
) -> Option<B256> {
    let mut pairs = Vec::new();
    for (label, pad_length) in reference_outputs() {
        let url = format!(
            "{}/{}",
            bucket_url.trim_end_matches('/'),
            reference_key(prf_instance, label, pad_length)
        );
        let attestation = match fetch_attestation(http, &url).await {
            Ok(Some(attestation)) => attestation,
            Ok(None) => {
                debug!(url, "No PRF reference object yet");
                return None;
            }
            Err(e) => {
                warn!(url, error = %e, "Cannot read the PRF reference object");
                return None;
            }
        };
        let subject = PrfOutputRef::new(prf_instance, label, pad_length);
        if let Err(e) = attestation.verify(&subject, signer) {
            warn!(url, %signer, error = %e, "PRF reference attestation is not valid");
            return None;
        }
        pairs.push((
            attestation.ciphertext_digest,
            attestation.sns_ciphertext_digest,
        ));
    }
    Some(reference_digest(pairs))
}

async fn fetch_attestation(
    http: &reqwest::Client,
    url: &str,
) -> anyhow::Result<Option<PrfOutputAttestation>> {
    let response = http.head(url).send().await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    let response = response.error_for_status()?;
    let header = response
        .headers()
        .get(S3_METADATA_ATTESTATION_HEADER)
        .context("no attestation header")?;
    Ok(Some(serde_json::from_slice(header.as_bytes())?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{primitives::keccak256, signers::local::PrivateKeySigner};
    use aws_sdk_s3::primitives::ByteStream;
    use ciphertext_attestation::{
        prf::Version, PrfOutputAttestationPayload, S3_METADATA_ATTESTATION_KEY,
    };
    use test_harness::{
        instance::{setup_test_db, ImportMode},
        localstack::{create_localstack_s3_client, start_localstack},
    };

    const PRF: u16 = 3;

    /// Uploads the reference objects of `bucket`, signed by `signer`. `bytes` sets the digests.
    async fn publish(s3: &aws_sdk_s3::Client, bucket: &str, signer: &PrivateKeySigner, bytes: u8) {
        for (label, pad_length) in reference_outputs() {
            let ct128 = vec![bytes; 4];
            let attestation = PrfOutputAttestationPayload::new(
                Version::V1,
                PRF,
                label,
                pad_length,
                keccak256([bytes, 64]),
                keccak256(&ct128),
            )
            .sign(signer)
            .await
            .unwrap();
            s3.put_object()
                .bucket(bucket)
                .key(reference_key(PRF, label, pad_length))
                .metadata(
                    S3_METADATA_ATTESTATION_KEY,
                    serde_json::to_string(&attestation).unwrap(),
                )
                .body(ByteStream::from(ct128))
                .send()
                .await
                .unwrap();
        }
    }

    async fn lane(pool: &PgPool) -> (Option<Vec<u8>>, bool) {
        sqlx::query_as(
            "SELECT reference_digest, reference_mismatch_at IS NOT NULL FROM prf_lanes
              WHERE prf_instance = $1",
        )
        .bind(i32::from(PRF))
        .fetch_one(pool)
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn writes_the_agreed_digest_or_the_mismatch() {
        let instance = setup_test_db(ImportMode::None).await.unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(instance.db_url())
            .await
            .unwrap();
        sqlx::query("INSERT INTO prf_lanes (prf_instance) VALUES ($1)")
            .bind(i32::from(PRF))
            .execute(&pool)
            .await
            .unwrap();
        let localstack = start_localstack().await.unwrap();
        let s3 = create_localstack_s3_client(localstack.host_port).await;
        let http = reqwest::Client::new();
        let signers: Vec<PrivateKeySigner> = (0..3).map(|_| PrivateKeySigner::random()).collect();
        let mut members = Vec::new();
        for (i, signer) in signers.iter().enumerate() {
            let bucket = format!("operator-{i}");
            s3.create_bucket().bucket(&bucket).send().await.unwrap();
            let url = format!("http://127.0.0.1:{}/{bucket}", localstack.host_port);
            members.push((signer.address(), url));
        }

        // Operator 2 has no objects yet: wait.
        publish(&s3, "operator-0", &signers[0], 1).await;
        publish(&s3, "operator-1", &signers[1], 1).await;
        check(&pool, &http, &members).await.unwrap();
        assert_eq!(lane(&pool).await, (None, false));

        // Objects signed by another signer are not valid: still wait.
        publish(&s3, "operator-2", &signers[0], 1).await;
        assert_eq!(compare(&http, &members, PRF).await, Comparison::Waiting);

        // Operator 2 has other bytes: mismatch.
        publish(&s3, "operator-2", &signers[2], 2).await;
        check(&pool, &http, &members).await.unwrap();
        assert_eq!(lane(&pool).await, (None, true));

        // Operator 2 now has the same bytes: the digest is agreed, and the mismatch is cleared.
        publish(&s3, "operator-2", &signers[2], 1).await;
        check(&pool, &http, &members).await.unwrap();
        let expected = reference_digest(
            reference_outputs()
                .iter()
                .map(|_| (keccak256([1, 64]), keccak256([1u8; 4]))),
        );
        assert_eq!(lane(&pool).await, (Some(expected.to_vec()), false));
    }
}
