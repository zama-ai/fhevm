-- Down for 20251208161022_implem_garbage_collection.
--
-- Replaces the `status` enum by the `under_process` flag again, drops the
-- `updated_at` columns with their refresh triggers, the completion triggers
-- and the four decryption indexes, and restores the triggers that removed a
-- request as soon as its response was inserted.
--
-- Status mapping: the previous version picks up every row whose flag is
-- FALSE. Only a `pending` row may be picked up again, so `pending` becomes
-- FALSE and every other status (`under_process`, `completed`, `failed`)
-- becomes TRUE, which keeps finished work from being processed or sent a
-- second time. The distinction between those three statuses is lost.

--------------------------------------------------------------------------------------------------
--                         `updated_at` / `status` triggers and functions                        --
--------------------------------------------------------------------------------------------------
DROP TRIGGER refresh_updated_at_public_decryption_requests_on_update ON public_decryption_requests;
DROP TRIGGER refresh_updated_at_user_decryption_requests_on_update ON user_decryption_requests;
DROP TRIGGER refresh_updated_at_prep_keygen_requests_on_update ON prep_keygen_requests;
DROP TRIGGER refresh_updated_at_keygen_requests_on_update ON keygen_requests;
DROP TRIGGER refresh_updated_at_crsgen_requests_on_update ON crsgen_requests;
DROP TRIGGER refresh_updated_at_prss_init_on_update ON prss_init;
DROP TRIGGER refresh_updated_at_key_reshare_same_set_on_update ON key_reshare_same_set;
DROP TRIGGER refresh_updated_at_public_decryption_responses_on_update ON public_decryption_responses;
DROP TRIGGER refresh_updated_at_user_decryption_responses_on_update ON user_decryption_responses;
DROP TRIGGER refresh_updated_at_prep_keygen_responses_on_update ON prep_keygen_responses;
DROP TRIGGER refresh_updated_at_keygen_responses_on_update ON keygen_responses;
DROP TRIGGER refresh_updated_at_crsgen_responses_on_update ON crsgen_responses;

DROP TRIGGER complete_public_decryption_request_on_response_insert ON public_decryption_responses;
DROP TRIGGER complete_user_decryption_requests_on_response_insert ON user_decryption_responses;
DROP TRIGGER complete_prep_keygen_requests_on_response_insert ON prep_keygen_responses;
DROP TRIGGER complete_keygen_requests_on_response_insert ON keygen_responses;
DROP TRIGGER complete_crsgen_requests_on_response_insert ON crsgen_responses;

DROP FUNCTION refresh_updated_at_public_decryption_request();
DROP FUNCTION refresh_updated_at_user_decryption_requests();
DROP FUNCTION refresh_updated_at_prep_keygen_requests();
DROP FUNCTION refresh_updated_at_keygen_requests();
DROP FUNCTION refresh_updated_at_crsgen_requests();
DROP FUNCTION refresh_updated_at_prss_init();
DROP FUNCTION refresh_updated_at_key_reshare_same_set();
DROP FUNCTION refresh_updated_at_public_decryption_responses();
DROP FUNCTION refresh_updated_at_user_decryption_responses();
DROP FUNCTION refresh_updated_at_prep_keygen_responses();
DROP FUNCTION refresh_updated_at_keygen_responses();
DROP FUNCTION refresh_updated_at_crsgen_responses();

DROP FUNCTION complete_public_decryption_request();
DROP FUNCTION complete_user_decryption_request();
DROP FUNCTION complete_prep_keygen_now();
DROP FUNCTION complete_keygen_now();
DROP FUNCTION complete_crsgen_now();

--------------------------------------------------------------------------------------------------
--                                 Indexes and `updated_at` columns                              --
--------------------------------------------------------------------------------------------------
DROP INDEX idx_public_decryption_requests_status_updated_at;
DROP INDEX idx_user_decryption_requests_status_updated_at;
DROP INDEX idx_public_decryption_responses_status_updated_at;
DROP INDEX idx_user_decryption_responses_status_updated_at;

ALTER TABLE public_decryption_requests DROP COLUMN updated_at;
ALTER TABLE user_decryption_requests DROP COLUMN updated_at;
ALTER TABLE prep_keygen_requests DROP COLUMN updated_at;
ALTER TABLE keygen_requests DROP COLUMN updated_at;
ALTER TABLE crsgen_requests DROP COLUMN updated_at;
ALTER TABLE prss_init DROP COLUMN updated_at;
ALTER TABLE key_reshare_same_set DROP COLUMN updated_at;

ALTER TABLE public_decryption_responses DROP COLUMN updated_at;
ALTER TABLE user_decryption_responses DROP COLUMN updated_at;
ALTER TABLE prep_keygen_responses DROP COLUMN updated_at;
ALTER TABLE keygen_responses DROP COLUMN updated_at;
ALTER TABLE crsgen_responses DROP COLUMN updated_at;

--------------------------------------------------------------------------------------------------
--                              `status` back to the `under_process` flag                        --
--------------------------------------------------------------------------------------------------
ALTER TABLE public_decryption_requests ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE user_decryption_requests ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE prep_keygen_requests ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE keygen_requests ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE crsgen_requests ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE prss_init ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE key_reshare_same_set ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE public_decryption_responses ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE user_decryption_responses ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE prep_keygen_responses ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE keygen_responses ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE crsgen_responses ADD COLUMN under_process BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE public_decryption_requests SET under_process = TRUE WHERE status <> 'pending';
UPDATE user_decryption_requests SET under_process = TRUE WHERE status <> 'pending';
UPDATE prep_keygen_requests SET under_process = TRUE WHERE status <> 'pending';
UPDATE keygen_requests SET under_process = TRUE WHERE status <> 'pending';
UPDATE crsgen_requests SET under_process = TRUE WHERE status <> 'pending';
UPDATE prss_init SET under_process = TRUE WHERE status <> 'pending';
UPDATE key_reshare_same_set SET under_process = TRUE WHERE status <> 'pending';

UPDATE public_decryption_responses SET under_process = TRUE WHERE status <> 'pending';
UPDATE user_decryption_responses SET under_process = TRUE WHERE status <> 'pending';
UPDATE prep_keygen_responses SET under_process = TRUE WHERE status <> 'pending';
UPDATE keygen_responses SET under_process = TRUE WHERE status <> 'pending';
UPDATE crsgen_responses SET under_process = TRUE WHERE status <> 'pending';

ALTER TABLE public_decryption_requests DROP COLUMN status;
ALTER TABLE user_decryption_requests DROP COLUMN status;
ALTER TABLE prep_keygen_requests DROP COLUMN status;
ALTER TABLE keygen_requests DROP COLUMN status;
ALTER TABLE crsgen_requests DROP COLUMN status;
ALTER TABLE prss_init DROP COLUMN status;
ALTER TABLE key_reshare_same_set DROP COLUMN status;

ALTER TABLE public_decryption_responses DROP COLUMN status;
ALTER TABLE user_decryption_responses DROP COLUMN status;
ALTER TABLE prep_keygen_responses DROP COLUMN status;
ALTER TABLE keygen_responses DROP COLUMN status;
ALTER TABLE crsgen_responses DROP COLUMN status;

DROP TYPE operation_status;

--------------------------------------------------------------------------------------------------
--                    Autoremoval of a request when its response is inserted                    --
--------------------------------------------------------------------------------------------------
-- As created by 20250604141515_decryption_response and 20250826111757_keygen_crs.
CREATE FUNCTION delete_from_public_decryption_requests()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM public_decryption_requests WHERE decryption_id = NEW.decryption_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_public_decryption_requests
AFTER INSERT ON public_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION delete_from_public_decryption_requests();

CREATE FUNCTION delete_from_user_decryption_requests()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM user_decryption_requests WHERE decryption_id = NEW.decryption_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_user_decryption_requests
AFTER INSERT ON user_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION delete_from_user_decryption_requests();

CREATE FUNCTION delete_from_prep_keygen_requests()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM prep_keygen_requests WHERE prep_keygen_id = NEW.prep_keygen_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_delete_prep_keygen_requests
AFTER INSERT ON prep_keygen_responses
FOR EACH ROW
EXECUTE FUNCTION delete_from_prep_keygen_requests();

CREATE FUNCTION delete_from_keygen_requests()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM keygen_requests WHERE key_id = NEW.key_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_delete_keygen_requests
AFTER INSERT ON keygen_responses
FOR EACH ROW
EXECUTE FUNCTION delete_from_keygen_requests();

CREATE FUNCTION delete_from_crsgen_requests()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM crsgen_requests WHERE crs_id = NEW.crs_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_delete_crsgen_requests
AFTER INSERT ON crsgen_responses
FOR EACH ROW
EXECUTE FUNCTION delete_from_crsgen_requests();
