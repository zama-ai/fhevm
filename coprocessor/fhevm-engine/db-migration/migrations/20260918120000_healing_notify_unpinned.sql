-- Wake healing for unhealed ct64-repair rows even before a target is pinned
-- (inferred findings start with NULL quorum_ct64_digest).
DROP TRIGGER IF EXISTS drifted_handle_healing_work_insert ON drifted_handle;
DROP TRIGGER IF EXISTS drifted_handle_healing_work_update ON drifted_handle;

CREATE TRIGGER drifted_handle_healing_work_insert
    AFTER INSERT
    ON drifted_handle
    FOR EACH ROW
    WHEN (
        NEW.healed_at IS NULL
        AND NEW.reason IN (
            'ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here'
        )
    )
    EXECUTE FUNCTION notify_healing_work();

CREATE TRIGGER drifted_handle_healing_work_update
    AFTER UPDATE
    ON drifted_handle
    FOR EACH ROW
    WHEN (
        NEW.healed_at IS NULL
        AND NEW.reason IN (
            'ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here'
        )
        AND (
            NEW.healed_at IS DISTINCT FROM OLD.healed_at
            OR NEW.next_retry_at IS DISTINCT FROM OLD.next_retry_at
            OR NEW.quorum_ct64_digest IS DISTINCT FROM OLD.quorum_ct64_digest
            OR NEW.peer_sources IS DISTINCT FROM OLD.peer_sources
            OR NEW.reason IS DISTINCT FROM OLD.reason
        )
    )
    EXECUTE FUNCTION notify_healing_work();
