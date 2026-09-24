-- Demand score is per ciphertext handle, shared by every finding for that
-- handle. Keeping it off drifted_handle means TFHE EMA writes do not lock
-- rows healing is picking, and do not fire event_healing_work.
CREATE TABLE drifted_handle_demand (
    handle BYTEA PRIMARY KEY CHECK (OCTET_LENGTH(handle) = 32),
    tx_unlock_potential DOUBLE PRECISION NOT NULL DEFAULT 0
        CHECK (tx_unlock_potential >= 0)
);

INSERT INTO drifted_handle_demand (handle, tx_unlock_potential)
SELECT handle, MAX(tx_unlock_potential)
  FROM drifted_handle
 GROUP BY handle
HAVING MAX(tx_unlock_potential) > 0;

DROP INDEX idx_drifted_handle_healing_priority;

ALTER TABLE drifted_handle DROP COLUMN tx_unlock_potential;

CREATE INDEX idx_drifted_handle_demand_priority
    ON drifted_handle_demand (tx_unlock_potential DESC);
