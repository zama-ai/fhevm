UPDATE public.consensus_test_host_report_fault
   SET reports = reports || jsonb_build_object(
       $2::text, jsonb_build_object(
           'chain', ($1::bigint)::text,
           'block', $2::text,
           'original', $3::text,
           'blockHash', $4::text,
           'bucket', $5::text))
 WHERE chain_id = $1
