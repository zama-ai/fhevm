//! The indexer's store check: every recorded store against its account on chain, once at start
//! and then every interval.
//!
//! Leaves are only appended, so the peaks of a store's first `n` leaves never change once the
//! record holds them. The check therefore compares at the chain's leaf count `n`, whatever slot
//! the chain was read at: a record holding fewer than `n` leaves is behind, and otherwise its
//! peaks at `n`, read from the recorded nodes, must equal the chain's. A store that disagrees is
//! quarantined, and the proof server answers it as inconsistent until a later check matches or
//! finds the store closed.
//! The KMS connector verifies every proof against the chain anyway; the quarantine saves it a
//! wrong answer and pages the operator.

use std::{
    collections::HashSet, future::Future, sync::LazyLock, time::Duration,
};

use prometheus::{
    register_int_counter_vec, register_int_gauge_vec, IntCounterVec, IntGauge,
    IntGaugeVec,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use zama_solana_acl::{account::AccountView, validate_store, StoreRejection};

use crate::{
    gauge_value,
    store::{
        lift_quarantine, load_peaks, load_quarantined_stores,
        load_served_store, load_store_page, quarantine_store,
    },
    unix_now_secs,
};

/// Stores per `getMultipleAccounts` call, its maximum.
const STORE_CHECK_PAGE: u16 = 100;

static STORE_CHECKS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "solana_merkle_indexer_store_checks_total",
        "Recorded stores compared with their account on chain, by result",
        &["host_chain_id", "result"]
    )
    .unwrap()
});

static QUARANTINED_STORES: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "solana_merkle_indexer_quarantined_stores",
        "Recorded stores that disagree with the chain, as the last store check left them",
        &["host_chain_id"]
    )
    .unwrap()
});

static STORE_CHECK_COMPLETED: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "solana_merkle_indexer_store_check_completed_timestamp_seconds",
        "Unix time the last store check over every recorded store completed",
        &["host_chain_id"]
    )
    .unwrap()
});

static STORE_CHECK_FAILURES: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "solana_merkle_indexer_store_check_failures_total",
        "Store checks stopped by an RPC or database error, retried at the next interval",
        &["host_chain_id"]
    )
    .unwrap()
});

/// How a recorded store compares with its account on chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StoreCheck {
    /// The record's peaks at the chain's leaf count are the chain's.
    Match,
    /// The account is not a valid store, or the record's peaks at the chain's leaf count differ.
    Mismatch,
    /// The record holds fewer leaves than the chain.
    Behind,
    /// No account at the address: the store was closed.
    Absent,
}

impl StoreCheck {
    fn label(self) -> &'static str {
        match self {
            Self::Match => "match",
            Self::Mismatch => "mismatch",
            Self::Behind => "behind",
            Self::Absent => "absent",
        }
    }
}

/// The quarantined stores as the table holds them, with their gauge. Each change is written
/// to the table first and then shown, so the gauge follows the table even when a check stops
/// partway.
struct Quarantine {
    stores: HashSet<[u8; 32]>,
    gauge: IntGauge,
}

impl Quarantine {
    async fn load(
        pool: &sqlx::PgPool,
        host_chain_id: &str,
    ) -> Result<Self, sqlx::Error> {
        let quarantine = Self {
            stores: load_quarantined_stores(pool).await?,
            gauge: QUARANTINED_STORES.with_label_values(&[host_chain_id]),
        };
        quarantine.show();
        Ok(quarantine)
    }

    async fn add(
        &mut self,
        pool: &sqlx::PgPool,
        store: [u8; 32],
    ) -> Result<(), sqlx::Error> {
        if !self.stores.contains(&store) {
            quarantine_store(pool, store).await?;
            self.stores.insert(store);
            self.show();
        }
        Ok(())
    }

    async fn lift(
        &mut self,
        pool: &sqlx::PgPool,
        store: [u8; 32],
    ) -> Result<(), sqlx::Error> {
        if self.stores.contains(&store) {
            lift_quarantine(pool, store).await?;
            self.stores.remove(&store);
            self.show();
        }
        Ok(())
    }

    fn show(&self) {
        self.gauge.set(gauge_value(self.stores.len() as u64));
    }
}

/// The on-chain accounts of a page of stores, `None` where nothing is stored.
pub trait StoreAccounts {
    fn accounts(
        &self,
        stores: &[[u8; 32]],
    ) -> impl Future<Output = anyhow::Result<Vec<Option<Account>>>> + Send;
}

/// At the client's commitment, confirmed for the indexer.
impl StoreAccounts for RpcClient {
    async fn accounts(
        &self,
        stores: &[[u8; 32]],
    ) -> anyhow::Result<Vec<Option<Account>>> {
        let keys: Vec<Pubkey> =
            stores.iter().copied().map(Pubkey::new_from_array).collect();
        Ok(self.get_multiple_accounts(&keys).await?)
    }
}

/// Checks every recorded store, then every `interval`, until `cancel` fires. A check that
/// fails is counted and retried at the next interval.
pub async fn run_store_checks(
    pool: sqlx::PgPool,
    accounts: impl StoreAccounts,
    host_program: Pubkey,
    host_chain_id: u64,
    interval: Duration,
    cancel: CancellationToken,
) {
    let chain = host_chain_id.to_string();
    // The quarantine left by an earlier run pages before this run's first check completes.
    if let Err(err) = Quarantine::load(&pool, &chain).await {
        warn!(error = %err, "quarantined stores not read");
    }
    loop {
        if let Err(err) =
            check_stores(&pool, &accounts, &host_program, &chain).await
        {
            warn!(error = %err, "store check failed");
            STORE_CHECK_FAILURES.with_label_values(&[&chain]).inc();
        }
        tokio::select! {
            () = cancel.cancelled() => return,
            () = tokio::time::sleep(interval) => {}
        }
    }
}

/// One check over every recorded store, page by page in address order. A store the record is
/// behind on is compared once more after the rest, so a store the indexer trails by a block or
/// two is compared too.
pub async fn check_stores(
    pool: &sqlx::PgPool,
    accounts: &impl StoreAccounts,
    host_program: &Pubkey,
    host_chain_id: &str,
) -> anyhow::Result<()> {
    let mut quarantine = Quarantine::load(pool, host_chain_id).await?;
    let mut behind = Vec::new();
    let mut results = Vec::new();
    let mut after = None;
    loop {
        let stores = load_store_page(pool, after, STORE_CHECK_PAGE).await?;
        let Some(&last) = stores.last() else {
            break;
        };
        let fetched = accounts.accounts(&stores).await?;
        anyhow::ensure!(
            fetched.len() == stores.len(),
            "asked for {} accounts, got {}",
            stores.len(),
            fetched.len()
        );
        for (store, account) in stores.into_iter().zip(fetched) {
            match check_store(
                pool,
                host_program,
                store,
                account.as_ref(),
                &mut quarantine,
            )
            .await?
            {
                StoreCheck::Behind => behind.push((store, account)),
                result => results.push(result),
            }
        }
        after = Some(last);
    }
    for (store, account) in behind {
        results.push(
            check_store(
                pool,
                host_program,
                store,
                account.as_ref(),
                &mut quarantine,
            )
            .await?,
        );
    }
    for result in &results {
        STORE_CHECKS
            .with_label_values(&[host_chain_id, result.label()])
            .inc();
    }
    STORE_CHECK_COMPLETED
        .with_label_values(&[host_chain_id])
        .set(gauge_value(unix_now_secs()));
    let mismatches = results
        .iter()
        .filter(|&&result| result == StoreCheck::Mismatch)
        .count();
    info!(
        mismatches,
        quarantined = quarantine.stores.len(),
        "store check completed"
    );
    Ok(())
}

/// Compares one recorded store with its account, as the KMS connector judges a store
/// (`validate_store`), and moves it in or out of `quarantine`. The record's leaf count is read
/// after the account, so a store the indexer wrote meanwhile is compared rather than behind.
async fn check_store(
    pool: &sqlx::PgPool,
    host_program: &Pubkey,
    store: [u8; 32],
    account: Option<&Account>,
    quarantine: &mut Quarantine,
) -> anyhow::Result<StoreCheck> {
    let store_address = |chain: &zama_solana_acl::EncryptedStore| {
        let bump = [chain.bump];
        let mut seeds: Vec<&[u8]> = chain.seeds().to_vec();
        seeds.push(&bump);
        Pubkey::create_program_address(&seeds, host_program)
            .ok()
            .map(|address| address.to_bytes())
    };
    let view = account.map(|account| AccountView {
        owner: account.owner.as_array(),
        data: &account.data,
    });
    let encrypted_store = bs58::encode(store).into_string();
    let chain = match validate_store(
        host_program.as_array(),
        &store,
        view,
        store_address,
    ) {
        Ok(chain) => chain,
        Err(StoreRejection::Absent) => {
            // A closed store has nothing left to serve. An RPC node that answers `null` for a
            // store it has not seen lifts the quarantine until the next check.
            quarantine.lift(pool, store).await?;
            return Ok(StoreCheck::Absent);
        }
        Err(rejection) => {
            error!(
                encrypted_store,
                ?rejection,
                "encrypted store account is not a valid store"
            );
            quarantine.add(pool, store).await?;
            return Ok(StoreCheck::Mismatch);
        }
    };
    let record_leaf_count = load_served_store(pool, store)
        .await?
        .map_or(0, |served| served.cursor.leaf_count);
    if record_leaf_count < chain.leaf_count {
        return Ok(StoreCheck::Behind);
    }
    let recorded = load_peaks(pool, store, chain.leaf_count).await?;
    if recorded.as_ref() == Some(&chain.peaks) {
        quarantine.lift(pool, store).await?;
        return Ok(StoreCheck::Match);
    }
    error!(
        encrypted_store,
        record_leaf_count,
        chain_leaf_count = chain.leaf_count,
        "encrypted store's recorded peaks differ from the chain's"
    );
    quarantine.add(pool, store).await?;
    Ok(StoreCheck::Mismatch)
}
