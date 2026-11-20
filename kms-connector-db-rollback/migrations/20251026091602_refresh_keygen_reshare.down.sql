-- Down for 20251026091602_refresh_keygen_reshare.
--
-- Drops the `key_reshare_same_set` table with its trigger, and the function
-- the trigger called. The stored events are lost.

DROP TABLE key_reshare_same_set;
DROP FUNCTION notify_key_reshare_same_set();
