-- HA fencing columns. `owner_epoch` is the dispatcher generation that owns a row for status
-- writes: the lock holder's claim stamps it, and every send-decision write in
-- store::sql::repositories applies only when the row's value is NULL or <= the writer's
-- epoch. Nullable with no default: NULL means "never claimed under the fence", which is true
-- of every pre-migration row and of rows written by images that predate the column, and the
-- claim treats NULL as open to any epoch. `gateway_chain_cursor.owner_epoch` carries the
-- same type and rationale; ChainCursorRepository::advance fences on it alongside its
-- monotonic block-number check.
--
-- `attempts` counts sweep re-dispatches of a row: the claim increments it, and a row
-- reaching max_attempts is failed out instead of claimed again. NOT NULL DEFAULT 0 keeps
-- pre-migration writers correct. The cursor gets no `attempts`: it is never re-dispatched.
--
-- All columns are additive and unconstrained; pre-migration INSERTs/UPDATEs keep working.
-- ADD COLUMN with a constant default is metadata-only on Postgres 11+ (deployed: 17).
ALTER TABLE user_decrypt_req
    ADD COLUMN owner_epoch BIGINT,
    ADD COLUMN attempts INTEGER NOT NULL DEFAULT 0;

ALTER TABLE public_decrypt_req
    ADD COLUMN owner_epoch BIGINT,
    ADD COLUMN attempts INTEGER NOT NULL DEFAULT 0;

ALTER TABLE input_proof_req
    ADD COLUMN owner_epoch BIGINT,
    ADD COLUMN attempts INTEGER NOT NULL DEFAULT 0;

ALTER TABLE gateway_chain_cursor
    ADD COLUMN owner_epoch BIGINT;

COMMENT ON COLUMN user_decrypt_req.owner_epoch IS
    'Dispatcher generation that currently owns this row for status writes. NULL means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for every row '
    'predating this column; a NULL row may be claimed by any epoch. Stamped by the lock '
    'holder''s claim, checked by every send-decision write in store::sql::repositories.';
COMMENT ON COLUMN user_decrypt_req.attempts IS
    'Number of times the sweep has re-dispatched this row. Incremented by the claim and '
    'bounded by the sweep''s max_attempts.';

COMMENT ON COLUMN public_decrypt_req.owner_epoch IS
    'Dispatcher generation that currently owns this row for status writes. NULL means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for every row '
    'predating this column; a NULL row may be claimed by any epoch. Stamped by the lock '
    'holder''s claim, checked by every send-decision write in store::sql::repositories.';
COMMENT ON COLUMN public_decrypt_req.attempts IS
    'Number of times the sweep has re-dispatched this row. Incremented by the claim and '
    'bounded by the sweep''s max_attempts.';

COMMENT ON COLUMN input_proof_req.owner_epoch IS
    'Dispatcher generation that currently owns this row for status writes. NULL means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for every row '
    'predating this column; a NULL row may be claimed by any epoch. Stamped by the lock '
    'holder''s claim, checked by every send-decision write in store::sql::repositories.';
COMMENT ON COLUMN input_proof_req.attempts IS
    'Number of times the sweep has re-dispatched this row. Incremented by the claim and '
    'bounded by the sweep''s max_attempts.';

COMMENT ON COLUMN gateway_chain_cursor.owner_epoch IS
    'Dispatcher generation that currently owns the cursor for position writes. NULL means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for the row '
    'predating this column; a NULL row may be claimed by any epoch. Stamped by the lock '
    'holder''s claim and checked by ChainCursorRepository::advance, which applies it '
    'alongside the monotonic block-number check.';
