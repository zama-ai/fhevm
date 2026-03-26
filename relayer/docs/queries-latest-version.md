## SCENARIOS :

### FIRST TODO:

-- TODO: change this to uniform names. (internal_decryption_id to internal_indexer_id) to bytea and enable hashmap ! we store as bytea not text (possibility of hash index) OK.
-- TODO: Change tx_sent to receipt_received status (MORE CLARITY.) OK.
-- DESIGNS COMMENTS: RETURN AS MINIMAL FIELDS AS POSSIBLE.
-- use consistent structure for internal_req_id and external_req_id (or int ext) Ok.

### NEEDED DATA STRUCT:

1.  At startup of the relayer: (purpose, take the state where we lefted at.)
    2 queries -> 1. 1. query all tables mixed and ordered by updated_at for status = 'queued', returns: int_indexer_id, req
    -> We will trigger readiness checker on this one. ()

        2. query all tables mixed and ordered by updated_at for status = 'processing'
            returns: int_indexer_id, req
        -> will trigger transaction helper -> (readiness has already passed.)

### USER DECRYPT:

1.  POST REQUEST => we receive a payload (v2)

    1.  Compute `internal_indexer_id` from payload
    2.  Check if it there in the `user_decrypt_req` table.
        1. If it already exists: return the `ext_reference_id` to the user with 202 OK.
        2. If not.
           1. Call readiness checker as an async function.
              1. if failure: -> Return 400 Code: Not ready For Decryption.
              2. if succeed: Insert the `req`, `ext_reference_id`, `int_indexer_id`. use ON CONFLICT METHOD for insert. if conflict get `ext_reference_id`.
                 v2 routes -> return ext reqId with 202 Created..
              3. if new creation.

2.  NEW FLOW POST:

    1.  Compute `internal_indexer_id` from payload
    2.  Check if it there in the `user_decrypt_req` table.

        1. If it already exists: return the `ext_reference_id` to the user with 202 OK.
        2. If not.

           1. Call host ACL readiness checker as an async function. (DUMMY always pass for next implem - we will do it later with a host listener - substreams/poller...)
              1. if failure: -> Return 400 Code: Not ready For Decryption.
              2. if succeed: Insert the `req`, `ext_reference_id`, `int_indexer_id`. use ON CONFLICT METHOD for insert. if conflict get `ext_reference_id`.
                 v2 routes -> return ext reqId with 202 Created..
              3. if new creation.

3.  Gateway readiness checker is triggered (large timeout, ~30 min coverage)

    1. if ready -> update user_decrypt_req by `int_indexer_id` for field req_status = `processing` -> Transaction::Send
    2. if not ready after 30min.. -> update by `int_indexer_id` req_status = `timed_out` + `err_reason`
       -> UserDecrypt::Failed emitted.

4.  RelayerEvent Transaction::Send is emitted.

    1. TxHandler emitt the event Transaction::Sucess (receipt)
       1. update the status to `receipt_received` + `gw_req_tx_hash` + `gw_reference_id` by `int_indexer_id`(we have the receipt at this point) -> We process the receipt. and we dispatch
    2. TxHandler emit Transaction::Failed: set status to `failure` and `err_reason` by `int_indexer_id` -> Dispatch error event as before to the orchestrator

-- Execute every one min in sql queries pg_cron. (We do at the end not now, just write the query.)
// later do it as a pg_cron !!! 5. If status == 'receipt_recieved' and now - `updated_at` > 30 min. - update by `int_indexer_id` req_status = `timed_out` + `err_reason`='response timed out' (SAME QUERY IN POST MODE.)

6.  Listener recieve user decrypt share or user consensus reached events transaction.

- If we recieve consensus reached tx:
  TODO: CHANGE THIS ONE !
- update `user_decrypt_req` table `gw_consensus_tx_hash` = value, where `gw_reference_id` if consensus_tx_hash is null, and status = 'receipt_recieved' and return (status, updated_at, err_reason, int_indexer_id,)
  - can be status = 'receipt_recieved' -> We ignore.
  - status = 'timed_out' -> (get err_reason, int_indexer_id, status, updated_at)

Rationale:
Want to update the table when status = 'receipt_recieved' in either case. I want the actual status in the raw.
for example: -> if the status is timed_out, i want to get status and updated at, in this case row affected will be zero. (we need err_reason and int_indexer_id)

in one query:
select `user_decrypt_req` where `gw_reference_id` and if consensus_tx_hash=null, and if status = 'receipt_recieved' update `consensus_tx_hash` = value received -> return (status, updated_at, err_reason, int_indexer_id,)

OR:
update `user_decrypt_req` table `gw_consensus_tx_hash` = value, where `gw_reference_id` if consensus_tx_hash is null, and status = 'receipt_recieved'
select (status, updated_at, err_reason, int_indexer_id,) where `gw_reference_id` = value

here. 6. IF THIS IS A SHARE: [2 transactions / 2 calls]

// This lead to possibility of non relevant shares (DOCUMENT THIS !!!!! ON CODE.)
// This also does not respect the status due to timeout on request context (ud tables.)
// Even for timeouts, we are registering incoming shares.

    transaction/call 1. insert into `user_decrypt_share` -> gw_reference_id, share_index, share, kms_signature, extra_data -> Return the count of total shares by `gw_reference_id` (QUERY)

    transaction/call 2. if count = threshold -> update `user_decrypt_req` table status = `completed` on `gw_reference_id` status != 'timeout' (ONLY ONE SINGLE TX QUERY)
        - return all the shares (IN THE SAME TX QUERY) + `int_indexer_id` + status + updated_at + err_reason

7.  INternally: we forward event is recieved as it is already done in our internal logic. (timeout logic etc...)

8.  GET REQUEST will pass to get route: `ext_req_id`

    1. select in `user_decrypt_req` by `ext_reference_id` and join on `gw_reference_id` to get all lines of `user_decrypt_share` 1 query. (need status field on query return + shares + updated at field)

    - if status == `completed` -> construct the response with the fields we queried.
    - if status == `processing` -> return updated_at and ext_request_id and status with 202.
    - if status == `queued` or `receipt_receieved` -> return back `ext_reference_id` with `status` and `updated_at` field.
    - if status == `timed_out` 504 return `ext_req_id` + `status`.
    - if status == `failure` 400 return `ext_req_id` + `status` + `err_reason`.

DONE FOR U.D.

NOTE: PAUSING STRATEGY.

### PUBLIC DECRYPT:

1.  POST REQUEST => we recieve a payload (v2)

    1.  Compute `internal_indexer_id` from payload
    2.  Check if it there in the `public_decrypt_req` table.
        1. If it already exists: return the `ext_req_id` to the user with 202 OK.
        2. If not.
           1. Call readiness checker as an async function.
              1. if failure: -> Return 400 Code: Not ready For Decryption.
              2. if succeed: Insert the `req`, `ext_reference_id`, `internal_indexer_id`. use ON CONFLICT METHOD for insert. if conflict get `ext_reference_id`.
                 v2 routes -> return ext reqId with 202 Created..
              3. if new creation.

2.  NEW POST FLOW (v2)

    1.  Compute `int_indexer_id` from payload
    2.  Check if it there in the `public_decrypt_req` table.

        1.  If it already exists: return the `ext_req_id` to the user with 202 OK.
            In new API, if already exists, we can return the result to user. so we need res + status as well. Buld it.
        2.  If not.

        3.  Call host ACL readiness checker as an async function. (DUMMY always pass for next implem - we will do it later with a host listener - substreams/poller...)
            1.  if failure: -> Return 400 Code: Not ready For Decryption.
            2.  if succeed: Insert the `req`, `ext_reference_id`, `internal_indexer_id`. use ON CONFLICT METHOD for insert. if conflict get `ext_reference_id`.
                v2 routes -> return ext reqId with 202 Created..
            3.  if new creation.

3.  Gateway readiness checker is triggered (large timeout, ~30 min coverage)

    1. if ready -> update public_decrypt_req by `int_indexer_id` for field req_status = `processing` -> Transaction::Send
    2. if not ready after 30min.. -> update by `int_indexer_id` req_status = `timed_out` + `err_reason`
       -> PublicDecrypt::Failed emitted.

4.  RelayerEvent Transaction::Send is emitted.

    1. TxHandler emitt the event Transaction::Sucess (receipt)
       1. update the status to `receipt_received` + `gw_req_tx_hash` + `gw_reference_id` by `int_indexer_id` (we have the receipt at this point) -> We process the receipt. and we dispatch
    2. TxHandler emit Transaction::Failed: set status to `failure` and `err_reason` by `int_indexer_id` -> Dispatch error event as before to the orchestrator

5.  Listener recieve public_decrypt share events transaction.

- update into public_decrypt_req by gw_reference_id, res = recieved value, completed status where status != timed_out (ret -> int_indexer_id, status, updated_at, err_reason)

4.  INternally: we forward event is recieved as it is already done in our internal logic.

5.  GET REQUEST will pass to get route: `ext_reference_id`

    1. select in `public_decrypt_req` by `ext_reference_id` (need status `res` and `err_reason` and `updated_at` and `ext_request_id`)

    - if status == `completed` -> we return 200 with response.
    - if status == `processing` -> return updated_at and ext_request_id and status with 202.
    - if status == `queued` or `receipt_receieved` -> return back `ext_reference_id` with `status` and `updated_at` field.
    - if status == `timed_out` 504 return `ext_req_id` + `status`.
    - if status == `failure` 400 return `ext_req_id` + `status` + `err_reason`.

TIME OUT STRATEGY:
-- Execute every one min in sql queries pg_cron. (We do at the end not now, just write the query.)
// later do it as a pg_cron !!! 5. If status == 'receipt_recieved' and now - `updated_at` > 30 min. - update by `int_indexer_id` req_status = `timed_out` + `err_reason`='response timed out' (SAME QUERY IN POST MODE.)

### INPUT PROOF

- Comment: no readiness check

1.  POST REQUEST => we recieve a payload (v2)

    - Create uuidV7 `internal_request_id`.
    - Insert `ext_reference_id`, `int_request_id`, `request` into `input_proof_req`
      v2 routes -> return `ext_reference_id` with 202 Created..

2.  RelayerEvent Transaction::Send is emitted.

    1. TxHandler emitt the event Transaction::Sucess (receipt)
       1. update the status to `receipt_received` + `gw_req_tx_hash` + `gw_reference_id` by `int_request_id`(we have the receipt at this point) -> We process the receipt. and we dispatch
    2. TxHandler emit Transaction::Failed: set status to `failure` and `err_reason` by `int_request_id` -> Dispatch error event as before to the orchestrator

3.  Listener recieve input_proof share events transaction.

- if input proof accepted:
  - Update into `input_proof_req` table: `res` = recieved value from gw where gateway_reference_id = value in the event and req_status = completed and accepted = `true` + `gw_response_tx_hash` (return `int_request_id`)
- if input proof rejected:
  - update input_proof_req with accpeted=false req_status=completed gw_response_tx_hash=tx hash of event (return `int_request_id`) (adding res to it as well...)

6.  INternally: we forward event is recieved as it is already done in our internal logic.

7.  GET REQUEST will pass to get route: `ext_reference_id`

    1. select in `input_proof_req` by `ext_reference_id` (need status `response` and `err_reason` and `updated_at`, and `accepted` and `req_status`)

    - if status == `completed` -> we return 200 with response and `accepted`
    - if status == `queued` or `receipt_receieved` -> return back `ext_req_id` with `status` and `updated_at` field. and accepted=null
    - if status == `timed_out` 504 return `ext_req_id` + `status`.
    - if status == `failure` 400 return `ext_req_id` + `status` + `err_reason`.

TIME OUT STRATEGY:
-- Execute every one min in sql queries pg_cron. (We do at the end not now, just write the query.)
// later do it as a pg_cron !!! 5. If status == 'receipt_recieved' and now - `updated_at` > 30 min. - update by `int_indexer_id` req_status = `timed_out` + `err_reason`='response timed out' (SAME QUERY IN POST MODE.)
