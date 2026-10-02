/** Private SQL control for a real downloaded-key application transaction. */
export function keyApplicationGateSql(key: string): string {
  if (!/^[0-9a-f]{64}$/.test(key)) throw new Error('invalid migration key identity');
  return `CREATE FUNCTION public.consensus_test_key_application_gate() RETURNS trigger LANGUAGE plpgsql AS $$
    BEGIN
      IF NEW.key_id = decode('${key}','hex') AND NEW.compressed_xof_keyset IS NOT NULL
         AND OLD.compressed_xof_keyset IS DISTINCT FROM NEW.compressed_xof_keyset THEN
        PERFORM pg_advisory_xact_lock(721029,1);
      END IF;
      RETURN NEW;
    END $$;
    CREATE TRIGGER consensus_test_key_application_gate BEFORE UPDATE OF compressed_xof_keyset ON public.keys
      FOR EACH ROW EXECUTE FUNCTION public.consensus_test_key_application_gate();`;
}
export const KEY_GATE_WAITERS = `SELECT count(*)::int AS count FROM pg_locks WHERE locktype='advisory'
  AND database=(SELECT oid FROM pg_database WHERE datname=current_database())
  AND classid=721029 AND objid=1 AND objsubid=2 AND NOT granted`;
export const DROP_KEY_GATE = `DROP TRIGGER IF EXISTS consensus_test_key_application_gate ON public.keys;
  DROP FUNCTION IF EXISTS public.consensus_test_key_application_gate();`;
