-- Notify Pbs computations
CREATE OR REPLACE FUNCTION notify_event_pbs_computations()
    RETURNS trigger AS $$
BEGIN
    NOTIFY event_pbs_computations;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER on_insert_notify_event_pbs_computations
    AFTER INSERT
    ON pbs_computations
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_event_pbs_computations();


-- Notify Allowed handles
CREATE OR REPLACE FUNCTION notify_event_allowed_handle()
    RETURNS trigger AS $$
BEGIN
    NOTIFY event_allowed_handle;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER on_insert_notify_event_allowed_handle
    AFTER INSERT
    ON allowed_handles
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_event_allowed_handle();
