-- Down for 20251025081948_prss_init.
--
-- Drops the `prss_init` table with its trigger, and the function the trigger
-- called. The stored events are lost.

DROP TABLE prss_init;
DROP FUNCTION notify_prss_init();
