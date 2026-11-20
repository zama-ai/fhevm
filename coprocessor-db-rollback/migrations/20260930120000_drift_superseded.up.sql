-- A cutover replaces the previous epoch's in-window ciphertexts in `public`
-- with the new epoch's. An older finding on such a block describes bytes that
-- are gone: healing it would overwrite the successor's copy, and freezing on it
-- would block work that reads correct bytes.
--
-- A finding is superseded when another succeeded epoch has a window on its
-- chain that starts at or before the finding's block, and later than the start
-- of the finding's own epoch there. `legacy` has no window and starts before
-- every other epoch. Only `start_block` and `outcome` decide, the facts the
-- cutover merge itself relies on; `allocated_at` is audit metadata.
--
-- It is evaluated when a finding is read, so a finding committed just after
-- the cutover is covered too. Healing records it in `superseded_at`.
CREATE OR REPLACE FUNCTION public.drift_superseded(
    finding_epoch TEXT,
    finding_chain BIGINT,
    finding_block BIGINT
) RETURNS BOOLEAN
LANGUAGE sql STABLE AS $$
    SELECT EXISTS (
        SELECT 1
          FROM public.consensus_epoch_block_window successor
          JOIN public.consensus_epoch_history history
            ON history.consensus_epoch = successor.consensus_epoch
         WHERE successor.host_chain_id = finding_chain
           AND successor.consensus_epoch <> finding_epoch
           AND history.outcome = 'succeeded'
           AND successor.start_block <= finding_block
           AND successor.start_block > COALESCE(
                (SELECT own.start_block
                   FROM public.consensus_epoch_block_window own
                  WHERE own.consensus_epoch = finding_epoch
                    AND own.host_chain_id = finding_chain),
                -1)
    )
$$;

-- Set by healing on a superseded finding: it leaves every queue and stays for
-- audit. It is not `healed_at`: nothing was installed.
ALTER TABLE drifted_handle
    ADD COLUMN superseded_at TIMESTAMPTZ NULL;
