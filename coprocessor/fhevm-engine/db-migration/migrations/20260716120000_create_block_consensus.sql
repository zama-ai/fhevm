-- Tracks the local block commitment and its manifest publication progress.
-- Competing block hashes at the same height intentionally coexist.
CREATE TABLE IF NOT EXISTS block_consensus
(
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    block_number BIGINT NOT NULL CHECK (block_number >= 0),
    block_hash BYTEA NOT NULL CHECK (OCTET_LENGTH(block_hash) = 32),
    parent_block_hash BYTEA NOT NULL CHECK (OCTET_LENGTH(parent_block_hash) = 32),

    block_content_digest BYTEA NULL
        CHECK (block_content_digest IS NULL OR OCTET_LENGTH(block_content_digest) = 32),
    descriptor_count BIGINT NULL CHECK (descriptor_count IS NULL OR descriptor_count >= 0),

    detailed_range_start BIGINT NULL
        CHECK (detailed_range_start IS NULL OR detailed_range_start >= 0),
    detailed_range_digest BYTEA NULL
        CHECK (detailed_range_digest IS NULL OR OCTET_LENGTH(detailed_range_digest) = 32),

    manifest_revision BIGINT NOT NULL DEFAULT 0 CHECK (manifest_revision >= 0),
    manifest_digest BYTEA NULL
        CHECK (manifest_digest IS NULL OR OCTET_LENGTH(manifest_digest) = 32),
    manifest_published BOOLEAN NOT NULL DEFAULT FALSE,
    manifest_published_at TIMESTAMPTZ NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (host_chain_id, block_hash),
    CHECK (
        (
            manifest_published
            AND manifest_digest IS NOT NULL
            AND manifest_published_at IS NOT NULL
            AND detailed_range_start IS NOT NULL
            AND detailed_range_digest IS NOT NULL
        ) OR (
            NOT manifest_published
            AND manifest_digest IS NULL
            AND manifest_published_at IS NULL
            AND detailed_range_start IS NULL
            AND detailed_range_digest IS NULL
        )
    )
);

-- Immutable, reusable roots for aligned dyadic ranges. A repair that changes
-- a root appends a distinct digest; equal roots are reused across manifests.
CREATE TABLE IF NOT EXISTS block_consensus_range
(
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    range_start BIGINT NOT NULL CHECK (range_start >= 0),
    range_end BIGINT NOT NULL CHECK (range_end >= range_start),
    scale INTEGER NOT NULL CHECK (scale >= 0),
    range_start_block_hash BYTEA NOT NULL
        CHECK (OCTET_LENGTH(range_start_block_hash) = 32),
    range_start_parent_block_hash BYTEA NOT NULL
        CHECK (OCTET_LENGTH(range_start_parent_block_hash) = 32),
    range_end_block_hash BYTEA NOT NULL
        CHECK (OCTET_LENGTH(range_end_block_hash) = 32),
    range_digest BYTEA NOT NULL CHECK (OCTET_LENGTH(range_digest) = 32),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (
        host_chain_id,
        range_start,
        range_end,
        range_end_block_hash,
        range_digest
    )
);

CREATE INDEX IF NOT EXISTS idx_block_consensus_range_end
ON block_consensus_range (host_chain_id, range_end, range_end_block_hash, scale);

-- Exact signed local and peer manifest bodies. All observed immutable revisions
-- coexist; publisher identity prevents equal peer keys from colliding locally.
CREATE TABLE IF NOT EXISTS block_consensus_manifest
(
    publisher BYTEA NOT NULL CHECK (OCTET_LENGTH(publisher) = 20),
    version SMALLINT NOT NULL CHECK (version > 0),
    coprocessor_context_id BYTEA NOT NULL
        CHECK (OCTET_LENGTH(coprocessor_context_id) = 32),
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    publication_block_number BIGINT NOT NULL
        CHECK (publication_block_number >= 0),
    publication_block_hash BYTEA NOT NULL
        CHECK (OCTET_LENGTH(publication_block_hash) = 32),
    revision BIGINT NOT NULL CHECK (revision >= 0),
    manifest_digest BYTEA NOT NULL CHECK (OCTET_LENGTH(manifest_digest) = 32),
    object_key TEXT NOT NULL CHECK (LENGTH(object_key) > 0),
    signed_manifest BYTEA NOT NULL CHECK (OCTET_LENGTH(signed_manifest) > 0),
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (
        publisher,
        version,
        coprocessor_context_id,
        host_chain_id,
        publication_block_number,
        publication_block_hash,
        revision
    ),

    UNIQUE (publisher, object_key)
);

CREATE INDEX IF NOT EXISTS idx_block_consensus_manifest_height
ON block_consensus_manifest (
    publisher,
    version,
    coprocessor_context_id,
    host_chain_id,
    publication_block_number DESC,
    publication_block_hash,
    revision DESC
);

-- Durable work item for verifying one exact local manifest revision. Registry
-- fields remain NULL until a complete GatewayConfig snapshot can be pinned.
CREATE TABLE IF NOT EXISTS block_consensus_verification_target
(
    id BIGSERIAL PRIMARY KEY,
    local_publisher BYTEA NOT NULL CHECK (OCTET_LENGTH(local_publisher) = 20),
    version SMALLINT NOT NULL CHECK (version > 0),
    coprocessor_context_id BYTEA NOT NULL
        CHECK (OCTET_LENGTH(coprocessor_context_id) = 32),
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    publication_block_number BIGINT NOT NULL
        CHECK (publication_block_number >= 0),
    publication_block_hash BYTEA NOT NULL
        CHECK (OCTET_LENGTH(publication_block_hash) = 32),
    revision BIGINT NOT NULL CHECK (revision >= 0),
    local_manifest_digest BYTEA NOT NULL
        CHECK (OCTET_LENGTH(local_manifest_digest) = 32),

    eligible_at TIMESTAMPTZ NOT NULL,
    next_attempt_at TIMESTAMPTZ NULL,
    retry_delay_micros BIGINT NOT NULL CHECK (retry_delay_micros >= 0),
    max_attempts INTEGER NOT NULL CHECK (max_attempts > 0),
    attempt_count INTEGER NOT NULL DEFAULT 0
        CHECK (attempt_count >= 0 AND attempt_count <= max_attempts),
    state TEXT NOT NULL DEFAULT 'waiting_registry'
        CHECK (state IN ('waiting_registry', 'pending', 'leased', 'complete', 'exhausted')),
    latest_outcome TEXT NOT NULL DEFAULT 'unknown'
        CHECK (latest_outcome IN (
            'unknown',
            'unknown_but_equal',
            'consensus',
            'drift',
            'partial_consensus'
        )),
    quorum_scope_count INTEGER NOT NULL DEFAULT 0 CHECK (quorum_scope_count >= 0),
    local_drift_scope_count INTEGER NOT NULL DEFAULT 0
        CHECK (local_drift_scope_count >= 0),

    lease_owner TEXT NULL,
    lease_expires_at TIMESTAMPTZ NULL,
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

    UNIQUE (
        local_publisher,
        version,
        coprocessor_context_id,
        host_chain_id,
        publication_block_number,
        publication_block_hash,
        revision
    ),
    CHECK (
        (state = 'leased') = (lease_owner IS NOT NULL AND lease_expires_at IS NOT NULL)
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

-- Per-peer progress for one pinned target. completed_attempt makes a lease
-- recovery skip peers already durably downloaded by the crashed worker while
-- allowing a later retry attempt to poll every peer for newer revisions.
CREATE TABLE IF NOT EXISTS block_consensus_peer_download
(
    target_id BIGINT NOT NULL
        REFERENCES block_consensus_verification_target(id) ON DELETE CASCADE,
    publisher BYTEA NOT NULL CHECK (OCTET_LENGTH(publisher) = 20),
    s3_bucket_url TEXT NOT NULL CHECK (LENGTH(s3_bucket_url) > 0),
    completed_attempt INTEGER NOT NULL DEFAULT 0 CHECK (completed_attempt >= 0),
    latest_revision BIGINT NULL CHECK (latest_revision IS NULL OR latest_revision >= 0),
    last_attempt_at TIMESTAMPTZ NULL,
    last_error TEXT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (target_id, publisher)
);

-- Mutable local view of handle-level drift against one observed peer result.
-- observed_has_quorum separates actionable remediation evidence from a
-- below-quorum divergence. Immutable manifests and verification targets remain
-- the evidence; this table makes the unresolved/resolved inventory cheap to query.
-- Gateway key identity is retained only as optional local provenance and does
-- not participate in the quorum descriptor.
CREATE TABLE IF NOT EXISTS block_consensus_drift_handle
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
    finding_kind TEXT NOT NULL
        CHECK (finding_kind IN (
            'missing_handle',
            'unexpected_handle',
            'descriptor_mismatch'
        )),
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
    keyset_mismatch BOOLEAN NOT NULL,
    ct64_digest_mismatch BOOLEAN NOT NULL,
    ct128_digest_mismatch BOOLEAN NOT NULL,
    ct128_format_mismatch BOOLEAN NOT NULL,

    observed_publisher BYTEA NOT NULL CHECK (OCTET_LENGTH(observed_publisher) = 20),
    observed_manifest_digest BYTEA NOT NULL
        CHECK (OCTET_LENGTH(observed_manifest_digest) = 32),
    observed_commitment_digest BYTEA NOT NULL
        CHECK (OCTET_LENGTH(observed_commitment_digest) = 32),
    observed_has_quorum BOOLEAN NOT NULL,
    first_detected_target_id BIGINT NOT NULL
        REFERENCES block_consensus_verification_target(id),
    last_observed_target_id BIGINT NOT NULL
        REFERENCES block_consensus_verification_target(id),
    resolved_target_id BIGINT NULL
        REFERENCES block_consensus_verification_target(id),
    first_detected_local_manifest_digest BYTEA NOT NULL
        CHECK (OCTET_LENGTH(first_detected_local_manifest_digest) = 32),
    last_observed_local_manifest_digest BYTEA NOT NULL
        CHECK (OCTET_LENGTH(last_observed_local_manifest_digest) = 32),
    resolved_local_manifest_digest BYTEA NULL
        CHECK (
            resolved_local_manifest_digest IS NULL
            OR OCTET_LENGTH(resolved_local_manifest_digest) = 32
        ),
    last_local_manifest_revision BIGINT NOT NULL
        CHECK (last_local_manifest_revision >= 0),
    resolved_local_manifest_revision BIGINT NULL
        CHECK (
            resolved_local_manifest_revision IS NULL
            OR resolved_local_manifest_revision >= 0
        ),

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
        (finding_kind = 'missing_handle' AND NOT local_present AND observed_present)
        OR
        (finding_kind = 'unexpected_handle' AND local_present AND NOT observed_present)
        OR
        (finding_kind = 'descriptor_mismatch' AND local_present AND observed_present)
    ),
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
        (finding_kind = 'descriptor_mismatch'
            AND (
                keyset_mismatch
                OR ct64_digest_mismatch
                OR ct128_digest_mismatch
                OR ct128_format_mismatch
            ))
        OR
        (finding_kind <> 'descriptor_mismatch'
            AND NOT keyset_mismatch
            AND NOT ct64_digest_mismatch
            AND NOT ct128_digest_mismatch
            AND NOT ct128_format_mismatch)
    ),
    CHECK (
        (status = 'unresolved'
            AND resolved_target_id IS NULL
            AND resolved_local_manifest_digest IS NULL
            AND resolved_local_manifest_revision IS NULL
            AND resolved_at IS NULL)
        OR
        (status = 'resolved'
            AND resolved_target_id IS NOT NULL
            AND resolved_local_manifest_digest IS NOT NULL
            AND resolved_local_manifest_revision IS NOT NULL
            AND resolved_at IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_block_consensus_drift_handle_unresolved
ON block_consensus_drift_handle (
    local_publisher,
    coprocessor_context_id,
    host_chain_id,
    block_number,
    block_hash,
    handle
)
WHERE status = 'unresolved';

CREATE INDEX IF NOT EXISTS idx_block_consensus_verification_due
ON block_consensus_verification_target (next_attempt_at, id)
WHERE state IN ('waiting_registry', 'pending', 'leased');

CREATE INDEX IF NOT EXISTS idx_block_consensus_parent
ON block_consensus (host_chain_id, parent_block_hash);

CREATE INDEX IF NOT EXISTS idx_block_consensus_height
ON block_consensus (host_chain_id, block_number);
