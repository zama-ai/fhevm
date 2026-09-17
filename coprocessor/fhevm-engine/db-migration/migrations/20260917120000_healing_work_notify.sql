-- Wake healing on new/repairable inventory, not on demand-score samples.
-- `WHEN` cannot use TG_OP, so insert and update are separate triggers.
-- The 30s poll still re-reads the latest scores.
CREATE OR REPLACE FUNCTION notify_healing_work()
    RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('event_healing_work', '');
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER drifted_handle_healing_work_insert
    AFTER INSERT
    ON drifted_handle
    FOR EACH ROW
    WHEN (NEW.can_be_healed)
    EXECUTE FUNCTION notify_healing_work();

CREATE TRIGGER drifted_handle_healing_work_update
    AFTER UPDATE
    ON drifted_handle
    FOR EACH ROW
    WHEN (NEW.can_be_healed AND (
        NEW.healed_at IS DISTINCT FROM OLD.healed_at
        OR NEW.next_retry_at IS DISTINCT FROM OLD.next_retry_at
        OR NEW.target_ct64_digest IS DISTINCT FROM OLD.target_ct64_digest
        OR NEW.peer_sources IS DISTINCT FROM OLD.peer_sources
        OR NEW.reason IS DISTINCT FROM OLD.reason
    ))
    EXECUTE FUNCTION notify_healing_work();
