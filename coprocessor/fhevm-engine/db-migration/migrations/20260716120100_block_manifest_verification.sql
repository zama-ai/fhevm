-- Durable work item for verifying one exact local manifest revision. Registry
-- fields remain NULL until a complete GatewayConfig snapshot can be pinned.
CREATE TABLE IF NOT EXISTS block_manifest_verification_task
(
    id BIGSERIAL PRIMARY KEY,
    generation TEXT NOT NULL DEFAULT 'legacy' CHECK (generation <> '' AND LENGTH(generation) <= 256),
    local_manifest_id BIGINT NOT NULL,

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

    UNIQUE (local_manifest_id, generation),
    UNIQUE (id, generation),
    FOREIGN KEY (local_manifest_id, generation)
        REFERENCES block_manifest(id, generation),
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
    generation TEXT NOT NULL DEFAULT 'legacy' CHECK (generation <> '' AND LENGTH(generation) <= 256),
    task_id BIGINT NOT NULL,
    publisher BYTEA NOT NULL CHECK (OCTET_LENGTH(publisher) = 20),
    s3_bucket_url TEXT NOT NULL CHECK (LENGTH(s3_bucket_url) > 0),
    completed_attempt INTEGER NOT NULL DEFAULT 0 CHECK (completed_attempt >= 0),
    rejected_object_keys TEXT[] NOT NULL DEFAULT '{}',
    latest_revision BIGINT NULL CHECK (latest_revision IS NULL OR latest_revision >= 0),
    last_attempt_at TIMESTAMPTZ NULL,
    last_error TEXT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (generation, task_id, publisher),
    FOREIGN KEY (task_id, generation)
        REFERENCES block_manifest_verification_task(id, generation)
);

-- Immutable audit record for every completed verification attempt. The task
-- row keeps only the latest state; these rows retain the exact decision and
-- whether handle-level localization could be completed.
CREATE TABLE IF NOT EXISTS block_manifest_verification_attempt
(
    generation TEXT NOT NULL DEFAULT 'legacy' CHECK (generation <> '' AND LENGTH(generation) <= 256),
    task_id BIGINT NOT NULL,
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
    -- Only complete localization with quorum for every local scope is reusable.
    localization_cacheable BOOLEAN NOT NULL DEFAULT FALSE,
    CHECK (NOT localization_cacheable OR (localization_complete AND outcome = 'drift')),
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (generation, task_id, attempt),
    FOREIGN KEY (task_id, generation)
        REFERENCES block_manifest_verification_task(id, generation),
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
    generation TEXT NOT NULL DEFAULT 'legacy' CHECK (generation <> '' AND LENGTH(generation) <= 256),
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

    PRIMARY KEY (generation, task_id, attempt, drift_index),
    FOREIGN KEY (generation, task_id, attempt)
        REFERENCES block_manifest_verification_attempt(generation, task_id, attempt),
    CHECK ((range_kind = 'detailed') = (scale IS NULL))
);

-- Exact historical-comparison lookup; evidence is reused across tasks only when
-- range, lineage, local publisher, context, groups, and quorum policy match.
CREATE INDEX block_manifest_localization_cache_lookup
    ON block_manifest_verification_attempt_drift
        (generation, end_block_hash, first_block_number, last_block_number, local_digest)
    WHERE range_kind = 'historical';

-- Mutable local view of handle-level drift against one observed peer result.
-- A non-NULL target_ct64_digest identifies quorum-backed ciphertext material;
-- NULL means no computed ct64 quorum target has been established.
-- Immutable manifests and verification tasks remain the evidence; this table makes the unresolved/resolved inventory cheap to query.
-- Gateway key identity is retained only as optional local provenance and does
-- not participate in the quorum descriptor.
CREATE TABLE IF NOT EXISTS drifted_handle
(
    id BIGSERIAL PRIMARY KEY,
    generation TEXT NOT NULL DEFAULT 'legacy'
        CHECK (generation <> '' AND LENGTH(generation) <= 256),
    version SMALLINT NOT NULL CHECK (version > 0),
    coprocessor_context_id BYTEA NOT NULL
        CHECK (OCTET_LENGTH(coprocessor_context_id) = 32),
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    block_number BIGINT NOT NULL CHECK (block_number >= 0),
    block_hash BYTEA NOT NULL CHECK (OCTET_LENGTH(block_hash) = 32),
    handle BYTEA NOT NULL CHECK (OCTET_LENGTH(handle) = 32),

    -- Origin is independent of both the reason and quorum availability.
    detection_kind TEXT NOT NULL DEFAULT 'verified'
        CHECK (detection_kind IN ('verified', 'inferred')),
    reason TEXT NOT NULL
        CHECK (reason IN ('ct64_mismatch', 'ct128_mismatch', 'missing_here',
            'unknown_on_peer', 'error_here', 'error_on_peer', 'uncomputed_here',
            'uncomputed_on_peer', 'metadata_mismatch')),
    can_be_healed BOOLEAN GENERATED ALWAYS AS
        (reason IN ('ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here')
            AND target_ct64_digest IS NOT NULL
            AND healed_at IS NULL) STORED,
    demand_count BIGINT NOT NULL DEFAULT 0 CHECK (demand_count >= 0),
    -- Evidence contains the pinned registry/quorum and authenticated statements.
    -- Sources contain publisher identities and their download locations.
    target_evidence JSONB NULL CHECK (jsonb_typeof(target_evidence) = 'object'),
    peer_sources JSONB NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(peer_sources) = 'array'),
    next_retry_at TIMESTAMPTZ NULL,
    claimed_by TEXT NULL CHECK (claimed_by <> ''),
    claim_expires_at TIMESTAMPTZ NULL,

    -- Observation status is separate from successful local installation.
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
    target_ct64_digest BYTEA NULL
        CHECK (target_ct64_digest IS NULL OR OCTET_LENGTH(target_ct64_digest) = 32),
    observed_ct64_digest BYTEA NULL
        CHECK (observed_ct64_digest IS NULL OR OCTET_LENGTH(observed_ct64_digest) = 32),
    local_ct128_digest BYTEA NULL
        CHECK (local_ct128_digest IS NULL OR OCTET_LENGTH(local_ct128_digest) = 32),
    observed_ct128_digest BYTEA NULL
        CHECK (observed_ct128_digest IS NULL OR OCTET_LENGTH(observed_ct128_digest) = 32),
    local_ct128_format SMALLINT NULL,
    observed_ct128_format SMALLINT NULL,
    observed_commitment_digest BYTEA NULL
        CHECK (OCTET_LENGTH(observed_commitment_digest) = 32),
    last_observed_task_id BIGINT NULL,
    resolved_task_id BIGINT NULL,

    detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    healed_at TIMESTAMPTZ NULL,

    UNIQUE (
        generation,
        version,
        coprocessor_context_id,
        host_chain_id,
        block_hash,
        handle,
        observed_commitment_digest
    ),
    FOREIGN KEY (last_observed_task_id, generation)
        REFERENCES block_manifest_verification_task(id, generation),
    FOREIGN KEY (resolved_task_id, generation)
        REFERENCES block_manifest_verification_task(id, generation),
    CHECK (local_present OR local_gateway_key_id IS NULL),
    CHECK (
        detection_kind = 'inferred' OR local_present = (
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
    CHECK ((status = 'unresolved' AND resolved_task_id IS NULL)
        OR (status = 'resolved' AND resolved_task_id IS NOT NULL)),
    CHECK ((claimed_by IS NULL) = (claim_expires_at IS NULL)),
    CHECK (claimed_by IS NULL OR
        (reason IN ('ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here') AND healed_at IS NULL)),
    CHECK (healed_at IS NULL OR
        (reason IN ('ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here')
            AND target_ct64_digest IS NOT NULL AND claimed_by IS NULL)),
    CHECK (target_evidence IS NULL OR target_ct64_digest IS NOT NULL),
    CHECK (detection_kind <> 'inferred' OR (local_present AND reason = 'ct64_mismatch')),
    CHECK (detection_kind = 'inferred' OR
        (observed_commitment_digest IS NOT NULL AND last_observed_task_id IS NOT NULL))
);

-- Inferred outputs have no observed commitment to deduplicate on.
CREATE UNIQUE INDEX idx_drifted_handle_inferred_identity
ON drifted_handle (generation, version, coprocessor_context_id, host_chain_id, block_hash, handle)
WHERE detection_kind = 'inferred';

CREATE INDEX idx_drifted_handle_healing_priority
ON drifted_handle (demand_count DESC, detected_at, block_number)
WHERE reason IN ('ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here')
    AND healed_at IS NULL;

-- Shared predicate for scheduling and result acceptance, independent of quorum.
-- Keep handle before block hash so dependency lookups need no height scan.
CREATE INDEX idx_drifted_handle_forbidden_dependency
ON drifted_handle (generation, coprocessor_context_id, host_chain_id, handle, block_hash)
WHERE reason = 'ct64_mismatch' AND healed_at IS NULL;

-- Select all revisions for one publication identity across publishers, ordered
-- exactly as load_tip_eligible_manifest consumes them.
CREATE INDEX IF NOT EXISTS idx_block_manifest_tip
ON block_manifest (
    generation,
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
ON block_manifest (generation, publisher, manifest_digest);

CREATE INDEX IF NOT EXISTS idx_drifted_handle_unresolved
ON drifted_handle (
    generation,
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
ON block_manifest_verification_task (generation, next_attempt_at, id)
WHERE state = 'pending' AND required_quorum IS NULL;

-- Claimed rows stay here only until expiry, while pending rows are ready for a
-- worker claim. This matches the SKIP LOCKED claim selection exactly.
CREATE INDEX IF NOT EXISTS idx_block_manifest_verification_task_claimable_due
ON block_manifest_verification_task (generation, next_attempt_at, id)
WHERE required_quorum IS NOT NULL AND state IN ('pending', 'claimed');
