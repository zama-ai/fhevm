-- A dedicated sequence that mints `owner_epoch` for the dispatcher lock (step 6's sweep is
-- the first writer; see `orchestrator::dispatcher_lock`).
--
-- Why a sequence and not a pod name, hostname, wall clock, or backend pid: `owner_epoch` must
-- be monotonic across acquisitions AND survive restarts, so a successor's epoch always
-- compares greater than any predecessor's. Postgres is the only state shared by every pod, a
-- pod/hostname is not orderable, a wall clock can go backwards or tie across pods, and a
-- backend pid is reused by Postgres itself once a session closes. `nextval()` is atomic
-- across concurrent callers and, being unlogged only for the counter's cache (not for having
-- passed a value out), never repeats a value once handed out.
--
-- Minted once per confirmed lock acquisition (`DispatcherLock`), not once per claimed row -
-- every row this pod claims while holding the lock shares one epoch value.
CREATE SEQUENCE dispatcher_epoch_seq AS BIGINT START WITH 1 INCREMENT BY 1 NO CYCLE;

COMMENT ON SEQUENCE dispatcher_epoch_seq IS
    'Mints owner_epoch at dispatcher-lock acquisition (orchestrator::dispatcher_lock). One '
    'value per acquisition, stamped on every row the sweep claims for that holder generation. '
    'Monotonic across restarts because it lives in Postgres, not process memory.';
