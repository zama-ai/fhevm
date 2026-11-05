-- Gives the database role of prf-service its rights on `pads`. initialize_db.sh runs this file after
-- the migrations, on each deploy. The role is created outside the migrations. Until then, this file
-- does nothing.
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'prf_service') THEN
        -- Revoke all first, so a right removed from this file is also removed from the role.
        REVOKE ALL ON pads FROM prf_service;
        GRANT SELECT ON pads TO prf_service;
        -- prf-service only adds rows. It cannot change a row after that.
        GRANT INSERT (prf_instance, label, pad_length, triggered_by) ON pads TO prf_service;
    ELSE
        RAISE NOTICE 'Role prf_service does not exist: no grants';
    END IF;
END
$$;
