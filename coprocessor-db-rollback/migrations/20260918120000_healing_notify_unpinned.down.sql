-- Down for 20260918120000_healing_notify_unpinned.
-- Restores the two triggers exactly as 20260917120000 created them. No data
-- impact.

DROP TRIGGER IF EXISTS drifted_handle_healing_work_insert ON drifted_handle;
DROP TRIGGER IF EXISTS drifted_handle_healing_work_update ON drifted_handle;

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
        OR NEW.quorum_ct64_digest IS DISTINCT FROM OLD.quorum_ct64_digest
        OR NEW.peer_sources IS DISTINCT FROM OLD.peer_sources
        OR NEW.reason IS DISTINCT FROM OLD.reason
    ))
    EXECUTE FUNCTION notify_healing_work();
