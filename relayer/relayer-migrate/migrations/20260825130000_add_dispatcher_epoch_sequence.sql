-- Mints `owner_epoch` for the dispatcher lock (see `orchestrator::dispatcher_lock`): one
-- value per confirmed acquisition, shared by every row that holder claims. Lives in
-- Postgres so it is monotonic across restarts and pods; `nextval()` is atomic and never
-- repeats a handed-out value.
CREATE SEQUENCE dispatcher_epoch_seq AS BIGINT START WITH 1 INCREMENT BY 1 NO CYCLE;

COMMENT ON SEQUENCE dispatcher_epoch_seq IS
    'Mints owner_epoch at dispatcher-lock acquisition (orchestrator::dispatcher_lock). One '
    'value per acquisition, stamped on every row the sweep claims for that holder generation. '
    'Monotonic across restarts because it lives in Postgres, not process memory.';
