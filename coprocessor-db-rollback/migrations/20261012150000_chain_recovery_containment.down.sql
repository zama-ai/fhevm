-- Down for 20261012150000_chain_recovery_containment.
-- Drops both tables. Recorded chain halts and replay windows are lost.

DROP INDEX IF EXISTS chain_recovery_halts_active_identity;
DROP TABLE IF EXISTS public.listener_replay_state;
DROP TABLE IF EXISTS public.chain_recovery_halts;
