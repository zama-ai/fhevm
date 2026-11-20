CREATE INDEX idx_dependence_chain_processed_last_updated
    ON dependence_chain (last_updated_at, dependence_chain_id)
    WHERE status = 'processed';
