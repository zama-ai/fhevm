-- Create function to notify on work updates
CREATE OR REPLACE FUNCTION notify_work_available()
    RETURNS trigger AS $$
BEGIN
    -- Notify all listeners of work_updated channel
    NOTIFY work_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to fire once per statement on computations inserts
CREATE TRIGGER work_updated_trigger_from_computations_insertions
    AFTER INSERT
    ON computations
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_work_available();
