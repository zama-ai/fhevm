# Test Findings

## Summary of fixes and steps taken (chronological)

1) **Detected Gateway InputVerification ABI mismatch (v1 vs v2).**
   - Found relayer tx simulation reverted when calling `registerInputVerification(bytes32,uint256,address,address,bytes)`.
   - On-chain Gateway InputVerificationRegistry only exposed legacy signature without `userSignature`.
   - Root cause: deployment pipeline only upgraded GatewayConfig/KMSGeneration/ProtocolPayment, leaving DecryptionRegistry/InputVerificationRegistry on old proxies.

2) **Updated gateway-contracts deployment tasks to include V2 registries.**
   - Added DecryptionRegistry/InputVerificationRegistry empty proxies to `task:deployEmptyUUPSProxies`.
   - Added deployment tasks for DecryptionRegistry/InputVerificationRegistry implementations and wired into `task:deployImplementationContracts`.
   - Preserved legacy address names (`Decryption`, `InputVerification`) for address env/solidity constants.

3) **Fixed gateway-contracts build failure due to new `apiUrl` fields.**
   - GatewayConfig now expects `apiUrl` fields in KmsNode/Coprocessor structs.
   - Added `KMS_NODE_API_URL_*` and `COPROCESSOR_API_URL_*` handling in deploy task.
   - Updated `gateway-contracts/.env.example` and staging gateway env to include API URLs.

4) **Resolved invalid initialization when upgrading new registries.**
   - DecryptionRegistry/InputVerificationRegistry used `reinitializer(1)` with `onlyFromEmptyProxy`.
   - OpenZeppelin proxy initializes empty proxies at version 1, so upgrade call reverted with `InvalidInitialization()`.
   - Bumped registry `REINITIALIZER_VERSION` to 2 to allow initialization on upgrade.

5) **Fixed missing KMS connector signer address.**
   - KMS worker failed with `missing configuration field "signer_address"`.
   - Added `KMS_CONNECTOR_SIGNER_ADDRESS` to `test-suite/fhevm/env/staging/.env.kms-connector`.
   - Extended `test-suite/fhevm/scripts/setup-kms-signer-address.sh` to populate it from kms-core.

6) **Address consistency after redeploying gateway contracts locally.**
   - Ran `DOTENV_CONFIG_PATH=../test-suite/fhevm/env/staging/.env.gateway-sc.local npx hardhat task:deployAllGatewayContracts` from `gateway-contracts/` to deploy V2 registries on the live gateway chain.
   - Updated local env/config files to match new deployed addresses:
     - `test-suite/fhevm/env/staging/.env.gateway-sc`
     - `test-suite/fhevm/env/staging/.env.host-sc`
     - `test-suite/fhevm/env/staging/.env.kms-connector`
     - `test-suite/fhevm/env/staging/.env.coprocessor`
     - `test-suite/fhevm/env/staging/.env.test-suite`
     - `test-suite/fhevm/env/staging/.env.gateway-mocked-payment`
     - `test-suite/fhevm/env/staging/.env.relayer`
     - `test-suite/fhevm/config/relayer/local.yaml`
     - `test-suite/e2e/.env.example`

7) **Corrected service startup order to avoid kms-worker failure.**
   - kms-worker was starting before Gateway contracts were deployed and failed on `getHostChains` (non-contract address).
   - Moved KMS connector startup to after `gateway-sc` and `host-sc` in `test-suite/fhevm/scripts/deploy-fhevm-stack.sh`.

8) **Ensured gateway-sc follow-up tasks use freshly deployed proxy addresses.**
   - `gateway-sc-add-network`, `gateway-sc-trigger-keygen`, `gateway-sc-trigger-crsgen`, and `gateway-sc-add-pausers` now use `--use-internal-proxy-address` / `--use-internal-pauser-set-address` so they read addresses from the shared `addresses-volume`.
   - Fixed a bug in `gateway-contracts/tasks/addPausers.ts` where the flag name was ignored.

9) **Automated address propagation after gateway-sc deploy.**
   - Added `sync_gateway_addresses_from_volume` in `test-suite/fhevm/scripts/deploy-fhevm-stack.sh` to copy deployed addresses from the docker volume into local env/config files (kms-connector, coprocessor, relayer, test-suite, host-sc, gateway-mocked-payment, gateway-sc).
   - This prevents KMS/Coprocessor/Relayer from pointing to stale contract addresses after a fresh chain deploy.

10) **Resolved relayer container and database startup issues.**
    - Relayer startup failed due to an existing `fhevm-relayer` container; removed it and restarted the relayer container.
    - Relayer then failed because `relayer_db` did not exist in Postgres; created the database in `coprocessor-and-kms-db` and restarted relayer.

11) **Brought up test-suite container for e2e execution.**
    - Started `test-suite` docker compose using the updated staging env.

12) **Identified missing key material in MinIO after keygen/crsgen.**
    - `input-proof` test failed due to 404 on the public key URL in MinIO.
    - On-chain `getActiveKeyId()` and `getActiveCrsId()` returned `0` (no active key/CRS).
    - KMS connector gateway listener starts from `latest` and skips catchup, so it missed earlier keygen/crsgen events.
    - Re-triggered keygen/crsgen after KMS connector start, but active key/CRS IDs still remained `0` at time of check.

13) **Enabled KMS connector catchup for KMS operations.**
    - Added `KMS_CONNECTOR_KMS_OPERATION_FROM_BLOCK_NUMBER=0` in `.env.kms-connector` and `.env.kms-connector.local`.
    - Restarted KMS connector so Keygen/Crsgen subscriptions catch up from genesis instead of `latest`.

14) **Recovered keygen/crsgen end-to-end after catchup.**
    - Re-triggered keygen/crsgen, confirmed KMS worker processed requests and tx-sender submitted responses.
    - Active IDs are now set on-chain (`getActiveKeyId` and `getActiveCrsId` non-zero).
    - MinIO now serves both the public key and CRS objects for the active IDs.

15) **Applied relayer DB migrations (missing tables caused 500s).**
    - Relayer returned 500 with `relation "input_proof_req" does not exist`.
    - Applied `relayer-migrate/migrations/20251109145104_create_tables.sql` to `relayer_db` and restarted relayer.

16) **Rebuilt relayer locally to pick up V2 signature changes.**
    - GHCR relayer image lacked `registerInputVerification(...)` with user signature (v2 flow).
    - Built a local relayer image from `console/apps/relayer` with gateway rust bindings mounted in.
    - Compiled with `DATABASE_URL=postgresql://postgres:postgres@host.docker.internal:5432/relayer_db` and relaunched relayer with `RELAYER_VERSION=local`.

17) **Triaged input-proof failure (revert during gas estimation).**
    - `input-proof` still failed with `Unexpected input proof response`; relayer logs showed revert with `ERC20InsufficientAllowance`.
    - Initially suspected address mismatch from `/data/.env.gateway`; verified later that deployed addresses matched their contract types via `getVersion()`.
    - Kept investigation focused on payment approvals rather than address mapping.

18) **Fixed input-proof failure by re-approving ProtocolPayment after gateway deploy.**
    - Verified on-chain that `PROTOCOL_PAYMENT_ADDRESS`, `DECRYPTION_ADDRESS`, and `INPUT_VERIFICATION_ADDRESS` point to the correct contracts (via `getVersion()`).
    - Found relayer (tx-sender) allowance for `ProtocolPayment` was `0`, causing `ERC20InsufficientAllowance`.
    - Root cause: `gateway-set-relayer-mocked-payment` was executed before `gateway-sc` deploy, so approvals were set for a stale ProtocolPayment address.
    - Ran `gateway-set-relayer-mocked-payment` after gateway deploy to mint mocked $ZAMA and approve current ProtocolPayment; allowance now max.
    - Updated `test-suite/fhevm/scripts/deploy-fhevm-stack.sh` to run:
      - `gateway-deploy-mocked-zama-oft` before `gateway-sc` (needed for addresses),
      - `gateway-set-relayer-mocked-payment` after `sync_gateway_addresses_from_volume`.

19) **Enabled Coprocessor V2 API for input verification.**
    - Added `COPROCESSOR_PRIVATE_KEY` to `test-suite/fhevm/env/staging/.env.coprocessor` and `.env.coprocessor.local`.
    - Wired `--coprocessor-private-key=${COPROCESSOR_PRIVATE_KEY}` into `test-suite/fhevm/docker-compose/coprocessor-docker-compose.yml` for `coprocessor-gw-listener`.
    - This allows the `/v1/verify-input` endpoint to work for relayer polling.

20) **Removed U256 request_id overflow in coprocessor DB and workers.**
    - `gw-listener` was truncating request IDs (U256→i64) and panicking; coprocessor responses returned “Request not found”.
    - Added migration `coprocessor/fhevm-engine/db-migration/migrations/20260111180000_u256_request_ids.sql` to store request IDs as `BYTEA` and drop i64 checks.
    - Updated `gw-listener` and `zkproof-worker` to store/query 32-byte request IDs (U256 `to_be_bytes<32>()`) and log hex IDs.
    - Rebuilt coprocessor images and re-ran migrations.

21) **Fixed missing server key/CRS material in coprocessor after keygen.**
    - Added `--catchup-kms-generation-from-block=0` to `coprocessor-gw-listener` so it replays ActivateKey/ActivateCrs events.
    - Updated `AWS_ENDPOINT_URL` to `http://minio:9000` in `.env.coprocessor` and `.env.coprocessor.local`.
    - Restarted gw-listener and confirmed it pulled keys/CRS from MinIO and populated DB; restarted zkproof worker after keys loaded.

22) **Input-proof e2e test confirmed working.**
    - `./test-suite/fhevm/fhevm-cli test input-proof` succeeded after the above fixes.

23) **Restored relayer SDK compatibility for user/public decrypt (v1 endpoints).**
    - Relayer SDK v0.3.0-6 calls `/v1/user-decrypt` and `/v1/public-decrypt`; relayer only exposed v2 routes, resulting in 404s.
    - Added synchronous v1 handlers for user/public decrypt that wait on gateway responses and return `{status, response}`.
    - Injected default `contractAddresses` (zero address per handle) for v1 public decrypt requests so v2 registry calldata can be built.
    - Rebuilt `ghcr.io/zama-ai/console/relayer:local` and upgraded the relayer container.

24) **Fixed KMS ACL lookup using wrong chain id from user decryption requests.**
    - User decrypt e2e hung; kms-worker logs showed `No ACL configured for host chain 8064677207778538613`.
    - Root cause: kms-worker used the chain_id from the gateway event (derived from handle bytes), which didn’t match configured host chain 12345 in this stack.
    - Added a fallback in `kms-connector/crates/kms-worker/src/core/event_processor/acl.rs` to derive chain id from handle bytes (22..30) when the provided chain id isn’t configured.
    - Rebuilt `ghcr.io/zama-ai/fhevm/kms-connector/kms-worker:local` and upgraded the kms-connector service with `CONNECTOR_KMS_WORKER_VERSION=local`.

25) **KMS worker was crashing on host chain RPC config parsing.**
    - `kms-connector-kms-worker` exited at startup with `invalid type: map, expected a sequence for key host_chain_urls`.
    - Root cause: config expects a sequence, but env vars are loaded as a map (`KMS_CONNECTOR_HOST_CHAIN_URLS__0__*`).
    - Fix: changed `host_chain_urls` config type to a map and updated the kms-worker to iterate over map values.
    - Ensured `.env.kms-connector.local` uses `KMS_CONNECTOR_HOST_CHAIN_URLS__0__CHAIN_ID` and `KMS_CONNECTOR_HOST_CHAIN_URLS__0__RPC_URL`.

26) **User decryption ACL used relayer address instead of user address.**
    - DecryptionRegistry emitted `userAddress = msg.sender`, but relayer is the msg.sender for v2 flow.
    - Updated DecryptionRegistry interface and implementation to accept `userAddress` as an explicit parameter and emit it in events.
    - Updated DecryptionRegistry tests and regenerated gateway rust bindings; relayer calldata now includes user address.

27) **Gateway/relayer address plumbing after registry redeploy.**
    - Rebuilt/deployed gateway contracts and synced new proxy addresses to local env files.
    - Fixed relayer config to use `test-suite/fhevm/config/relayer/local.yaml` (not `.local`) so runtime picks up new addresses.
    - Restarted relayer/kms-connector/coprocessor to consume the updated addresses.

28) **User decryption requests were dropped due to request-id collisions after redeploy.**
    - `kms-connector-gw-listener` inserts `user_decryption_requests` with `ON CONFLICT DO NOTHING`.
    - Gateway registry counters reset after local redeploy, so new request IDs collided with existing rows.
    - Added a stack step to truncate KMS connector request/response tables on deploy to avoid collisions in local e2e runs.

29) **Coprocessor gw-listener ciphertext API was failing due to invalid column selection.**
    - `/v1/ciphertext/:handle` returned 500 and kms-worker logged `No coprocessor quorum reached`.
    - Root cause: query selected `ct.ciphertext_digest` from `ciphertexts` table, but the column does not exist.
    - Fixed by computing the digest from the fetched ciphertext (keccak) in `gw-listener` and rebuilt the local image.

30) **tfhe-worker crashed on tenant key deserialization (no ciphertexts).**
    - `tfhe-worker` panicked: `We can't deserialize our own validated sks key`, so no ciphertexts were produced.
    - Root cause: coprocessor services were on mixed versions (gw-listener built locally, tfhe-worker still on v0.10.5), causing key serialization mismatch.
    - Built a local `coprocessor/tfhe-worker` image from this repo and upgraded coprocessor with `COPROCESSOR_TFHE_WORKER_VERSION=local`.
    - Ciphertexts began populating in `coprocessor.ciphertexts`, and `/v1/ciphertext/:handle` returned `found`.

31) **KMS worker rejected coprocessor ciphertext signatures (EIP-712 domain mismatch).**
    - KMS logs showed `Coprocessor signature does not match configured signer`, stalling user decrypt.
    - Root cause: `KMS_CONNECTOR_INPUT_VERIFICATION_CONTRACT__DOMAIN_NAME` defaulted to `InputVerificationRegistry`, but coprocessor signs with domain name `FHEVM`.
    - Fixed by explicitly setting `KMS_CONNECTOR_INPUT_VERIFICATION_CONTRACT__DOMAIN_NAME=FHEVM` (and version `1`) in `.env.kms-connector` and `.env.kms-connector.local`, then upgrading kms-connector.

32) **Fixed: V2 ciphertext API serving wrong format to KMS (was misdiagnosed as tfhe-rs version incompatibility).**
    - User decryption tests failed with `Failed user decryption: On deserialization, expected type high_level_api::SquashedNoiseFheUint, got type high_level_api::CompressedCiphertextList`.
    - **Initial misdiagnosis:** Blamed kms-core tfhe-rs version incompatibility.
    - **Actual root cause:** V2 HTTP API (`/v1/ciphertext/:handle`) was serving ciphertexts from the wrong source:
      - V1 architecture: tfhe-worker → DB → SNS worker → `squash_noise_and_serialize()` → S3 (`ct128` bucket) → KMS fetches from S3
      - V2 HTTP API was reading from `ciphertexts.ciphertext` column (raw CompressedCiphertextList format) instead of fetching the SNS-processed ciphertext from S3
      - KMS expects SquashedNoise format, not CompressedCiphertextList
    - **Additional issue:** Even after fixing the source, KMS still failed with `expected SquashedNoiseFheBool, got CompressedSquashedNoiseCiphertextList` because:
      - SNS worker uses `--enable-compression` flag, producing `CompressedSquashedNoiseCiphertextList` (format 11=CompressedOnCpu)
      - V2 code was hardcoding `CiphertextFormat::BigExpanded` instead of using the actual format
    - **Fix applied:**
      1. Updated `gw-listener/src/api/handlers.rs` `fetch_ciphertext` to query `ciphertext_digest` table for ct128 digest and fetch from S3
      2. Added `ciphertext_format` field to `CiphertextResponse` API type
      3. Updated `kms-connector` `CoprocessorApi` to pass through `ciphertext_format`
      4. Updated `kms-connector` `build_typed_ciphertexts` to map format 11/21 (compressed) → `CiphertextFormat::BigCompressed`
    - **Result:** KMS worker backend is working correctly - logs show "Result successfully retrieved from KMS Core!" and "Event successfully processed!". Responses are being stored in the `kms-connector` database.
    - **However:** V1 synchronous endpoints still hang (see issue #33 below).

33) **Critical: V1 synchronous user-decrypt endpoints hang due to response retrieval bug (BLOCKER).**
    - User decryption tests hang indefinitely after backend processing succeeds.
    - **Investigation findings:**
      - KMS worker backend is working correctly: processes requests via GRPC and stores responses with `status='completed'` in `user_decryption_responses` table
      - KMS worker **does** expose `/v1/share/{request_id}` HTTP API endpoint (in `kms-worker/src/api/handlers.rs`)
      - API handler queries `user_decryption_responses` WHERE `decryption_id = $1 AND status = 'completed'` with correct little-endian conversion
      - Relayer is configured to poll `http://kms-connector-kms-worker:9100/v1/share/{request_id}`
      - Relayer logs show: `[V2] Polling workers for user decrypt request 0x02...` but no "Fetching share" debug logs
      - Manual curl tests to KMS worker API return `{"status":"not_found"}` even when response exists in DB with matching request ID
    - **Root cause:** Either:
      1. Bug in KMS worker API handler - SQL query not finding existing responses (endianness issue in query binding?)
      2. Relayer not actually making HTTP requests (old binary or polling logic not executing)
      3. Both relayer and KMS worker binaries are outdated compared to source code
    - **Evidence of version mismatch:**
      - Running relayer logs contain messages not found in current source code
      - KMS worker handler code looks correct but runtime behavior doesn't match
    - **Impact:** All user-decryption e2e tests using V1 SDK endpoints are blocked. V2 async flow (via events) works, but V1 compatibility layer is broken.
    - **Next steps for resolution:**
      1. Rebuild both relayer and kms-worker from latest source
      2. Add debug logging to KMS worker API handler to trace SQL query execution
      3. Verify relayer actually makes HTTP requests with correct debug logging
      4. Test with latest binaries
    - This is a critical blocker preventing Gateway V2 user decryption from being tested end-to-end with the V1 synchronous SDK.

34) **Coprocessor keys stayed empty due to stale env addresses after redeploy.**
    - Symptoms: user-decrypt hung with `No coprocessor quorum reached`; tfhe-worker panicked on SKS deserialization.
    - Root cause: coprocessor services were still running with old `KMS_GENERATION_ADDRESS` / `INPUT_VERIFICATION_ADDRESS`, so `ActivateKey`/`ActivateCrs` events were missed and tenant keys remained zeroed.
    - Fix: restarted coprocessor stack after address sync so it picked up the latest addresses; gw-listener then processed `ActivateKey`/`ActivateCrs` and populated `tenants.{sks_key,pks_key,public_params}`.

35) **User-decrypt hang due to tfhe-worker starting before keys were populated.**
    - `coprocessor-tfhe-worker` panicked: `We can't deserialize our own validated sks key` while `sks_key`/`pks_key` were still zero.
    - Cause: tfhe-worker booted and processed work items before gw-listener finished `ActivateKey`/`ActivateCrs` and filled tenant keys.
    - Fix: restart tfhe-worker (or `upgrade coprocessor`) after keys are populated so it reloads valid keys.
    - If it still panics after a restart with non‑zero keys, then investigate TFHE version mismatch (rebuild tfhe-worker locally).

36) **KMS worker API returns `not_found` even though responses exist (DB mismatch suspected).**
    - Observed: `kms-worker` logs show `Querying user_decryption_responses ...` followed by `Request not found`.
    - Verified: `kms-connector` DB contains completed rows for the same `decryption_id` (e.g., `0600...0002`).
    - Likely cause: API pool is connected to a different DB/schema than the writer, or DB URL is mis-resolved.
    - Next step: point `KMS_CONNECTOR_DATABASE_URL` (and `DATABASE_URL`) explicitly to `coprocessor-and-kms-db` with user/password, restart kms-connector, and re-test. If still failing, add logging of DB host/connection and dump recent IDs in the handler.

37) **User decryption failures traced to missing relayer database migrations (resolved issue #36).**
    - **Symptoms observed:**
      - User decryption tests hanging with "Request not found" errors from KMS worker API
      - Initial investigation in #36 suspected DB connection mismatch
      - KMS worker logs: `Request not found for request_id=0x0200...0006`
      - Relayer polling logs visible but no responses received
      - DB query confirmed responses exist with status='completed'
    - **Investigation timeline:**
      - Checked request ID endianness conversion (big-endian ↔ little-endian) - found to be correct by design
      - Verified KMS worker backend processing worked correctly (responses in DB with matching IDs)
      - Discovered KMS connector services restarted ~10 minutes after test started (at 15:07:59), creating availability gap
      - Found relayer startup logs showing critical errors at 14:27:00-14:27:10
    - **Root cause identified:**
      - Relayer was missing database tables: `gateway_block_number_store`, `user_decrypt_req`, etc.
      - Errors in logs: `relation "gateway_block_number_store" does not exist`, `relation "user_decrypt_req" does not exist`
      - Relayer retry loop failed after 20 attempts: `Gateway listener 0 failed: exceeded max reconnection attempts`
      - Cause: deployment script did not run relayer database migrations before starting relayer service
    - **Why this caused "Request not found":**
      - Without state tracking tables, relayer couldn't persist checkpoint data or manage request lifecycle
      - Relayer became unstable and eventually restarted
      - Even after switching to V2 (which doesn't need all those tables), system remained unstable from startup failures
      - KMS workers had processed requests correctly, but relayer couldn't coordinate polling properly
    - **Fix applied:**
      - Created `relayer-db-init` service in `test-suite/fhevm/docker-compose/relayer-docker-compose.yml` to create `relayer_db` database
      - Created `relayer-db-migration` service to run sqlx migrations from `/Users/msaug/zama/console/apps/relayer/relayer-migrate/migrations/`
      - Updated `relayer` service to depend on successful completion of both init and migration services
      - Modified `test-suite/fhevm/scripts/deploy-fhevm-stack.sh` to wait for:
        - `fhevm-relayer-db-init:complete`
        - `fhevm-relayer-db-migration:complete`
        - `fhevm-relayer:running`
      - Pattern matches existing services: `coprocessor-db-migration` and `kms-connector-db-migration`
    - **Files modified:**
      - `test-suite/fhevm/docker-compose/relayer-docker-compose.yml` - Added db-init and migration services with proper dependencies
      - `test-suite/fhevm/scripts/deploy-fhevm-stack.sh` - Updated RUN_COMPOSE call to wait for migration completion
    - **Verification steps (pending):**
      - `./test-suite/fhevm/fhevm-cli deploy --local` - Test full deployment with migrations
      - `docker logs fhevm-relayer-db-init` - Should show "Relayer database ready"
      - `docker logs fhevm-relayer-db-migration` - Should show "Migrations executed"
      - `docker logs fhevm-relayer` - Should NOT show "relation does not exist" errors
      - `docker exec coprocessor-and-kms-db psql -U postgres -d relayer_db -c "\dt"` - Should list all required tables
      - `./test-suite/fhevm/fhevm-cli test user-decryption` - Should pass without hanging
    - **Key learnings:**
      - Request ID endianness (big-endian hex ↔ little-endian bytes) is correct by design - don't assume it's a bug
      - Service restart logs are critical indicators - check container uptime when debugging availability issues
      - Deployment automation must handle database migrations for ALL services, not just core infrastructure
      - The `fhevm-cli` tool is now fully self-sufficient - no manual migration steps required

38) **Fixed relayer local build issues (sqlx verification, bindings path, migration config).**
    - **Symptoms:**
      - Build failed with `database "relayer_db" does not exist` (sqlx compile-time verification)
      - Build failed with `expected a tuple with 4 elements, found one with 5 elements` (bindings mismatch)
      - Build failed with `"/fhevm/gateway-contracts/rust_bindings": not found` (Docker mount path)
      - Migration failed with `MissingValue("max_attempts")` (missing env var)
    - **Root causes identified:**
      1. **sqlx online mode**: Dockerfile had `SQLX_OFFLINE=false` but DATABASE_URL pointed to `coprocessor-and-kms-db` which isn't resolvable during Docker build phase
      2. **Bindings mismatch**: Console repo had outdated bindings (4 params) vs fhevm repo (5 params with `userAddress`)
      3. **Docker mount path**: Build tried to mount from `fhevm/gateway-contracts/rust_bindings` but console repo's copy was deleted
      4. **Migration config**: `relayer-db-migration` service missing `MAX_ATTEMPTS` environment variable
    - **Fixes applied:**
      1. Changed `DATABASE_URL` in `build-relayer-local.sh` to use `host.docker.internal:5432` (accessible during Docker build)
      2. Updated deploy script to create `relayer_db` and run migrations BEFORE the build (so sqlx can verify against live schema)
      3. Copied updated bindings to `console/fhevm/gateway-contracts/rust_bindings/` and updated Dockerfile mount source
      4. Added `MAX_ATTEMPTS=1` to `relayer-db-migration` service in docker-compose
    - **Files modified:**
      - `relayer.local.Dockerfile` - Changed mount source to `console/fhevm/gateway-contracts/rust_bindings`
      - `test-suite/fhevm/scripts/build-relayer-local.sh` - Changed DATABASE_URL to `host.docker.internal`, added NO_CACHE option
      - `test-suite/fhevm/scripts/deploy-fhevm-stack.sh` - Added pre-build DB init and migration steps
      - `test-suite/fhevm/docker-compose/relayer-docker-compose.yml` - Added `MAX_ATTEMPTS=1` to migration service
    - **Key learnings:**
      - sqlx compile-time verification requires DB to be accessible during build - use `host.docker.internal` for port-mapped services
      - Docker bind mount sources are relative to build context, not working directory
      - Keep bindings copies in sync when using separate repos, or use a single source of truth
      - Migration services need same env vars as main service (e.g., `MAX_ATTEMPTS`)

39) **KMS worker API fails to find responses due to V2 status race condition and SQL type mismatch.**
    - **Symptoms:**
      - `curl http://localhost:9100/v1/share/...` returns `{"status": "not_found"}`
      - Direct DB query works: `SELECT ... WHERE decryption_id = ... AND status = 'completed'` returns the row
      - KMS worker logs: `Request not found for request_id=...` with no error messages
    - **Root causes identified (two separate issues):**
      1. **V2 status race condition**: Response INSERT used default status='pending', but API queries for status='completed'
         - In V1, tx-sender would update status to 'completed' after submitting the on-chain transaction
         - In V2, decryption responses are served via HTTP API (no on-chain tx), so responses were never marked completed
      2. **SQL type mismatch**: `EXTRACT(EPOCH FROM timestamp)` returns `NUMERIC` in PostgreSQL, but Rust code expected `f64`/`FLOAT8`
         - Error was silently swallowed: `if let Ok(Some(...))` ignored the `Err` case
         - Actual error: `mismatched types; Rust type 'f64' is not compatible with SQL type 'NUMERIC'`
    - **Fixes applied:**
      1. **Status fix** in `kms-connector/crates/kms-worker/src/core/kms_response_publisher.rs`:
         - Modified `publish_user_decryption` and `publish_public_decryption` to INSERT with `status = 'completed'`
         - Added comment: "V2: Set status='completed' on INSERT so responses are immediately queryable via HTTP API"
      2. **Type cast fix** in `kms-connector/crates/kms-worker/src/api/handlers.rs`:
         - Changed `EXTRACT(EPOCH FROM created_at) AS response_at` → `EXTRACT(EPOCH FROM created_at)::float8 AS response_at`
         - Applied to all 4 query locations (user_response, public_response, user_request_status, public_request_status)
      3. **Port mapping fix** in `test-suite/fhevm/docker-compose/kms-connector-docker-compose.yml`:
         - Added `ports: - "9100:9100"` to kms-worker service for V2 API access
    - **Files modified:**
      - `kms-connector/crates/kms-worker/src/core/kms_response_publisher.rs` - Set status='completed' on INSERT
      - `kms-connector/crates/kms-worker/src/api/handlers.rs` - Cast EXTRACT result to float8
      - `test-suite/fhevm/docker-compose/kms-connector-docker-compose.yml` - Expose port 9100
    - **Verification:**
      - After fix: `curl http://localhost:9100/v1/share/...` returns `{"status": "ready", "encryptedShare": "...", ...}`
      - E2E test now receives data (fails on SDK format parsing, not on missing data)
    - **Key learnings:**
      - When switching from on-chain to HTTP API (V1→V2), ensure all status transitions are still handled
      - PostgreSQL `EXTRACT(EPOCH FROM ...)` returns `NUMERIC`, not `FLOAT8` - always cast explicitly
      - Silent error handling (`if let Ok(...)`) can hide type mismatches - add error logging during debug
      - Add debug queries to verify DB connectivity when troubleshooting "not found" issues

40) **User decryption tests fail on SDK response format parsing (in progress).**
    - **Symptoms:**
      - Tests get data from KMS API but fail with: `invalid type: JsValue(Object({"result":[...]})), expected a sequence`
      - The `node-tkms` WASM library (`process_user_decryption_resp_from_js`) expects a different format
    - **Current state:**
      - KMS worker API is correctly returning: `{"status": "ready", "encryptedShare": "...", "signature": "...", ...}`
      - Relayer transforms this to: `{"result": [{"payload": "...", "signature": "..."}]}`
      - SDK expects just the array/sequence, not wrapped in `{"result": [...]}`
    - **Next steps:**
      - Investigate relayer response transformation logic
      - Check SDK expected format in `@zama-fhe/relayer-sdk` and `node-tkms`
      - Either fix relayer to return correct format OR update SDK to handle wrapper

41) **RESOLVED: Fixed V1 user-decrypt response format mismatch (resolves issue #40).**
    - **Root cause identified:**
      - SDK V1 Provider (`RelayerV1Provider.ts:89-92`) extracts `json.response` and expects it to be an array `[{payload, signature}]`
      - Relayer V1 handler was returning `response: UserDecryptResponseJson` which is `{"result": [...]}`
      - So `json.response` was `{"result": [...]}` instead of `[...]`
      - This got passed to `process_user_decryption_resp_from_js()` in node-tkms WASM, which expects a sequence (array)
      - Error: `invalid type: JsValue(Object({"result":[...]})), expected a sequence`
    - **Fix applied:**
      - Modified `console/apps/relayer/src/http/endpoints/v1/handlers/user_decrypt.rs:216-221`
      - Changed `response: response_json` to `response: response_json.result`
      - Added comment: "V1 SDK expects response to be the array directly, not wrapped in {"result": [...]}"
    - **Files modified:**
      - `console/apps/relayer/src/http/endpoints/v1/handlers/user_decrypt.rs`
    - **Verification:**
      - Rebuilt relayer with `./test-suite/fhevm/fhevm-cli deploy --build --local --resume relayer`
      - Ran `./test-suite/fhevm/fhevm-cli test user-decryption`
      - All 8 user decryption tests passed (ebool, euint8, euint16, euint32, euint64, euint128, eaddress, euint256)
    - **Key learnings:**
      - V1 SDK expects `json.response` to be the final array directly, not wrapped in an object
      - V2 SDK handles nested `json.result.result` extraction internally
      - Always trace the full data flow from API response → SDK extraction → WASM processing when debugging format issues

42) **Fixed V1 public-decrypt response format (camelCase vs snake_case field naming).**
    - **Symptoms:**
      - Public decrypt test failed with: `TypeError: Cannot read properties of undefined (reading 'startsWith')`
      - Error occurred in SDK's `ensure0x(result.decrypted_value)` call
      - User decryption tests passed (8/8), but public decryption failed
    - **Root cause:**
      - V1 public-decrypt handler reused V2's `PublicDecryptResponseJson` type
      - V2 type has `#[serde(rename_all = "camelCase")]`, serializing as `decryptedValue`, `extraData`
      - V1 SDK expects snake_case: `result.decrypted_value`, `result.extra_data`
      - When SDK accessed `result.decrypted_value`, it got `undefined` (field was named `decryptedValue`)
    - **Fix applied:**
      - Created V1-specific response type `PublicDecryptResponseJsonV1` with snake_case field names
      - Updated V1 handler to use the new type instead of V2's camelCase type
    - **Files modified:**
      - `console/apps/relayer/src/http/endpoints/v1/handlers/public_decrypt.rs`
    - **Key learnings:**
      - V1 and V2 have different JSON field naming conventions (snake_case vs camelCase)
      - V1 handlers cannot simply reuse V2 response types for serialization
      - Each API version should have its own response types to avoid naming conflicts

---

## API Versioning Clarification (V1 vs V2)

The relayer exposes two API versions with different patterns:

| Aspect | V1 (`/v1/*`) | V2 (`/v2/*`) |
|--------|--------------|--------------|
| **Pattern** | Synchronous | Async polling |
| **POST behavior** | Waits and returns final result | Returns `job_id` immediately |
| **GET behavior** | N/A | Poll `/{job_id}` for status/result |
| **Field naming** | `snake_case` | `camelCase` |
| **SDK class** | `RelayerV1Provider` | `RelayerV2Provider` |

**SDK version selection:**
```typescript
createRelayerProvider(relayerUrl, defaultRelayerVersion: 1 | 2)
```
- URL suffix (`/v1` or `/v2`) overrides default
- E2E tests currently use V1 (synchronous, simpler for testing)
- V2 exists for production (no long-held connections)

**Common pitfall:** V1 handlers were initially reusing V2 internal types, which have `camelCase` serialization via `#[serde(rename_all = "camelCase")]`. This caused field naming mismatches. V1 handlers need their own response types with `snake_case` fields.
