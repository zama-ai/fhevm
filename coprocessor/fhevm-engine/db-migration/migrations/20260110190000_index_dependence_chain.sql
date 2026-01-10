CREATE INDEX IF NOT EXISTS idx_dependence_chain_last_updated_at
    ON dependence_chain (last_updated_at)
    WHERE status = 'updated'::text
      AND worker_id IS NULL;

