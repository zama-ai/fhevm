-- Rights of the prf-service and pad-worker roles. initialize_db.sh runs this file after the
-- migrations, on each deploy. A role that does not exist gets no grants.
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

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'pad_worker') THEN
        -- Additive: an older copy of this file must not remove a right of a newer pad-worker. Revoke
        -- only what the role must never have.
        REVOKE INSERT, TRUNCATE, REFERENCES, TRIGGER ON pads FROM pad_worker;
        REVOKE UPDATE (prf_instance, label, pad_length, triggered_by, created_at)
            ON pads FROM pad_worker;
        REVOKE INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER ON keys FROM pad_worker;
        -- No INSERT, and no UPDATE of the key or the payer.
        GRANT SELECT, DELETE ON pads TO pad_worker;
        GRANT UPDATE (
            claimed_at, attempts, error_message, is_error, s3_verified_at,
            ciphertext64, ciphertext128, ciphertext_digest, ciphertext128_digest, signer, signature,
            attestation_version
        ) ON pads TO pad_worker;
        GRANT SELECT ON keys TO pad_worker;
    ELSE
        RAISE NOTICE 'Role pad_worker does not exist: no grants';
    END IF;
END
$$;
