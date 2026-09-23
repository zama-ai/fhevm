-- Recover parents closed by the old parent-finality-only discovery rule.
-- A pending successor is insufficient: another child may become canonical.
UPDATE block_manifest_state parent
   SET child_block_discovery_closed = FALSE,
       updated_at = NOW()
  FROM host_chain_blocks_valid host
 WHERE parent.child_block_discovery_closed
   AND host.chain_id = parent.host_chain_id
   AND host.block_hash = parent.block_hash
   AND host.block_status = 'finalized'
   AND NOT EXISTS (
       SELECT 1
         FROM host_chain_blocks_valid child
         JOIN block_manifest_state discovered
           ON discovered.consensus_epoch = parent.consensus_epoch
          AND discovered.host_chain_id = child.chain_id
          AND discovered.block_hash = child.block_hash
        WHERE child.chain_id = parent.host_chain_id
          AND child.parent_hash = parent.block_hash
          AND child.block_status = 'finalized'
   );

COMMENT ON COLUMN block_manifest_state.child_block_discovery_closed IS
    'Discovery is complete for an orphaned parent, or a finalized parent with a durably discovered finalized successor.';
