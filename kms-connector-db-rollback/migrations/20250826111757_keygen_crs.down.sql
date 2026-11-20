-- Down for 20250826111757_keygen_crs.
--
-- Restores the key generation tables as left by 20250604143523_kms_worker_notify:
--   * drops the prep keygen, keygen and crsgen response tables, the prep keygen
--     request table, their triggers and functions, and the `key_digest`,
--     `key_type` and `params_type` types;
--   * renames the `keygen_requests` and `crsgen_requests` columns back, moves
--     the `keygen_requests` primary key back to `pre_key_id`, and drops
--     `crsgen_requests.params_type`;
--   * restores the `crs_request_available` channel of `notify_crsgen_request`;
--   * recreates the `preprocess_keygen_requests` and
--     `preprocess_kskgen_requests` tables with their notify functions and
--     triggers.
--
-- The up repurposed existing columns: `keygen_requests.fhe_params_digest`
-- became `key_id`, `crsgen_requests.fhe_params_digest` became
-- `max_bit_length`. Renaming them back would label one kind of value as
-- another, so this fails, without changing anything, while either table has
-- rows.
--
-- Forced rollback (`migrate.sh rollback <target> --force`): those rows are
-- deleted and the revert goes ahead. Meant for test environments.

DO $$
DECLARE
    keygen bigint;
    crsgen bigint;
    forced boolean;
BEGIN
    forced := coalesce(current_setting('kms_connector_db_rollback.force', true), '') = 'on';

    SELECT count(*) INTO keygen FROM keygen_requests;
    SELECT count(*) INTO crsgen FROM crsgen_requests;

    IF keygen + crsgen > 0 AND NOT forced THEN
        RAISE EXCEPTION
            'Cannot restore the previous key generation tables: % rows in keygen_requests and % rows in crsgen_requests hold values the previous columns do not describe. Force the rollback to delete them.',
            keygen, crsgen;
    END IF;

    IF keygen + crsgen > 0 THEN
        DELETE FROM keygen_requests;
        DELETE FROM crsgen_requests;

        RAISE NOTICE 'Forced rollback: deleted % keygen_requests rows and % crsgen_requests rows.', keygen, crsgen;
    END IF;
END
$$;

--------------------------------------------------------
--                       Crsgen                       --
--------------------------------------------------------
DROP TABLE crsgen_responses;
DROP FUNCTION notify_crsgen_response();
DROP FUNCTION delete_from_crsgen_requests();

ALTER TABLE crsgen_requests DROP COLUMN params_type;
ALTER TABLE crsgen_requests DROP CONSTRAINT crsgen_requests_pkey;
ALTER TABLE crsgen_requests RENAME crs_id TO crsgen_request_id;
ALTER TABLE crsgen_requests ADD PRIMARY KEY (crsgen_request_id);
ALTER TABLE crsgen_requests RENAME max_bit_length TO fhe_params_digest;

CREATE OR REPLACE FUNCTION notify_crsgen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY crs_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

--------------------------------------------------------
--                       Keygen                       --
--------------------------------------------------------
DROP TABLE keygen_responses;
DROP FUNCTION notify_keygen_response();
DROP FUNCTION delete_from_keygen_requests();
DROP TYPE key_digest;

ALTER TABLE keygen_requests DROP CONSTRAINT keygen_requests_pkey;
ALTER TABLE keygen_requests RENAME prep_keygen_id TO pre_key_id;
ALTER TABLE keygen_requests ADD PRIMARY KEY (pre_key_id);
ALTER TABLE keygen_requests RENAME key_id TO fhe_params_digest;

--------------------------------------------------------
--                     PrepKeygen                     --
--------------------------------------------------------
DROP TABLE prep_keygen_responses;
DROP TABLE prep_keygen_requests;
DROP FUNCTION notify_prep_keygen_request();
DROP FUNCTION notify_prep_keygen_response();
DROP FUNCTION delete_from_prep_keygen_requests();

DROP TYPE key_type;
DROP TYPE params_type;

--------------------------------------------------------
--         Tables and triggers the up dropped         --
--------------------------------------------------------
CREATE TABLE preprocess_keygen_requests (
    pre_keygen_request_id BYTEA NOT NULL,
    fhe_params_digest BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (pre_keygen_request_id)
);

CREATE TABLE preprocess_kskgen_requests (
    pre_kskgen_request_id BYTEA NOT NULL,
    fhe_params_digest BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (pre_kskgen_request_id)
);

CREATE FUNCTION notify_preprocess_keygen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY preprocess_keygen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION notify_preprocess_kskgen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY preprocess_kskgen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_preprocess_keygen_requests_insertions
    AFTER INSERT
    ON preprocess_keygen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_preprocess_keygen_request();

CREATE TRIGGER trigger_from_preprocess_kskgen_requests_insertions
    AFTER INSERT
    ON preprocess_kskgen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_preprocess_kskgen_request();
