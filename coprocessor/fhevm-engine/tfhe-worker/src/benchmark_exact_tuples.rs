use std::collections::HashSet;
use std::time::{Duration, Instant};

use sqlx::{
    postgres::{PgListener, PgPoolOptions},
    PgPool, Postgres, QueryBuilder,
};

pub type TerminalTuple = (Vec<u8>, Vec<u8>);
type ExactTerminalRow = (Vec<u8>, Vec<u8>, Option<bool>, Option<bool>, Option<String>);

#[derive(Clone, Debug)]
pub struct TerminalObservation {
    pub output_handle: Vec<u8>,
    pub transaction_id: Vec<u8>,
    pub is_completed: Option<bool>,
    pub is_error: Option<bool>,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct ExactTerminalPoll {
    pub completed: usize,
    pub missing: Vec<TerminalTuple>,
    pub incomplete: Vec<TerminalTuple>,
}

/// A warmed exact-tuple observer for the native main baseline.  It keeps
/// connection setup and LISTEN registration outside the measured interval,
/// while retaining tuple-paired completion checks on the legacy table.
pub struct ExactLegacyTerminalObserver {
    pool: PgPool,
    listener: PgListener,
}

impl ExactLegacyTerminalObserver {
    pub async fn connect(db_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(db_url)
            .await?;
        // Establish a checked-out query connection before the measured phase.
        sqlx::query("SELECT 1").execute(&pool).await?;
        let mut listener = PgListener::connect_with(&pool).await?;
        listener.listen("event_ciphertext_computed").await?;
        Ok(Self { pool, listener })
    }

    pub async fn wait_until_completed(
        &mut self,
        terminals: &[TerminalTuple],
        timeout: Duration,
        trace: bool,
    ) -> Result<Instant, Box<dyn std::error::Error>> {
        if terminals.is_empty() {
            return Err("main_block_baseline sample has no terminal legacy computations".into());
        }
        let expected = terminals.iter().cloned().collect::<HashSet<_>>();
        if expected.len() != terminals.len() {
            return Err("main_block_baseline sample supplied duplicate terminal tuples".into());
        }
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            let poll = query_exact_legacy_terminals(&self.pool, &expected).await?;
            if trace {
                eprintln!(
                    "MAIN_BLOCK_TRACE waiter_poll expected={} completed={} missing={} incomplete={}",
                    expected.len(),
                    poll.completed,
                    poll.missing.len(),
                    poll.incomplete.len(),
                );
            }
            if poll.missing.is_empty() && poll.incomplete.is_empty() {
                return Ok(Instant::now());
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(exact_terminal_timeout(&poll, timeout).into());
            }
            // Worker result writes publish this channel in the same transaction.
            // The short bounded wait recovers from a missed notification.
            let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
            let wait = remaining.min(Duration::from_millis(100));
            let _ = tokio::time::timeout(wait, self.listener.recv()).await;
        }
    }
}

/// Poll exact legacy terminal tuples through a shared pool. Each poll acquires
/// and releases its own autocommit connection so it can observe committed work
/// produced by another session.
pub async fn wait_for_exact_legacy_terminals(
    db_url: &str,
    terminals: &[TerminalTuple],
    timeout: Duration,
    trace: bool,
) -> Result<Instant, Box<dyn std::error::Error>> {
    if terminals.is_empty() {
        return Err("main_block_baseline sample has no terminal legacy computations".into());
    }
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db_url)
        .await?;
    let expected = terminals.iter().cloned().collect::<HashSet<_>>();
    if expected.len() != terminals.len() {
        return Err("main_block_baseline sample supplied duplicate terminal tuples".into());
    }
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let poll = query_exact_legacy_terminals(&pool, &expected).await?;
        if trace {
            eprintln!(
                "MAIN_BLOCK_TRACE waiter_poll expected={} completed={} missing={} incomplete={}",
                expected.len(),
                poll.completed,
                poll.missing.len(),
                poll.incomplete.len(),
            );
        }
        if poll.missing.is_empty() && poll.incomplete.is_empty() {
            return Ok(Instant::now());
        }
        if tokio::time::Instant::now() >= deadline {
            return Err(exact_terminal_timeout(&poll, timeout).into());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn query_exact_legacy_terminals(
    pool: &PgPool,
    expected: &HashSet<TerminalTuple>,
) -> Result<ExactTerminalPoll, Box<dyn std::error::Error>> {
    // Build the expected pairs as SQL rows. This keeps output_handle and
    // transaction_id paired at the database boundary instead of relying on
    // independent array membership or a broad count.
    let mut query =
        QueryBuilder::<Postgres>::new("WITH expected(output_handle, transaction_id) AS (VALUES ");
    {
        let mut values = query.separated(", ");
        for (handle, transaction_id) in expected {
            values
                .push("(")
                .push_bind_unseparated(handle)
                .push_unseparated(", ")
                .push_bind_unseparated(transaction_id)
                .push_unseparated(")");
        }
    }
    query.push(
        ") \
            SELECT expected.output_handle, expected.transaction_id, \
                   computations.is_completed, computations.is_error, computations.error_message \
            FROM expected \
            LEFT JOIN computations ON computations.host_chain_id = 42 \
                AND computations.output_handle = expected.output_handle \
                AND computations.transaction_id = expected.transaction_id",
    );

    // Do not retain a transaction/connection between polls: workers commit
    // through independent sessions, and every observation must be fresh.
    let mut connection = pool.acquire().await?;
    let rows: Vec<ExactTerminalRow> = query.build_query_as().fetch_all(&mut *connection).await?;
    drop(connection);
    let observations = rows
        .into_iter()
        .map(
            |(output_handle, transaction_id, is_completed, is_error, error_message)| {
                TerminalObservation {
                    output_handle,
                    transaction_id,
                    is_completed,
                    is_error,
                    error_message,
                }
            },
        )
        .collect::<Vec<_>>();
    classify_exact_terminal_rows(expected, &observations)
        .map_err(|error| format!("main_block_baseline exact-terminal query: {error}").into())
}

fn exact_terminal_timeout(poll: &ExactTerminalPoll, timeout: Duration) -> String {
    let missing = poll
        .missing
        .iter()
        .take(10)
        .map(|(handle, transaction_id)| {
            format!("{}/{}", hex::encode(handle), hex::encode(transaction_id))
        })
        .collect::<Vec<_>>();
    let incomplete = poll
        .incomplete
        .iter()
        .take(10)
        .map(|(handle, transaction_id)| {
            format!("{}/{}", hex::encode(handle), hex::encode(transaction_id))
        })
        .collect::<Vec<_>>();
    format!(
        "timed out after {}s waiting for exact main_block_baseline legacy terminals; missing(first 10)={missing:?}; incomplete(first 10)={incomplete:?}",
        timeout.as_secs()
    )
}

/// Classify only the requested `(output_handle, transaction_id)` pairs.
/// Callers must build observations from an exact tuple query; this guard also
/// rejects cross-pair results that broad independent-array matching can admit.
pub fn classify_exact_terminal_rows(
    expected: &HashSet<TerminalTuple>,
    rows: &[TerminalObservation],
) -> Result<ExactTerminalPoll, String> {
    let mut observed = HashSet::new();
    let mut completed = 0;
    let mut missing = Vec::new();
    let mut incomplete = Vec::new();
    for row in rows {
        let pair = (row.output_handle.clone(), row.transaction_id.clone());
        if !observed.insert(pair.clone()) {
            return Err(format!(
                "duplicate exact terminal {}/{}",
                hex::encode(&row.output_handle),
                hex::encode(&row.transaction_id)
            ));
        }
        if row.is_error == Some(true) {
            return Err(format!(
                "terminal {}/{} failed: {}",
                hex::encode(&row.output_handle),
                hex::encode(&row.transaction_id),
                row.error_message
                    .as_deref()
                    .unwrap_or("worker reported no error message")
            ));
        }
        if row.is_completed.is_none() {
            missing.push(pair);
        } else if row.is_completed == Some(true) {
            completed += 1;
        } else {
            incomplete.push(pair);
        }
    }
    if observed != *expected {
        return Err(format!(
            "unexpected exact terminal tuple set: expected={}, observed={}",
            expected.len(),
            observed.len()
        ));
    }
    Ok(ExactTerminalPoll {
        completed,
        missing,
        incomplete,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        classify_exact_terminal_rows, ExactLegacyTerminalObserver, TerminalObservation,
        TerminalTuple,
    };
    use std::collections::HashSet;
    use std::time::{Duration, Instant};
    use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
    use tokio::runtime::Runtime;

    #[test]
    fn rejects_overlapping_handle_transaction_cross_pairs() {
        let expected = HashSet::from([(vec![0x10], vec![0xa0]), (vec![0x20], vec![0xb0])]);
        let cross_pairs = vec![
            TerminalObservation {
                output_handle: vec![0x10],
                transaction_id: vec![0xb0],
                is_completed: Some(true),
                is_error: Some(false),
                error_message: None,
            },
            TerminalObservation {
                output_handle: vec![0x20],
                transaction_id: vec![0xa0],
                is_completed: Some(true),
                is_error: Some(false),
                error_message: None,
            },
        ];
        assert!(classify_exact_terminal_rows(&expected, &cross_pairs).is_err());
    }

    #[test]
    fn accepts_all_four_exact_completed_terminals() {
        let rows = vec![
            (vec![1], vec![11]),
            (vec![2], vec![11]),
            (vec![3], vec![22]),
            (vec![4], vec![22]),
        ]
        .into_iter()
        .map(|(output_handle, transaction_id)| TerminalObservation {
            output_handle,
            transaction_id,
            is_completed: Some(true),
            is_error: Some(false),
            error_message: None,
        })
        .collect::<Vec<_>>();
        let expected = rows
            .iter()
            .map(|row| (row.output_handle.clone(), row.transaction_id.clone()))
            .collect::<HashSet<_>>();
        let poll = classify_exact_terminal_rows(&expected, &rows).unwrap();
        assert_eq!(poll.completed, 4);
        assert!(poll.missing.is_empty());
        assert!(poll.incomplete.is_empty());
    }

    #[test]
    fn warmed_observer_observes_delayed_terminal_updates_with_tokio_runtime() {
        Runtime::new().unwrap().block_on(async {
            let container = GenericImage::new("postgres", "15.7")
                .with_wait_for(WaitFor::message_on_stderr(
                    "database system is ready to accept connections",
                ))
                .with_env_var("POSTGRES_USER", "postgres")
                .with_env_var("POSTGRES_PASSWORD", "postgres")
                .with_env_var("POSTGRES_DB", "postgres")
                .start()
                .await
                .expect("postgres test container started");
            let host = container.get_host().await.unwrap();
            let port = container.get_host_port_ipv4(5432).await.unwrap();
            let db_url = format!("postgresql://postgres:postgres@{host}:{port}/postgres");
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(2)
                .connect(&db_url)
                .await
                .unwrap();
            sqlx::query(
                "CREATE TABLE computations (\
                    host_chain_id BIGINT NOT NULL, \
                    output_handle BYTEA NOT NULL, \
                    transaction_id BYTEA NOT NULL, \
                    is_completed BOOLEAN NOT NULL DEFAULT FALSE, \
                    is_error BOOLEAN NOT NULL DEFAULT FALSE, \
                    error_message TEXT)",
            )
            .execute(&pool)
            .await
            .unwrap();

            let terminals: Vec<TerminalTuple> = vec![
                (vec![1], vec![11]),
                (vec![2], vec![11]),
                (vec![3], vec![22]),
                (vec![4], vec![22]),
            ];
            for (handle, transaction_id) in &terminals {
                sqlx::query(
                    "INSERT INTO computations \
                     (host_chain_id, output_handle, transaction_id, is_completed, is_error) \
                     VALUES (42, $1, $2, FALSE, FALSE)",
                )
                .bind(handle)
                .bind(transaction_id)
                .execute(&pool)
                .await
                .unwrap();
            }
            let mut observer = ExactLegacyTerminalObserver::connect(&db_url).await.unwrap();

            let updater_pool = pool.clone();
            let updater = tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(300)).await;
                sqlx::query("UPDATE computations SET is_completed = TRUE WHERE host_chain_id = 42")
                    .execute(&updater_pool)
                    .await
                    .unwrap();
                sqlx::query("SELECT pg_notify('event_ciphertext_computed', 'benchmark-test')")
                    .execute(&updater_pool)
                    .await
                    .unwrap();
            });
            let started = Instant::now();
            observer
                .wait_until_completed(&terminals, Duration::from_secs(5), false)
                .await
                .unwrap();
            updater.await.unwrap();
            assert!(started.elapsed() >= Duration::from_millis(250));
        });
    }
}
