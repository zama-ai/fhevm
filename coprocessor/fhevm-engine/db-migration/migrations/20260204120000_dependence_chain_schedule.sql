CREATE TABLE IF NOT EXISTS dependence_chain_schedule (
    dependence_chain_id  bytea PRIMARY KEY,
    last_scheduled_at     TIMESTAMP NOT NULL
);
