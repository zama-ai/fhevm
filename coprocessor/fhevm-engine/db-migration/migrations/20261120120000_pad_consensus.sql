-- The consensus-detector compares each pad output that this coprocessor computed with the outputs
-- of the other coprocessors in S3, with the rules of the manifest comparison. Observation only: no
-- result changes an output.
CREATE TABLE IF NOT EXISTS pad_consensus (
    prf_instance INTEGER NOT NULL CHECK (prf_instance BETWEEN 0 AND 65535),
    label BYTEA NOT NULL CHECK (OCTET_LENGTH(label) = 32),
    pad_length INTEGER NOT NULL CHECK (pad_length BETWEEN 1 AND 2048),
    -- Our attestation: the two digests and our signer.
    ciphertext_digest BYTEA NOT NULL CHECK (OCTET_LENGTH(ciphertext_digest) = 32),
    ciphertext128_digest BYTEA NOT NULL CHECK (OCTET_LENGTH(ciphertext128_digest) = 32),
    signer BYTEA NOT NULL CHECK (OCTET_LENGTH(signer) = 20),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    next_check_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Set when the check ends. `outcome`: consensus, drift, unknown_but_equal or unknown.
    -- `local_result`: matches_quorum, differs_from_quorum or inconclusive.
    outcome TEXT NULL,
    local_result TEXT NULL,
    done_at TIMESTAMPTZ NULL,
    PRIMARY KEY (prf_instance, label, pad_length)
);

CREATE INDEX IF NOT EXISTS idx_pad_consensus_due ON pad_consensus (next_check_at)
    WHERE done_at IS NULL;

-- The answer of each other coprocessor: the digests of its attestation, or the reason that the
-- attestation is not valid. Only valid attestations count in the comparison.
CREATE TABLE IF NOT EXISTS pad_consensus_answer (
    prf_instance INTEGER NOT NULL,
    label BYTEA NOT NULL,
    pad_length INTEGER NOT NULL,
    peer_signer BYTEA NOT NULL CHECK (OCTET_LENGTH(peer_signer) = 20),
    ciphertext_digest BYTEA NULL CHECK (OCTET_LENGTH(ciphertext_digest) = 32),
    ciphertext128_digest BYTEA NULL CHECK (OCTET_LENGTH(ciphertext128_digest) = 32),
    reason TEXT NULL,
    -- The attestation header of the peer object.
    peer_attestation TEXT NOT NULL,
    answered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (prf_instance, label, pad_length, peer_signer),
    FOREIGN KEY (prf_instance, label, pad_length) REFERENCES pad_consensus ON DELETE CASCADE,
    CHECK ((ciphertext_digest IS NULL) = (ciphertext128_digest IS NULL)),
    CHECK ((ciphertext_digest IS NULL) = (reason IS NOT NULL))
);
