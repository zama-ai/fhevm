//! Opt-in omission or divergence of one host track's eligible publication.
#[derive(Clone, Copy)]
pub enum Fault {
    Withhold,
    Diverge,
}

/// Alter only the test publication. Computed state and production comparison
/// remain unchanged, so the real all-operator anchor must reject this report.
///
/// On a JSON state-hash record the flip targets the first byte of the `fhe`
/// value, so the divergent object stays a well-formed record and exercises
/// divergence attribution ("fhe (ct64 compute)"); any other payload has its
/// first byte flipped. XOR 1 keeps a hex digit a printable JSON string byte.
pub fn divergent_hash(bytes: &mut [u8]) {
    const FHE_VALUE: &[u8] = b"\"fhe\":\"";
    let target = bytes
        .windows(FHE_VALUE.len())
        .position(|w| w == FHE_VALUE)
        .map(|pos| pos + FHE_VALUE.len())
        .filter(|&pos| pos < bytes.len())
        .unwrap_or(0);
    if let Some(byte) = bytes.get_mut(target) {
        *byte ^= 1;
    }
}

pub async fn fault(
    pool: &sqlx::PgPool,
    chain: i64,
    block: i64,
) -> Result<Option<Fault>, sqlx::Error> {
    let exists: bool = sqlx::query_scalar(
        "SELECT to_regclass('public.consensus_test_host_report_fault') IS NOT NULL",
    )
    .fetch_one(pool)
    .await?;
    if !exists {
        return Ok(None);
    }
    let mode: Option<String> = sqlx::query_scalar(include_str!("test_host_report.sql"))
        .bind(chain)
        .bind(block)
        .fetch_optional(pool)
        .await?;
    match mode.as_deref() {
        None => Ok(None),
        Some("withhold") => Ok(Some(Fault::Withhold)),
        Some("diverge") => Ok(Some(Fault::Diverge)),
        Some(other) => Err(sqlx::Error::Protocol(format!(
            "unknown host report fault {other}"
        ))),
    }
}

/// Persist restoration material before publishing a divergent object. This
/// table is outside the GCS schema, whose failed-proposal reset deletes rows.
/// `key` is the exact attempt-scoped S3 key the divergent object is published
/// at, so restoration targets the same object without re-deriving the attempt.
pub async fn journal(
    pool: &sqlx::PgPool,
    chain: i64,
    block: i64,
    original: &str,
    block_hash: &str,
    bucket: &str,
    key: &str,
) -> Result<(), sqlx::Error> {
    let updated = sqlx::query(include_str!("test_host_report_journal.sql"))
        .bind(chain)
        .bind(block.to_string())
        .bind(original)
        .bind(block_hash)
        .bind(bucket)
        .bind(key)
        .execute(pool)
        .await?;
    if updated.rows_affected() != 1 {
        return Err(sqlx::Error::Protocol(
            "host report recovery owner disappeared".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn divergent_report_preserves_length_and_changes_the_commitment() {
        let original = [42; 32];
        let mut report = original;
        super::divergent_hash(&mut report);
        assert_ne!(report, original);
        assert_eq!(report[0], original[0] ^ 1);
        assert_eq!(report[1..], original[1..]);
    }

    #[test]
    fn divergent_report_targets_the_fhe_component_of_a_record() {
        let record = br#"{"v":1,"fhe":"aabb","pbs":"ccdd"}"#.to_vec();
        let mut report = record.clone();
        super::divergent_hash(&mut report);
        assert_eq!(report.len(), record.len());
        assert_ne!(report, record);
        // Still well-formed and attributable: only the fhe value changed.
        let parsed: crate::state_hash::StateHashRecord = serde_json::from_slice(&report).unwrap();
        assert_ne!(parsed.fhe, "aabb");
        assert_eq!(parsed.pbs, "ccdd");
    }
}
