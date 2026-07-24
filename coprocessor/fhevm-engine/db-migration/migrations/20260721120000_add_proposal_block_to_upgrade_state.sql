-- Block the CoprocessorUpgradeProposed event was seen in. proposalId is
-- caller-supplied and unordered, so this gives the upsert an ordering key to
-- reject an older proposal replayed after a cycle finished. Nullable: existing
-- rows stay permissive until the next upsert sets it.
ALTER TABLE upgrade_state
    ADD COLUMN IF NOT EXISTS proposal_block BIGINT NULL CHECK (proposal_block >= 0);
