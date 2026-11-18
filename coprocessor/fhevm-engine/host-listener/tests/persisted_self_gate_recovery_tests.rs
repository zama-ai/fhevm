//! Run the operator's SQL verbatim against charged and uncharged self-edges.
mod ingest_fixtures;

#[tokio::test]
async fn self_edge_runbook_preserves_real_gates_and_refuses_ambiguous_counts(
) -> anyhow::Result<()> {
    let (_db, pool, _instance) = ingest_fixtures::setup().await?;
    let section = include_str!("../../../docs/chain-history-recovery.md")
        .split("## CFI-0023 persisted self-edge recovery")
        .nth(1)
        .unwrap();
    let template = section
        .split("```sql\n")
        .nth(2)
        .unwrap()
        .split("```")
        .next()
        .unwrap();
    for (count, external, expected, repaired) in [
        (1_i32, true, 1, true), // self-edge merged without charging a gate
        (2, true, 1, true),     // self-edge and real parent both charged
        (1, false, 0, true),    // self-edge was the only gate
        (0, false, 0, true),    // self-edge was never counted
        (3, true, 3, false),    // unexplained extra gates require investigation
        (0, true, 0, false),    // missing real gate requires investigation
    ] {
        sqlx::query("DELETE FROM dependence_chain")
            .execute(&pool)
            .await?;
        let child = vec![0xc1_u8; 32];
        sqlx::query("INSERT INTO dependence_chain (dependence_chain_id, status, dependency_count, dependents)
                     VALUES ($1, 'updated', $2, ARRAY[$1]::bytea[])")
            .bind(&child).bind(count).execute(&pool).await?;
        if external {
            sqlx::query("INSERT INTO dependence_chain (dependence_chain_id, status, dependents)
                         VALUES ($1, 'updated', ARRAY[$2]::bytea[])")
                .bind(vec![0xb1_u8;32]).bind(&child).execute(&pool).await?;
        }
        let sql =
            template.replace("<chain-id-hex>", &alloy::hex::encode(&child));
        sqlx::raw_sql(&sql).execute(&pool).await?;
        let (remaining, self_edge): (i32, bool) = sqlx::query_as(
            "SELECT dependency_count, dependence_chain_id = ANY(dependents)
             FROM dependence_chain WHERE dependence_chain_id=$1",
        )
        .bind(&child)
        .fetch_one(&pool)
        .await?;
        assert_eq!(remaining, expected, "count={count}, external={external}");
        assert_eq!(!self_edge, repaired, "count={count}, external={external}");
    }
    Ok(())
}
