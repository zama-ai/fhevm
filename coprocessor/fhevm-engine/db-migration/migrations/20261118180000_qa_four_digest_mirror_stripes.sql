-- QA BRANCH ONLY. Never backport this file or deploy it to a shared database.
-- Preserve each installed trigger body, changing only its stripe modulus.
-- The ordered SNS binary on this QA branch uses the matching modulus four.
DO $qa_four_stripes$
DECLARE
    function_name TEXT;
    definition TEXT;
BEGIN
    FOREACH function_name IN ARRAY ARRAY[
        'mirror_ciphertext_digest_branchless',
        'mirror_ciphertext_digest_for_pbs_context'
    ] LOOP
        SELECT pg_get_functiondef(to_regprocedure('public.' || function_name || '()'))
          INTO definition;
        IF definition IS NULL THEN
            RAISE EXCEPTION 'QA mirror function not found: %', function_name;
        END IF;
        IF regexp_count(definition, '% 256') = 1 THEN
            EXECUTE replace(definition, '% 256', '% 4');
        ELSIF regexp_count(definition, '% 4([^0-9]|$)') <> 1 THEN
            RAISE EXCEPTION 'Unexpected QA stripe expression in %', function_name;
        END IF;
    END LOOP;
END
$qa_four_stripes$;
