-- Drop the allowed handle notify trigger and function as they are no longer used by transaction-sender.
DROP TRIGGER IF EXISTS on_insert_notify_event_allowed_handle ON allowed_handles;
DROP FUNCTION IF EXISTS notify_event_allowed_handle();