-- Down for 20260804000000_add_compressed_key_set.
--
-- Removes the `CompressedKeySet` value from the `key_type` enum.
--
-- Postgres has no `ALTER TYPE ... DROP VALUE`, and `key_type` is used through
-- the `key_digest` composite type stored in `keygen_responses.key_digests`. So
-- the column is moved to text, both types are recreated as left by
-- 20250826111757_keygen_crs, and the column is cast back.
--
-- A keygen response holding a `CompressedKeySet` digest cannot be represented
-- afterwards. This fails, without changing anything, while such a response
-- exists.
--
-- Forced rollback (`migrate.sh rollback <target> --force`): those responses are
-- deleted and the revert goes ahead. Meant for test environments.

DO $$
DECLARE
    compressed bigint;
    forced boolean;
BEGIN
    forced := coalesce(current_setting('kms_connector_db_rollback.force', true), '') = 'on';

    SELECT count(*) INTO compressed
    FROM keygen_responses r
    WHERE EXISTS (
        SELECT 1 FROM unnest(r.key_digests) AS d
        WHERE d.key_type::text = 'CompressedKeySet'
    );

    IF compressed > 0 AND NOT forced THEN
        RAISE EXCEPTION
            'Cannot remove CompressedKeySet from key_type: % rows in keygen_responses hold a digest of that type. Force the rollback to delete them.',
            compressed;
    END IF;

    IF compressed > 0 THEN
        DELETE FROM keygen_responses r
        WHERE EXISTS (
            SELECT 1 FROM unnest(r.key_digests) AS d
            WHERE d.key_type::text = 'CompressedKeySet'
        );

        RAISE NOTICE 'Forced rollback: deleted % keygen_responses rows holding a CompressedKeySet digest.', compressed;
    END IF;
END
$$;

ALTER TABLE keygen_responses
    ALTER COLUMN key_digests TYPE text[] USING key_digests::text[];

DROP TYPE key_digest;
DROP TYPE key_type;

CREATE TYPE key_type AS ENUM ('Server', 'Public');
CREATE TYPE key_digest AS (
    key_type key_type,
    digest BYTEA
);

ALTER TABLE keygen_responses
    ALTER COLUMN key_digests TYPE key_digest[] USING key_digests::key_digest[];
