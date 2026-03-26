## Queries to create:

### User decrypt requests:

- (Medium frequency): Insert user decrypt request with internal Id (hash) and request
- (Medium frequency): When making a successful transaction, we update user decrypt by internal request id, for fields (gw request id, hash, status) or error reason/status
- (Medium/high frequency): Update by gatweay Id (number) with response jsonb and status to completed. - (triggered when user decrypt available)
- (Periodic query): LATER: Scan the table and update status timed_out request for too long queued.
- (High frequency): Select by external Id to get the response attached + internal request Id + Status. (GET ROUTE)
- (Internal transaction poller): Update status to in-flight for oldest queued status and retrieve request payload and internal ref id.(Limited number of in-flight requests at the same time).
- (At start): Query all tx_sent transaction and populate an hashmap with corresponding gw_request_id(key), and consensus reached (value) (boolean hashmap)
- (Medium update) - Update field consensus_reached by gateway id.

### User decrypt share requests:

- (APPLICATION LOGIC function): retrieve from hashset of all the tx_sent (gw_request_ids(key), Consensus reached boolean(val)).
- (High frequency): Insert shares only if on hashset of gw_request_id we carry - response is a number of share already in DB and no more.
- (Medium frequency): Select all shares by gateway Id. (Internally verify)
- (Periodic query): LATER: Remove shares by gateway Id..

### Public decrypt:

- (Medium frequency): Insert public decrypt request with internal Id (hash) and request
- (Medium frequency): When making a successful transaction, update user decrypt by internal request id, for fields (gw request id, hash, status) or error reason/status
- (Medium/high frequency): Update by gatweay Id (number) with response jsonb and status to completed. - (when public decrypt is available (event listener))
- (Periodic query): LATER: Scan the table and update status timed_out request for too long queued.
- (High frequency): Select by external Id to get the response attached + internal request Id + Status. (GET ROUTE)
- (Internal transaction poller): Update status to in-flight for oldest queued status and retrieve request payload and internal ref id. (Limited number of in-flight requests at the same time).

### Input proof:

- (Medium frequency): Insert public decrypt request with internal Id (hash) and request
- (Medium frequency): When making a successful transaction, update user decrypt by internal request id, for fields (gw request id, hash, status) or error reason/status
- (Medium/high frequency): Update by gatweay Id (number) with response jsonb and status to completed. - (when public decrypt is available (event listener))
- (Periodic query): LATER: Scan the table and update status timed_out request for too long queued.
- (High frequency): Select by external Id to get the response attached + internal request Id + Status. (GET ROUTE)
- (Internal transaction poller (each 1 sec)): Update status to in-flight for oldest queued status and retrieve request payload and internal ref id (Limited number of in-flight requests at the same time).
