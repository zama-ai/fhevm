-- HA fencing columns. `owner_epoch` is the dispatcher generation that owns a row for status
-- writes: the lock holder's claim stamps it, and every send-decision write in
-- store::sql::repositories applies only when the row's value is <= the writer's epoch.
-- `gateway_chain_cursor.owner_epoch` carries the same type and rationale;
-- ChainCursorRepository::advance fences on it alongside its monotonic block-number check.
--
-- 0 means "never claimed under the fence", which covers every pre-migration row and any row
-- written by an image predating the column. `dispatcher_epoch_seq` starts at 1, so 0 loses to
-- every epoch it can mint. NULL would say the same thing but does not compare - `epoch <=
-- NULL` is unknown, not false - which would put an `IS NULL` arm in all two dozen fencing
-- predicates. Rust keeps the distinction in the type system instead: `current_epoch()` stays
-- `Option<i64>` for gating, and `fencing_epoch()` flattens it only for a write.
--
-- `attempts` counts sweep re-dispatches of a row: the claim increments it, and a row
-- reaching max_attempts is failed out instead of claimed again. The cursor gets no
-- `attempts`: it is never re-dispatched.
--
-- ADD COLUMN with a constant default is metadata-only on Postgres 11+ (deployed: 17), so
-- this does not rewrite the populated request tables.
ALTER TABLE user_decrypt_req
    ADD COLUMN owner_epoch BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN attempts INTEGER NOT NULL DEFAULT 0;

ALTER TABLE public_decrypt_req
    ADD COLUMN owner_epoch BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN attempts INTEGER NOT NULL DEFAULT 0;

ALTER TABLE input_proof_req
    ADD COLUMN owner_epoch BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN attempts INTEGER NOT NULL DEFAULT 0;

ALTER TABLE gateway_chain_cursor
    ADD COLUMN owner_epoch BIGINT NOT NULL DEFAULT 0;

COMMENT ON COLUMN user_decrypt_req.owner_epoch IS
    'Dispatcher generation that currently owns this row for status writes. 0 means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for every row '
    'predating this column; 0 is below every epoch dispatcher_epoch_seq can mint, so such a '
    'row loses to any real epoch. Stamped by the lock holder''s claim, checked by every '
    'send-decision write in store::sql::repositories.';
COMMENT ON COLUMN user_decrypt_req.attempts IS
    'Number of times the sweep has re-dispatched this row. Incremented by the claim and '
    'bounded by the sweep''s max_attempts.';

COMMENT ON COLUMN public_decrypt_req.owner_epoch IS
    'Dispatcher generation that currently owns this row for status writes. 0 means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for every row '
    'predating this column; 0 is below every epoch dispatcher_epoch_seq can mint, so such a '
    'row loses to any real epoch. Stamped by the lock holder''s claim, checked by every '
    'send-decision write in store::sql::repositories.';
COMMENT ON COLUMN public_decrypt_req.attempts IS
    'Number of times the sweep has re-dispatched this row. Incremented by the claim and '
    'bounded by the sweep''s max_attempts.';

COMMENT ON COLUMN input_proof_req.owner_epoch IS
    'Dispatcher generation that currently owns this row for status writes. 0 means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for every row '
    'predating this column; 0 is below every epoch dispatcher_epoch_seq can mint, so such a '
    'row loses to any real epoch. Stamped by the lock holder''s claim, checked by every '
    'send-decision write in store::sql::repositories.';
COMMENT ON COLUMN input_proof_req.attempts IS
    'Number of times the sweep has re-dispatched this row. Incremented by the claim and '
    'bounded by the sweep''s max_attempts.';

COMMENT ON COLUMN gateway_chain_cursor.owner_epoch IS
    'Dispatcher generation that currently owns the cursor for position writes. 0 means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for the row '
    'predating this column; 0 is below every epoch dispatcher_epoch_seq can mint, so such a '
    'row loses to any real epoch. Stamped by the lock holder''s claim and checked by '
    'ChainCursorRepository::advance, which applies it alongside the monotonic block-number '
    'check.';
