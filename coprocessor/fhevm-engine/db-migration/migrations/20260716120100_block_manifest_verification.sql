-- Durable work item for verifying one exact local manifest revision. Registry
-- fields remain NULL until a complete GatewayConfig snapshot can be pinned.
CREATE TABLE IF NOT EXISTS block_manifest_verification_task
(
    id BIGSERIAL PRIMARY KEY,
    local_manifest_id BIGINT NOT NULL REFERENCES block_manifest(id),

    eligible_at TIMESTAMPTZ NOT NULL,
    next_attempt_at TIMESTAMPTZ NULL,
    retry_delay_micros BIGINT NOT NULL CHECK (retry_delay_micros >= 0),
    max_attempts INTEGER NOT NULL CHECK (max_attempts > 0),
    attempt_count INTEGER NOT NULL DEFAULT 0
        CHECK (attempt_count >= 0 AND attempt_count <= max_attempts),
    state TEXT NOT NULL DEFAULT 'pending'
        CHECK (state IN ('pending', 'claimed', 'consensus', 'retry_exhausted')),
    latest_outcome TEXT NOT NULL DEFAULT 'unknown'
        CHECK (latest_outcome IN (
            'unknown',
            'unknown_but_equal',
            'consensus',
            'drift',
            'partial_consensus'
        )),
    claim_owner TEXT NULL,
    claim_expires_at TIMESTAMPTZ NULL,
    last_attempt_at TIMESTAMPTZ NULL,
    last_error TEXT NULL,

    gateway_chain_id BIGINT NULL CHECK (gateway_chain_id IS NULL OR gateway_chain_id >= 0),
    gateway_config_address BYTEA NULL
        CHECK (gateway_config_address IS NULL OR OCTET_LENGTH(gateway_config_address) = 20),
    registry_block_number BIGINT NULL
        CHECK (registry_block_number IS NULL OR registry_block_number >= 0),
    registry_block_hash BYTEA NULL
        CHECK (registry_block_hash IS NULL OR OCTET_LENGTH(registry_block_hash) = 32),
    registered_coprocessor_count INTEGER NULL
        CHECK (registered_coprocessor_count IS NULL OR registered_coprocessor_count > 0),
    required_quorum INTEGER NULL CHECK (required_quorum IS NULL OR required_quorum > 0),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (local_manifest_id),
    CHECK (
        (state = 'claimed') = (claim_owner IS NOT NULL AND claim_expires_at IS NOT NULL)
    ),
    CHECK (
        (
            gateway_chain_id IS NULL
            AND gateway_config_address IS NULL
            AND registry_block_number IS NULL
            AND registry_block_hash IS NULL
            AND registered_coprocessor_count IS NULL
            AND required_quorum IS NULL
        ) OR (
            gateway_chain_id IS NOT NULL
            AND gateway_config_address IS NOT NULL
            AND registry_block_number IS NOT NULL
            AND registry_block_hash IS NOT NULL
            AND registered_coprocessor_count IS NOT NULL
            AND required_quorum IS NOT NULL
            AND required_quorum <= registered_coprocessor_count
        )
    )
);

-- Per-peer progress for one pinned task. completed_attempt makes a claim
-- recovery skip peers already durably downloaded by the crashed worker while
-- allowing a later retry attempt to poll every peer for newer revisions.
CREATE TABLE IF NOT EXISTS block_manifest_peer_download
(
    task_id BIGINT NOT NULL
        REFERENCES block_manifest_verification_task(id),
    publisher BYTEA NOT NULL CHECK (OCTET_LENGTH(publisher) = 20),
    s3_bucket_url TEXT NOT NULL CHECK (LENGTH(s3_bucket_url) > 0),
    completed_attempt INTEGER NOT NULL DEFAULT 0 CHECK (completed_attempt >= 0),
    latest_revision BIGINT NULL CHECK (latest_revision IS NULL OR latest_revision >= 0),
    last_attempt_at TIMESTAMPTZ NULL,
    last_error TEXT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (task_id, publisher)
);

-- Immutable audit record for every completed verification attempt. The task
-- row keeps only the latest state; these rows retain the exact decision and
-- whether handle-level localization could be completed.
CREATE TABLE IF NOT EXISTS block_manifest_verification_attempt
(
    task_id BIGINT NOT NULL
        REFERENCES block_manifest_verification_task(id),
    attempt INTEGER NOT NULL CHECK (attempt > 0),
    outcome TEXT NOT NULL CHECK (outcome IN (
        'unknown',
        'unknown_but_equal',
        'consensus',
        'drift',
        'partial_consensus'
    )),
    local_quorum_status TEXT NOT NULL CHECK (local_quorum_status IN (
        'matches_quorum',
        'differs_from_quorum',
        'inconclusive'
    )),
    drifted_block_count BIGINT NULL CHECK (drifted_block_count >= 0),
    drifted_handle_count BIGINT NULL CHECK (drifted_handle_count >= 0),
    localization_complete BOOLEAN NOT NULL,
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (task_id, attempt),
    CHECK (
        localization_complete
        OR (drifted_block_count IS NULL AND drifted_handle_count IS NULL)
    )
);

-- Audit details for divergent block-range comparisons. Consensus comparisons
-- are omitted because the immutable manifests already contain the matching
-- commitments. Each JSON group records one digest and the publishers that
-- reported it.
CREATE TABLE IF NOT EXISTS block_manifest_verification_attempt_drift
(
    task_id BIGINT NOT NULL,
    attempt INTEGER NOT NULL,
    drift_index INTEGER NOT NULL CHECK (drift_index >= 0),
    range_kind TEXT NOT NULL CHECK (range_kind IN ('detailed', 'historical')),
    first_block_number BIGINT NOT NULL CHECK (first_block_number >= 0),
    last_block_number BIGINT NOT NULL CHECK (last_block_number >= first_block_number),
    scale INTEGER NULL CHECK (scale IS NULL OR scale >= 0),
    end_block_hash BYTEA NOT NULL CHECK (OCTET_LENGTH(end_block_hash) = 32),
    local_digest BYTEA NULL
        CHECK (local_digest IS NULL OR OCTET_LENGTH(local_digest) = 32),
    quorum_digest BYTEA NULL
        CHECK (quorum_digest IS NULL OR OCTET_LENGTH(quorum_digest) = 32),
    publisher_groups JSONB NOT NULL CHECK (JSONB_TYPEOF(publisher_groups) = 'array'),

    PRIMARY KEY (task_id, attempt, drift_index),
    FOREIGN KEY (task_id, attempt)
        REFERENCES block_manifest_verification_attempt(task_id, attempt),
    CHECK ((range_kind = 'detailed') = (scale IS NULL))
);

-- Mutable local view of handle-level drift against one observed peer result.
-- observed_has_quorum separates actionable remediation evidence from a
-- below-quorum divergence. Immutable manifests and verification tasks remain
-- the evidence; this table makes the unresolved/resolved inventory cheap to query.
-- Gateway key identity is retained only as optional local provenance and does
-- not participate in the quorum descriptor.
CREATE TABLE IF NOT EXISTS drifted_handle
(
    id BIGSERIAL PRIMARY KEY,
    local_publisher BYTEA NOT NULL CHECK (OCTET_LENGTH(local_publisher) = 20),
    version SMALLINT NOT NULL CHECK (version > 0),
    coprocessor_context_id BYTEA NOT NULL
        CHECK (OCTET_LENGTH(coprocessor_context_id) = 32),
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    block_number BIGINT NOT NULL CHECK (block_number >= 0),
    block_hash BYTEA NOT NULL CHECK (OCTET_LENGTH(block_hash) = 32),
    handle BYTEA NOT NULL CHECK (OCTET_LENGTH(handle) = 32),

    status TEXT NOT NULL DEFAULT 'unresolved'
        CHECK (status IN ('unresolved', 'resolved')),
    local_present BOOLEAN NOT NULL,
    observed_present BOOLEAN NOT NULL,
    local_keyset_id BYTEA NULL
        CHECK (local_keyset_id IS NULL OR OCTET_LENGTH(local_keyset_id) = 32),
    observed_keyset_id BYTEA NULL
        CHECK (observed_keyset_id IS NULL OR OCTET_LENGTH(observed_keyset_id) = 32),
    local_gateway_key_id BYTEA NULL
        CHECK (local_gateway_key_id IS NULL OR OCTET_LENGTH(local_gateway_key_id) = 32),
    local_ct64_digest BYTEA NULL
        CHECK (local_ct64_digest IS NULL OR OCTET_LENGTH(local_ct64_digest) = 32),
    observed_ct64_digest BYTEA NULL
        CHECK (observed_ct64_digest IS NULL OR OCTET_LENGTH(observed_ct64_digest) = 32),
    local_ct128_digest BYTEA NULL
        CHECK (local_ct128_digest IS NULL OR OCTET_LENGTH(local_ct128_digest) = 32),
    observed_ct128_digest BYTEA NULL
        CHECK (observed_ct128_digest IS NULL OR OCTET_LENGTH(observed_ct128_digest) = 32),
    local_ct128_format SMALLINT NULL,
    observed_ct128_format SMALLINT NULL,
    observed_manifest_id BIGINT NOT NULL REFERENCES block_manifest(id),
    observed_commitment_digest BYTEA NOT NULL
        CHECK (OCTET_LENGTH(observed_commitment_digest) = 32),
    observed_has_quorum BOOLEAN NOT NULL,
    first_detected_task_id BIGINT NOT NULL
        REFERENCES block_manifest_verification_task(id),
    last_observed_task_id BIGINT NOT NULL
        REFERENCES block_manifest_verification_task(id),
    resolved_task_id BIGINT NULL
        REFERENCES block_manifest_verification_task(id),

    first_detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_observed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (
        local_publisher,
        version,
        coprocessor_context_id,
        host_chain_id,
        block_hash,
        handle,
        observed_commitment_digest
    ),
    CHECK (local_present OR observed_present),
    CHECK (local_present OR local_gateway_key_id IS NULL),
    CHECK (
        local_present = (
            local_keyset_id IS NOT NULL
            AND local_ct64_digest IS NOT NULL
            AND local_ct128_digest IS NOT NULL
            AND local_ct128_format IS NOT NULL
        )
    ),
    CHECK (
        observed_present = (
            observed_keyset_id IS NOT NULL
            AND observed_ct64_digest IS NOT NULL
            AND observed_ct128_digest IS NOT NULL
            AND observed_ct128_format IS NOT NULL
        )
    ),
    CHECK (
        (status = 'unresolved'
            AND resolved_task_id IS NULL
            AND resolved_at IS NULL)
        OR
        (status = 'resolved'
            AND resolved_task_id IS NOT NULL
            AND resolved_at IS NOT NULL)
    )
);

-- Select all revisions for one publication identity across publishers, ordered
-- exactly as load_tip_eligible_manifest consumes them.
CREATE INDEX IF NOT EXISTS idx_block_manifest_tip
ON block_manifest (
    version,
    coprocessor_context_id,
    host_chain_id,
    publication_block_number,
    publication_block_hash,
    revision,
    publisher
);

-- Resolves a localized peer descriptor to its immutable archived manifest.
CREATE INDEX IF NOT EXISTS idx_block_manifest_publisher_digest
ON block_manifest (publisher, manifest_digest);

CREATE INDEX IF NOT EXISTS idx_drifted_handle_unresolved
ON drifted_handle (
    local_publisher,
    coprocessor_context_id,
    host_chain_id,
    block_number,
    block_hash,
    handle
)
WHERE status = 'unresolved';

-- Unbound tasks are the only rows that need registry binding. Keeping them
-- separate prevents the binder from repeatedly rebinding already pinned work.
CREATE INDEX IF NOT EXISTS idx_block_manifest_verification_task_unbound_due
ON block_manifest_verification_task (next_attempt_at, id)
WHERE state = 'pending' AND required_quorum IS NULL;

-- Claimed rows stay here only until expiry, while pending rows are ready for a
-- worker claim. This matches the SKIP LOCKED claim selection exactly.
CREATE INDEX IF NOT EXISTS idx_block_manifest_verification_task_claimable_due
ON block_manifest_verification_task (next_attempt_at, id)
WHERE required_quorum IS NOT NULL AND state IN ('pending', 'claimed');
