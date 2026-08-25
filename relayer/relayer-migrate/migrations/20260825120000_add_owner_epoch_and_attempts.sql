-- Columns on the three request tables plus the gateway chain cursor, laying the ground for
-- the HA dispatch lock built in later steps: a single Postgres advisory lock decides which
-- pod dispatches, and `owner_epoch` fences an ex-holder that lost the lock without knowing
-- it yet.
--
-- This migration adds the columns only. Nothing reads or writes them yet -- later steps
-- wire the lock, the fencing predicate, and the sweep. No behaviour changes here.
--
-- `owner_epoch` identifies the dispatcher generation that currently owns a row for status
-- writes. It is nullable with no default rather than defaulted to 0: NULL means "no
-- dispatcher has claimed this row under the epoch fence yet", which is exactly the state of
-- every row written before this migration by a relayer image that does not know the column
-- exists. The staleness predicate future steps add is expected to treat NULL as open to
-- claim by any epoch, so a pre-existing row is claimed by whichever pod first touches it
-- rather than being rejected as owned by a phantom epoch. Reserving 0 as an "unclaimed"
-- sentinel instead would work too, but would make every future epoch comparison carry a
-- magic number; NULL says "unclaimed" without one. `gateway_chain_cursor.owner_epoch` carries
-- the same type, nullability and rationale: the single cursor row predates this column exactly
-- as every pre-existing request row does, and a stalled ex-holder must not be able to fence
-- itself in by writing an epoch nothing else recognizes. Nothing reads or writes it yet either
-- -- `ChainCursorRepository::advance` stays monotonic-only until the step-8 ownership check
-- wires this column in.
--
-- `attempts` counts re-dispatches of a row, so the step-6 sweep can bound how many times it
-- re-drives one. It is NOT NULL DEFAULT 0: every row, old or new, starts at zero attempts,
-- which is true both for rows inserted before this migration and for rows a pre-migration
-- relayer image inserts afterwards without naming the column. The cursor has no `attempts`
-- column: it is not re-dispatched, so the sweep's retry count has nothing to bound there.
--
-- Backward compatibility: every column added here is additive, nullable or defaulted, not
-- referenced by any constraint, and nothing is dropped, renamed, or narrowed. An INSERT or
-- UPDATE written before this migration -- one that names none of these columns -- keeps
-- working unchanged: `owner_epoch` lands NULL, `attempts` lands 0.
--
-- Lock behaviour: `ADD COLUMN ... DEFAULT <constant>` has been a metadata-only operation
-- since Postgres 11 (no table rewrite, no per-row write of the default) as long as the
-- default is a constant, which 0 is here. The ACCESS EXCLUSIVE lock `ALTER TABLE` still
-- takes is real, but its duration is independent of table size -- it is not a scan or a
-- rewrite, just a catalog update. Deployed relayer Postgres is 17 (gitops
-- values-relayer-{dev,testnet}.yaml pin engineVersion "17.4"), well past that optimization.
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
    'predating this column; a NULL row may be claimed by any epoch. Set by the lock holder, '
    'checked by the stale-write guard added in a later migration step.';
COMMENT ON COLUMN user_decrypt_req.attempts IS
    'Number of times this row has been re-dispatched. Bounds the sweep''s retries.';

COMMENT ON COLUMN public_decrypt_req.owner_epoch IS
    'Dispatcher generation that currently owns this row for status writes. NULL means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for every row '
    'predating this column; a NULL row may be claimed by any epoch. Set by the lock holder, '
    'checked by the stale-write guard added in a later migration step.';
COMMENT ON COLUMN public_decrypt_req.attempts IS
    'Number of times this row has been re-dispatched. Bounds the sweep''s retries.';

COMMENT ON COLUMN input_proof_req.owner_epoch IS
    'Dispatcher generation that currently owns this row for status writes. NULL means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for every row '
    'predating this column; a NULL row may be claimed by any epoch. Set by the lock holder, '
    'checked by the stale-write guard added in a later migration step.';
COMMENT ON COLUMN input_proof_req.attempts IS
    'Number of times this row has been re-dispatched. Bounds the sweep''s retries.';

COMMENT ON COLUMN gateway_chain_cursor.owner_epoch IS
    'Dispatcher generation that currently owns the cursor for position writes. NULL means no '
    'dispatcher has claimed it under the epoch fence yet, which is the case for the row '
    'predating this column; a NULL row may be claimed by any epoch. Set by the lock holder, '
    'checked by the stale-write guard added in a later migration step. Not yet consulted by '
    'ChainCursorRepository::advance, which stays monotonic-only until that step lands.';
