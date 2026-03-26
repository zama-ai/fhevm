-- CI validation migration: proves testcontainers applies new migrations correctly.
-- Remove this migration and its test before merging.

ALTER TABLE ciphertexts128 ADD COLUMN ci_test_column BOOLEAN DEFAULT NULL;

CREATE TABLE ci_test_table (
    id          SERIAL PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    value       BIGINT NOT NULL DEFAULT 0,
    metadata    JSONB,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ci_test_table_name ON ci_test_table (name);

INSERT INTO ci_test_table (name, value, metadata) VALUES
    ('alpha', 100, '{"env": "ci"}'),
    ('beta',  200, '{"env": "ci"}'),
    ('gamma', 300, NULL);
