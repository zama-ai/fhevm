-- Tracks the local block commitment and its manifest publication progress.
-- Competing block hashes at the same height intentionally coexist.
CREATE TABLE IF NOT EXISTS block_manifest_state
(
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    block_number BIGINT NOT NULL CHECK (block_number >= 0),
    block_hash BYTEA NOT NULL CHECK (OCTET_LENGTH(block_hash) = 32),
    parent_block_hash BYTEA NOT NULL CHECK (OCTET_LENGTH(parent_block_hash) = 32),

    block_content_digest BYTEA NULL
        CHECK (block_content_digest IS NULL OR OCTET_LENGTH(block_content_digest) = 32),
    block_handle_count BIGINT NULL CHECK (block_handle_count IS NULL OR block_handle_count >= 0),
    publication_cadence BIGINT NOT NULL CHECK (publication_cadence > 0),
    manifest_required BOOLEAN GENERATED ALWAYS AS (
        MOD(block_number, publication_cadence) = 0
    ) STORED,
    -- Scheduler bookkeeping only. While the parent is live, discover its
    -- direct non-orphaned children; close this scan once it is finalized or orphaned.
    child_block_discovery_closed BOOLEAN NOT NULL DEFAULT FALSE,

    -- First block and digest of the range directly represented by this manifest.
    manifest_range_start BIGINT NULL
        CHECK (manifest_range_start IS NULL OR manifest_range_start >= 0),
    manifest_range_digest BYTEA NULL
        CHECK (manifest_range_digest IS NULL OR OCTET_LENGTH(manifest_range_digest) = 32),

    manifest_revision BIGINT NOT NULL DEFAULT 0 CHECK (manifest_revision >= 0),
    manifest_publisher BYTEA NULL
        CHECK (manifest_publisher IS NULL OR OCTET_LENGTH(manifest_publisher) = 20),
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
            AND manifest_publisher IS NOT NULL
            AND manifest_published_at IS NOT NULL
            AND manifest_range_start IS NOT NULL
            AND manifest_range_digest IS NOT NULL
        ) OR (
            NOT manifest_published
            AND manifest_digest IS NULL
            AND manifest_published_at IS NULL
            AND manifest_range_start IS NULL
            AND manifest_range_digest IS NULL
        )
    )
);

-- Immutable, reusable roots for aligned dyadic ranges. A repair that changes
-- a root appends a distinct digest; equal roots are reused across manifests.
CREATE TABLE IF NOT EXISTS block_range_commitment
(
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    range_start BIGINT NOT NULL CHECK (range_start >= 0),
    range_end BIGINT NOT NULL CHECK (range_end >= range_start),
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

-- Exact signed manifests created locally or downloaded from peers. All observed
-- immutable revisions coexist; publisher identity prevents equal peer keys from
-- colliding locally.
CREATE TABLE IF NOT EXISTS block_manifest
(
    id BIGSERIAL PRIMARY KEY,
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
    -- A local publication promotes an already-downloaded copy to `local`.
    manifest_source TEXT NOT NULL CHECK (manifest_source IN ('local', 'peer')),
    archived_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (
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

CREATE INDEX IF NOT EXISTS idx_block_manifest_state_parent
ON block_manifest_state (host_chain_id, parent_block_hash);

CREATE INDEX IF NOT EXISTS idx_block_manifest_state_pending_manifest
ON block_manifest_state (host_chain_id, block_number, block_hash)
WHERE block_content_digest IS NULL
   OR (manifest_required AND NOT manifest_published);

CREATE INDEX IF NOT EXISTS idx_block_manifest_state_open_discovery
ON block_manifest_state (host_chain_id, block_number, block_hash)
WHERE NOT child_block_discovery_closed;
