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

CREATE INDEX IF NOT EXISTS idx_block_consensus_parent
ON block_consensus (host_chain_id, parent_block_hash);

CREATE INDEX IF NOT EXISTS idx_block_consensus_height
ON block_consensus (host_chain_id, block_number);
