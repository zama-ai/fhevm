-- ── Catchup request bookkeeping ─────────────────────────────────────────────

-- Catchup and final-catchup share an identical lifecycle, so one table
-- discriminated by flow (as with filters.filter_type) rather than two tables
-- (as with blocks/final_blocks, which differ in their status dimension).
CREATE TYPE catchup_flow AS ENUM ('CATCHUP', 'FINAL_CATCHUP');

-- ACTIVE is the only non-terminal value; every other value names the reason the
-- request stopped being current, so no separate reason column is needed.
--   ACTIVE     → the request this consumer currently wants replayed
--   CANCELLED  → explicit cancel_catchup
--   SKIPPED    → block_start was above the chain head, so fanout produced
--                nothing; named after listener_catchup_skipped_above_head_total
--   COMPLETED  → every block this request fanned out has been published
--                (D16). Set by the coverage merge, never by a count.
--
-- There is deliberately no SUPERSEDED. The core never retires a request on a
-- consumer's behalf; the consumer cancels the old id itself, and CANCELLED
-- already records that.
--
-- All four values ship in this CREATE TYPE rather than arriving via ALTER TYPE
-- ADD VALUE (D18). Postgres refuses to use a newly added enum value in the
-- transaction that added it, and sqlx runs every migration in a transaction by
-- default (sqlx-core-0.8.6 migrate/migrator.rs:25, no_tx: false), so a later
-- migration could add the value but not reference it. Nothing is committed yet,
-- so the value belongs here instead.
--
-- Rows are never deleted. There is no retention window and no sweep, and that
-- is a correctness property rather than an oversight.
--
-- A row is the only record that a catchup_id was ever used. Both no-op paths
-- depend on it: admit's ON CONFLICT DO NOTHING recognises a duplicate request,
-- and the fetcher's guard discards a stale sub-range by reading the row and
-- finding it terminal. Absent row means "never seen", which means run. So
-- deleting a row re-arms the work it describes — a consumer re-sending an id
-- whose row was swept gets a fresh ACTIVE row and re-runs a catchup that had
-- already finished, or one an operator had explicitly cancelled. Deleting a
-- COMPLETED row has the same effect on the consumer's boot-time replay: it
-- re-runs the whole backfill.
--
-- The deletion horizon would therefore be the idempotency horizon, and no
-- finite value is defensible: a consumer's boot-time re-send is unbounded in
-- time. The growth this avoids is not worth it either — a row is ~200 bytes
-- and requests are operator-scale events, so a realistic chain accrues tens of
-- kilobytes a year. The hot index below is partial on status = 'ACTIVE', so
-- terminal rows are not in it at all; they cost a primary key entry and heap
-- space and nothing else. An append-only table is also the easy case for
-- autovacuum, whereas a sweep manufactures dead tuples that would not
-- otherwise exist.
--
-- If a real growth problem ever appears, prune the wide columns of a terminal
-- row rather than the row: the identity is what must survive.
CREATE TYPE catchup_status AS ENUM ('ACTIVE', 'CANCELLED', 'SKIPPED', 'COMPLETED');

-- The only table. Sub-ranges have no durable state; they carry catchup_id in
-- the payload and read this row.
CREATE TABLE IF NOT EXISTS catchup_requests (
    catchup_id    UUID           PRIMARY KEY,  -- minted by the consumer
    flow          catchup_flow   NOT NULL,
    chain_id      BIGINT         NOT NULL,
    consumer_id   TEXT           NOT NULL,
    -- Nullable: when cancel arrives before the orchestrator has fanned the
    -- request out, the cancel handler creates the row and does not know the
    -- range. A 0 placeholder would collide with a legal genesis replay, since
    -- CatchupPayload::validate() only requires block_start <= block_end. NULL
    -- cannot collide, because no legal request produces it. Such a row is
    -- written terminal and is never backfilled.
    block_start   BIGINT,
    block_end     BIGINT,
    status        catchup_status NOT NULL DEFAULT 'ACTIVE',
    -- Set once, after the last sub-range of this request has been published.
    -- NULL means either "not fanned out yet" or "the orchestrator died part-way
    -- through", and both are handled identically: fan out again. Non-NULL is
    -- what makes a duplicate request a true no-op instead of a repeated fanout
    -- -- which for a multi-year backfill is hundreds of thousands of redundant
    -- messages.
    fanned_out_at TIMESTAMPTZ,
    -- How many sub-ranges the fanout produced. Observability only: nothing
    -- reads it to make a decision. It is what tells an operator that a request
    -- they expected to be small became 315,000 messages.
    sub_ranges    INTEGER,
    -- The last block the fanout actually covered, after clamping to the chain
    -- head (evm_listener.rs computes it as min(block_end, chain_height)).
    -- Written by admit_request, which already runs after the split and so
    -- already knows it.
    --
    -- This is the completion target, and it MUST NOT be block_end. A request
    -- for 1..1_000_000 against a head of 500 fans out 1..500 and nothing else;
    -- measuring coverage against the *requested* end leaves 501..1_000_000
    -- permanently uncovered and the request permanently ACTIVE (D16).
    --
    -- NULL on a SKIPPED row: the fanout produced nothing, the row is already
    -- terminal, and nothing will ever merge against it.
    fanned_end    BIGINT,
    -- The set of blocks published so far, as a union of half-open ranges.
    --
    -- A multirange rather than a counter, because the merge has to be
    -- idempotent: the one redelivery path that always exists -- a
    -- connection-error XACK leaves the message in the PEL and the sweeper
    -- reclaims it after claim_min_idle -- completes a sub-range twice. Set
    -- union absorbs that; addition does not. D11 rejected completion detection
    -- on exactly this ground, and rejected it correctly for every *counting*
    -- mechanism. D16 revisits it because a coverage set is not a count.
    --
    -- Fragmentation is bounded by concurrency, not by block count: with
    -- prefetch = 5 and range_prefetch = 1 only a handful of sub-ranges are ever
    -- in flight, workers claim roughly in stream order, and adjacent ranges
    -- coalesce on union. The column holds tens of ranges, not tens of
    -- thousands, regardless of whether the request spans 100 blocks or 31.5M.
    --
    -- Requires Postgres 14+.
    covered       INT8MULTIRANGE NOT NULL DEFAULT '{}',
    terminal_at   TIMESTAMPTZ,   -- set exactly when status leaves ACTIVE
    created_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

-- Operator and API query: list the active catchups of one consumer, served by
-- the (chain_id, consumer_id) leftmost prefix. Partial, so it matches the
-- predicate exactly and stays small.
--
-- NOT UNIQUE, deliberately. "At most one active request per consumer per flow"
-- is the consumer's policy, not the core's. Enforcing it here would also turn a
-- legal ordering into an outage: the consumer's retire sequence is cancel(X)
-- then request(Y) on two different streams with no cross-stream ordering, so
-- request(Y) arriving first is ordinary. Against a unique index that raises a
-- unique violation, which classifies transient, and five of those in a row open
-- the circuit breaker for all catchup orchestration on the pod. Nothing depends
-- on uniqueness either: the fetcher's guard is a primary-key lookup.
CREATE INDEX idx_catchup_requests_active
    ON catchup_requests(chain_id, consumer_id, flow)
    WHERE status = 'ACTIVE';

-- Deliberately NOT created:
--  * anything keyed on sub-ranges — there is no sub-range table, and the
--    fetcher's guard is a primary-key lookup on catchup_id.
--  * anything on terminal_at — nothing queries rows by when they ended. The
--    column is history an operator reads, not a sweep predicate; nothing is
--    swept.
--  * anything on covered — it is only ever read through the row it belongs to,
--    by primary key.
--  * (consumer_id) alone — no query filters by consumer without a chain.
