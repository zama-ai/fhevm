# fhevm-cli redesign — exemplar findings

This document records the chart-on-kind work — from the offline render (§1–§2) through a
**full live boot to a finalized FHE keygen (§0 below, 2026-06-17 — the authoritative
current state)**. It maps every scaffolded file to the design idea it demonstrates, and
captures the **proven boot recipe + invariants** that `stack/lib` must encode.

---

## 0. Full stack boot through FHE keygen — PROVEN on kind (2026-06-17)

A hand-driven boot took the stack **from an empty kind cluster to a finalized FHE
keygen** — the hardest, most load-bearing part of the protocol — validating the
charts-on-kind + staging-env approach end to end. Everything here was run live and
verified, not inferred. Cluster: `fhevm-p2`.

### Live state

| Layer | Status |
|---|---|
| anvil host (chainId 12345) + gateway (54321) + `host-node`/`gateway-node` alias svcs | Running, RPC verified |
| postgres `db` (+ `coprocessor`, `kms-connector` databases) | Running |
| minio (buckets `kms-public`/`ct64`/`ct128`, user `fhevm-access-key`) | Running |
| gateway + host contracts | deployed; **deterministic addresses match staging exactly** |
| `sc-addresses` ConfigMap | published |
| kms-core (centralized, `core-service:v0.13.20-0`, signing key on a PVC) | Running on :50051 |
| kms-connector (gw-listener / kms-worker / tx-sender + migration) | Running, polling host KMSGeneration |
| **FHE keygen** | **FINALIZED** — `activeKeyId` set; `PublicKey` 32 KiB + `ServerKey` 1.6 GiB + `CRS` 4.4 MiB in minio; realized `FHE_KEY_ID = 0400…0001` |
| coprocessor (7 svcs) → relayer → `erc20` + L2 golden | **DONE — `fhevm up` boots genesis→erc20 self-contained; 11/11 green** |

This supersedes the "still open: full multi-chart boot" note in §0a below — that boot is
now done, through keygen.

### PHASE 2 COMPLETE — full default-case equivalence, recipe-driven (2026-06-19)

`npx tsx stack/cli/main.ts up` (ONE command) now boots an empty kind cluster through all
15 RECIPE phases — cluster → chains → data-plane → kms-core → kms-signer → gateway-deploy →
host-deploy → gateway-wire → fund-tx-sender → coprocessor → kms-connector → trigger-keygen →
await-keygen → relayer → **erc20** — and the erc20 e2e passes **11/11** (mint, transfer×2,
transferFrom, allowance, reject-not-allowed, should-not-transfer, + all HCU block-cap tests:
accumulate-until-cap, block-rollover, whitelist-removal, reject-from-non-owner). Proven across
5 consecutive clean genesis boots. The finish-line Phase 2 gate — *"up(default) reaches Ready;
L2 erc20 green; L0 default matches golden"* — is met, plus:

- **Full diff test GREEN**: L0 `default` (25 docs) + L0 `two-of-three` (49 docs) match golden,
  distinct shas (topologies genuinely diverge); L2 `erc20` golden recorded (11 passing) in
  `stack/.migration/golden/L2-erc20/`. typecheck clean on all stack TS.
- **Self-contained**: the erc20 phase applies `erc20.yaml` (envFrom `erc20-env`) and gates on
  the Job; `render.threadDiscovery` populates `erc20-env` with the discovered host+gateway
  addresses — no hand-set test env. Same discover→regenerate path as every other consumer.
- Thin driver ≈ 1,460 non-comment LoC vs the old CLI's ~12,850 (~8.8×).

REMAINING (Phases 3–7, each a substantial effort): **Phase 3** two-of-three + threshold-KMS
L2 behavioral (3 kms-core MPC parties + 2-of-3 consensus + 3 coprocessor releases — the
recipe is currently centralized/single; the `--kms threshold:N` flag is parsed but no phase
branches on topology yet); **Phase 4** procedures→runbooks + L3 upgrade receipts + chaos;
**Phase 5** delete `resolve/`+`compat/` (manifest as sole version mechanism); **Phase 6** CI
cutover (acceptance.yml step bodies are still PLACEHOLDERs); **Phase 7** relocate
`test-suite/fhevm` → top-level `stack/`.

### TIER 1 — kill the log-scraping regression (2026-06-27)

The original recipe scraped deploy-pod logs to learn contract addresses — the exemplar was
*more* fragile here than the real CLI, which reads structured `.env` artifacts. Addresses are
deterministic for a fixed deployer + deploy order + genesis, so they are now **static
constants** (`STATIC_GATEWAY_ADDRESSES` / `STATIC_HOST_ADDRESSES`), and the KMS signer is read
from minio's structured `VerfAddress` artifact (log scrape kept only as a fallback). This
collapses the discover surface from ~12 values to ~1 irreducible one (the KMS signing key,
which has no seed flag) and is the prerequisite for parametrizing topology — see the parity
ladder: Tier 1 (static) → **Tier 2** (CREATE2, order-independent addresses) → topology as data.

Also fixed: `envs.yaml` carried a stale gateway `INPUT_VERIFICATION_ADDRESS` used as the host
InputVerifier's cross-chain EIP712 `verifyingContractSource` — a stale value there is the
`InvalidSigner` revert. `render.threadDiscovery` now threads the live value into host-sc-env,
coprocessor-env, relayer-env, and erc20-env alike. Driver unit tests (`bun test stack/lib`,
18/18) are now hermetic — they no longer shell out to a live kubeconfig.

### Full-suite behavioral oracle (L2, ≤64-bit)

`stack/manifests/e2e.yaml` runs the *entire* `test-suite/e2e` (encryptedERC20 + the
fhevmOperations matrix + delegatedUserDecryption + consensusWatchdog + fhevmjsTest) against
the booted stack — the comprehensive behavioral oracle that supersedes the erc20-only smoke.
Two **env-gated, default-off** filters skip the heavy >64-bit ops (euint128/256, ebytes,
eaddress) to keep the run ≈1h on a CPU/kind stack: `MOCHA_GREP`/`MOCHA_INVERT` (runtime title
filter, in `hardhat.config.ts`) and `E2E_MAX_BITS` (codegen-time, in `library-solidity/codegen`).
Both default to no-op, so canonical behavior and full-coverage codegen are unchanged when the
vars are unset; the ≤64-bit *regenerated corpus* is deliberately a local artifact, never
committed, so the shared suite keeps its full width.

### Proven boot recipe (the spec for `stack/lib`)

Versions (v0.13 train): `GATEWAY_VERSION`/`HOST_VERSION` = `v0.13.0-6`, `CORE_VERSION` =
`v0.13.20-0`, `COPROCESSOR_*`/`CONNECTOR_*`/`LISTENER_CORE_VERSION` = `v0.13.0-6`,
`RELAYER_SDK_VERSION` = `0.4.2`. Private ghcr images via a `registry-credentials` secret
(`read:packages` token).

1. kind cluster + `registry-credentials` secret.
2. Chains: 2 `anvil-node` releases. **host** chainId 12345, mnemonic `adapt mosquito move
   limb mobile illegal tree voyage juice mosquito burger raise father hope layer`, port
   8545. **gateway** chainId 54321, mnemonic `coyote sketch defense hover finger envelope
   celery urge panther venue verb cheese`, port 8546. Add alias Services
   `host-node`/`gateway-node` (the chart names its service `<release>-anvil-node`; the
   stack expects `host-node:8545` / `gateway-node:8546`). Anvil **requires** a mnemonic
   (empty → `anvil --mnemonic ""` exit 1).
3. postgres `db` (databases `coprocessor` + `kms-connector`) · minio (buckets + user +
   `/minio_secrets/{access_key,secret_key}` files for kms-core).
4. kms-core up: config = `templates/config/kms-core-modern.toml` (NOT the legacy
   `config/kms-core/config.toml` — its `storage_cache_size` is rejected by v0.13.20-0).
   Persist `/app/kms/core/service/keys` on a PVC so the signing key is stable.
5. **Discover** kms-core's signer = the address in its own log
   (`stored … ethereum address 0x…`). Do **not** trust minio `VerfAddress` (goes stale).
6. Deploy gateway contracts: `gateway-contracts:v0.13.0-6`, RPC=`gateway-node:8546`,
   env=`.env.gateway-sc` (deployer derived from the gateway "coyote sketch" mnemonic).
7. Deploy host contracts: `host-contracts:v0.13.0-6`, RPC=`host-node:8545`,
   env=`.env.host-sc`, **`KMS_SIGNER_ADDRESS_0` = discovered signer (step 5)**,
   `KMS_TX_SENDER_ADDRESS_0=0x31de9c8a…`. KMSGeneration is a **host** contract in v0.13.
   Write the deploy's `/app/addresses` to a shared volume (the trigger needs it).
8. Publish `sc-addresses` (deterministic from deployer+nonce — a fresh chain reproduces
   them: ACL `0x05fD9B…`, host KMSGeneration `0x3E0fBCcE…`).
9. **Fund** the connector tx-sender (`0x31de9c8a…`) on host + gateway.
10. kms-connector (4 svcs): `KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS` = the host
    KMSGeneration `0x3E0fBCcE…`, watched over `KMS_CONNECTOR_ETHEREUM_URL=host-node:8545`
    (NOT the staging gateway value `0x3576…`). Wait for the gw-listener log
    `Started KMSGeneration polling from block N`.
11. Trigger keygen+crsgen **host-side**: `task:triggerKeygen --params-type 0
    --use-internal-proxy-address true` then `task:triggerCrsgen --params-type 0
    --max-bit-length 2048 --use-internal-proxy-address true`, with the deploy's
    `/app/addresses` mounted (else `HH404 FHEVMHostAddresses.sol`).
12. Wait `activeKeyId ≠ 0` on the host KMSGeneration **and** `PublicKey`/`ServerKey`/`CRS`
    in minio → the realized `FHE_KEY_ID` is discovered here; feed it forward.
13. coprocessor (db-migration + 9 svcs): `AWS_ENDPOINT_URL=http://minio:9000` (staging
    uses a compose IP), expand `${VAR}` in `DATABASE_URL`, `FHE_KEY_ID` = realized id.
14. relayer (`.env.relayer`).
15. `erc20` e2e from test-suite → PASS → record the L2 golden.

### Five invariants (each hit live — encode as guardrails)

1. **KMSGeneration is host-side in v0.13** (gateway KMSGeneration is read-only); trigger
   on host, connector watches host over `ETHEREUM_URL`.
2. **Discover the live signer; never hardcode.** `kms-gen-keys` won't overwrite an
   existing minio object, so `VerfAddress` goes stale across pod restarts. Staging `.env`
   constants (signer, KMSGen addr, FHE key id) are **seeds overwritten at runtime** — the
   CLI's `generateRuntime` placeholder-overwrite pattern.
3. **Order: deploy → connector → trigger.** Never reset a chain under a running
   connector — its DB `last_block_polled` desyncs from the fresh chain.
4. The keygen **tx-sender must be funded** on the host chain **and** its on-chain
   registered signer must equal kms-core's actual signer (else `KmsSignerDoesNotMatchTxSender`).
5. The connector must be **polling before** the trigger fires (no backfill); keygen is
   **two-phase** (prep → keygen) — the prep response must land on-chain to emit the real
   `KeygenRequest`.

**How the CLI does steps 5–7 cleanly** (the pattern `stack/lib` must encode): the pipeline
order is `base → kms-signer → gateway-deploy → host-deploy` (`src/types.ts`);
`discoverKmsSigners` scrapes the signing-key handle from kms-core's logs and reads its
`VerfAddress` from minio (`src/flow/readiness.ts:270`); `generateRuntime` then injects
that address into the deploy env as `KMS_SIGNER_ADDRESS_x` (`src/generate/env.ts:150`). The
hand-boot above inverts this (deploy-then-discover), which is exactly the bug to avoid.

### Coprocessor bring-up — PROVEN live (2026-06-17), with three new load-bearing findings

Step 13 was driven live on `fhevm-p2`: the coprocessor (db-migration + 7 service Deployments
— host-listener, host-listener-poller, gw-listener, tfhe-worker, zkproof-worker, sns-worker,
transaction-sender; **the redis-only `host-listener-consumer` is omitted** unless listener-core
runs) hand-rolled as raw manifests at `v0.13.0-6` (same pattern as kms-connector — only
anvil-node uses Helm on this cluster). db-migration applies the full schema; the 7 services
reach Running and the FHE key + CRS land in the coprocessor `keys`/`crs` tables. Env deltas vs
`.env.coprocessor` confirmed live: `AWS_ENDPOINT_URL→http://minio:9000`, `DATABASE_URL`
expanded, `FHE_KEY_ID`/KMS URLs → realized ids, `KMS_GENERATION_ADDRESS` → **host** KMSGeneration
`0x3E0fBCcE…` (the host-listener watches the host chain). `host_chains` must be seeded
(`chain_id, name, acl_contract_address`) — db-migration alone left it empty here.

Three findings `stack/lib` MUST encode (each cost real debugging):

1. **The host-listener only learns the key from the on-chain `ActivateKey`/`ActivateCrs`
   event** (it does not read `activeKeyId` directly). A listener started *after* keygen sits at
   the chain tip and never backfills → keys never download. Fix: `--start-at-block=1` (or before
   the keygen block) to re-scan history; the CLI avoids this by ordering coprocessor **before**
   the keygen trigger (same as connector invariant #5). This is the §0 ordering inversion again.

2. **minio never overwrites existing objects → on-chain digest can mismatch stale bytes.**
   Our first "finalized" keygen (`…0001`) was unusable: kms-core logged
   `"…already exists. Keeping the data without overwriting"` (CRS) and the on-chain digest from
   the successful run didn't match the older minio bytes, so the coprocessor rejected it
   (`Invalid Key/CRS digest`). Nothing had consumed the key before, so it went unnoticed — the
   coprocessor is the **first digest-validating consumer**. Versions are correctly paired
   (`versions.ts` `to`: kms-core `v0.13.20-0` ↔ coprocessor `v0.13.0-6`), so it was NOT a skew.
   Clean fix: keygen with **no pre-existing minio key objects** (fresh `key_id` via a new
   trigger advances the counter → `…0002`, stored fresh, digest matches). Encode: keygen must run
   against an empty public vault, or be the sole writer of each `key_id`.

3. **`host-listener:v0.13.0-6` hardcodes `minio:9000` → `172.17.0.1:9000`** in its S3 download
   path (`coprocessor/fhevm-engine/host-listener/src/kms_generation/aws_s3.rs:142` `split_url`,
   comment `// TODO: replace by docker configuration`) — a docker-compose bridge assumption.
   `AWS_ENDPOINT_URL` is ignored on this path. On kind that IP isn't minio, so key downloads fail
   `not found … at http://172.17.0.1:9000`. Two clean options: (a) advertise the storage URL
   under a host that does NOT contain the literal substring `minio:9000` (it's the contract param
   `KMS_NODE_STORAGE_URL_0`, `.env.host-sc:39` — e.g. the FQDN `minio.fhevm.svc…:9000`), so the
   hack is skipped; or (b) DNAT `172.17.0.1:9000 → minio` in the listener pod netns (privileged
   initContainer, the istio-init pattern) — what unblocked the live boot. The redesign should
   prefer (a) and flag the image bug. (Minor wrinkle: the coprocessor caches "latest key" by max
   `sequence_number`, so a leftover stale `key_id` row can shadow the on-chain active key — keep
   exactly one key per boot.)

### Relayer + erc20 e2e — live up to the key-consistency wall (2026-06-17)

The relayer (`relayer` + `relayer-migrate` + `relayer_db`) was hand-rolled at `v0.13.0-6`
and runs healthy (`Server listening on :3000`, `All servers ready`). `relayer-migrate`
`/bin/server` only runs the DB migration and exits — it is NOT the `:3001` key-server the
template's `keyurl` assumes; scale it to 0 after migrating. The erc20 e2e (`test-suite/e2e`,
hardhat `staging` network, relayer-sdk) was driven from the host against port-forwards and
reached the encrypt step. Findings for the harness + the **final blocker**:

4. **erc20 client packaging gotchas:** the harness statically imports `@fhevm/sdk` (declared
   `"*"`), whose workspace build ships no `_cjs` → CJS `require` fails at load even though the
   relayer-sdk path never uses it (lazy-import it, or build the test-suite image properly).
   And the relayer-sdk MUST be pinned to **0.4.2** (`^0.5.0-alpha.2` resolves to 0.5.0-alpha.2,
   which GETs a keyurl path the v0.13.0-6 relayer 404s; 0.4.2 matches `GET /v2/keyurl` → 200).

5. **THE KEY-CONSISTENCY WALL (the reason erc20 isn't green on this cluster).** The relayer's
   advertised key URL is **sourced on-chain (gateway side)**, NOT from its env/config — so the
   `APP_KEYURL`/yaml overrides are ignored, and `GET /v2/keyurl` returns whatever the gateway
   holds. Two consequences: (a) the advertised host is the on-chain `minio:9000` (in-cluster
   only) → the erc20 client must run **in-cluster** (where `minio:9000` resolves), not from the
   host. (b) Our host-only re-keygen (finding 2) advanced **host + coprocessor to `…0002`** but
   left **gateway + relayer on `…0001`** (whose minio bytes are the stale ones) — a host/gateway
   **active-key divergence**. The erc20 flow spans both chains and needs ONE consistent key.
   Update (later live check): the gateway has **no KMSGeneration code** at the staging
   `0x3576…` (`cast call` → "does not have any code") — the v0.13 migration (PR #2395) made the
   gateway KMSGeneration **view-only** and moved keygen host-side.
   **RESOLVED (source analysis):** there is **NO automatic host→gateway key relay** in v0.13.
   The relayer's `GET /v2/keyurl` is **config-driven** — `relayer/src/gateway/keyurl_handler.rs`
   `load_key_data()` reads the `keyurl` section from the relayer config (with a TODO to load it
   dynamically later), NOT from a gateway contract. Host-side `ActivateKey` is consumed by the
   coprocessor host-listener (host chain); the **relayer is fed the active key by configuration**.
   The gateway's `GatewayConfig` separately stores each KMS node's `storageUrl` (`Structs.sol`
   `KmsNode.storageUrl`, set at deploy / `updateKmsContext`) for the decryption path. So the
   "divergence" was mundane: the re-keygen advanced the host to `…0002` while the relayer config
   still pointed at `…0001` — the §0 seed-overwrite pattern applied to the relayer keyurl
   (confirmed live: after recovery with the keyurl config → `…0002`, the relayer advertised
   `…0002`, matching host+coprocessor). **Recipe consequence:** `await-keygen` discovers the
   realized key id; the `relayer` phase MUST thread that id into the relayer `keyurl` config
   (and the coprocessor env) — no cross-chain relay step is needed. The key-consistency wall is
   therefore a config-threading task, not a research unknown.

6. **The hand-assembled cluster DECAYS — don't treat it as a stable oracle.** A later check
   found the relayer crash-looping on `database "relayer_db" does not exist`: the `db` pod had
   restarted and lost the imperatively-`psql`-created database. This is why the data layer needs
   an explicit **`db-init`** step (now in `stack/manifests/setup.yaml`) and why the proven
   manifests had to be **persisted out of the live cluster** (`stack/manifests/`) before more
   decay — and it reconfirms erc20-green needs a **clean from-scratch boot**, not in-place
   repair of this decaying cluster.
   **Verified live (2026-06-17):** `setup.yaml`'s `db-init` was applied and **succeeded** —
   creating `coprocessor`/`kms-connector`/`relayer_db` and recovering the relayer
   ("Subscription active … Recovery complete"). Testing it caught a real bug (the db is named
   `relayer_db`, not `relayer` — names must match each service's `DATABASE_URL` exactly).
   It also proved a **recipe ordering** requirement: **db-init → service-migration → service**
   — a freshly-created DB is empty, so the relayer errored on `relation … does not exist` until
   `relayer-migrate` was re-run on it. The recipe's data-plane/relayer phases must sequence
   db-init before the migration before the service (the migration is NOT a one-shot to skip on
   a fresh DB). **Re-proved a second time:** a later probe found the coprocessor DB had ALSO
   lost its schema (`relation "keys" does not exist`) → all 7 coprocessor pods CrashLoopBackOff
   and the registered key `…0002` gone — same pattern (fresh/empty DB needs its migration before
   the service). By then the decay was extensive enough (coprocessor schema lost, key
   registration gone, ~8 pods crashlooping) that incremental recovery = a full coprocessor
   re-boot + re-keygen. **Conclusion stands, now decisively: erc20-green needs a clean
   from-scratch boot.** The recovered relayer advertising `…0002` is config-level only — the
   coprocessor no longer holds that key — so the cluster is NOT a functional stack to test against.
   **Final confirmation (attempted coprocessor recovery):** re-ran the coprocessor db-migration
   (schema rebuilt, 22 tables) and seeded `host_chains`, but then found **minio's `kms-public`
   bucket EMPTY** — minio had restarted (RESTARTS 1) and **lost ALL FHE key material**
   (`…0002` PublicKey/ServerKey/CRS gone), while the host chain still reports `activeKeyId=…0002`.
   So the host-listener can no longer re-download/register the key; recovery now requires a full
   re-keygen, i.e., the from-scratch boot. This is the definitive proof the cluster is
   unrecoverable for erc20. **Recipe requirement surfaced: minio MUST have persistent storage
   (a PVC)** — without it a restart wipes the FHE keys and breaks the whole stack (add a minio
   PVC to `data-plane.yaml`; kms-core keys already persist on the `kms-keys` PVC).

**Conclusion for `stack/lib`:** erc20-green requires a **clean from-scratch boot**, not patching
this cruft-accumulated cluster. The recipe must: (1) start kms-core against an **empty public
vault** so the first keygen is the sole writer of each `key_id` (no stale-overwrite); (2) bring
the coprocessor host-listener up **before** triggering keygen (no backfill); (3) trigger keygen
**once**, and ensure the activation propagates to **both** host and gateway so coprocessor and
relayer agree; (4) run the erc20 client **in-cluster** (service DNS), pinning relayer-sdk 0.4.2.
The full multi-chart boot through coprocessor + relayer is otherwise PROVEN live; only this
single-consistent-keygen + in-cluster-client step remains for the behavioral L2 golden.

### BREAKTHROUGH — single-consistent-keygen achieved end-to-end (2026-06-18)

The key-consistency wall is **passed live**: a fresh keygen produced FHE key `…0001`, it is
**registered in the coprocessor** (`keys` table: pks+sks present, `kms_key_activation_events`
+ `kms_crs_activation_events` = `activated`), and `…0001` is threaded into the relayer `keyurl`
config + `coprocessor-env`. The full stack (kms-core, both anvils, 7 coprocessor svcs, 3
connector svcs, relayer, db, minio) runs healthy against ONE consistent key. Findings proven
en route (all are recipe requirements):

7. **postgres needs a PVC** (added to `data-plane.yaml`, `Recreate` strategy + `PGDATA`
   subdir). Same persistence class as minio/anvil — postgres was ephemeral, so a `db` restart
   dropped every database (`coprocessor`/`kms-connector`/`relayer_db`) and cascaded failures.
   Proven live: a Docker-daemon wedge restarted every stateful pod at once.
8. **anvil needs `--state`** (added to `charts/anvil-node` statefulset: `--state <pvc>/state.json
   --state-interval 5`). Without it a pod restart resets the chain to genesis and **all deployed
   contracts vanish** — proven live (block→low, `eth_getCode` = `0x`). The chain-data PVC alone
   is useless unless anvil is told to load/dump state.
9. **kms-core's signing key is NON-deterministic** — `kms-gen-keys centralized` has no seed/
   deterministic flag, so each vault (re)generation yields a fresh signer address. Therefore the
   on-chain KMS signer (`KMS_SIGNER_ADDRESS_0` in host-sc-env) **must be DISCOVERED from kms-core
   at boot and threaded in before host-deploy** — hardcoding it breaks after any vault reset with
   `KmsSignerDoesNotMatchTxSender` (the on-chain response reverts). The tx-sender address is
   separate and stable (connector config).
10. **minio-dnat must resolve minio's IP at runtime.** The `host-listener:v0.13.0-6` rewrites
    `minio:9000`→`172.17.0.1:9000` (compose docker-bridge assumption, `aws_s3.rs`); the kind
    workaround DNATs that back to minio. A HARDCODED target IP silently breaks when minio's pod
    is recreated (PVC change) — the initContainer now does `getent hosts minio` (resolves the
    stable ClusterIP). Without it: "Failed to download key from all S3 buckets" → key never
    registers (activation stuck `pending`).
11. **Operational EOAs need funding on a fresh host chain.** After an anvil reset/redeploy the
    KMS tx-sender (`KMS_TX_SENDER_ADDRESS_0`) has 0 ETH → response submission fails with
    "Insufficient funds for gas". The recipe must fund the KMS tx-sender (and coprocessor signer)
    post-deploy (`anvil_setBalance` on the dev chain).
12. **Chain reset ⇒ coordinated connector state.** The connector polls KMSGeneration **from the
    current head** (no DB cursor to rewind), and keygen request-ids are **deterministic** (so any
    partial run leaves a "Duplicate request ID in meta store" in kms-core's vault + a stale row in
    the connector keygen tables). Net recipe rule: bring the connector up against a **fresh low
    head**, ensure kms-core vault + connector keygen tables are clean, then trigger keygen **once,
    live** — exactly the §0 "trigger once / host-listener up first / empty vault" requirements.
13. **Recorded Jobs must drop their `controller-uid`/`selector` labels** to be re-runnable after
    `kubectl delete job` (the recorded UID is rejected on re-create) — fixed in `minio-init`,
    `kmsconn-migration`.

### THE design principle (learned 2026-06-18, the keystone for the thin rewrite)

The fhevm-cli `up` is a flat **16-step ordered list** (`src/types.ts` STEP_NAMES):
`preflight → resolve → generate → base → kms-signer → gateway-deploy → host-deploy →
discover → regenerate → validate → coprocessor → kms-connector → bootstrap → relayer →
test-suite`. The recipe is **data** (this list), not imperative code.

**The one principle that collapses the 10K LoC: `discover → regenerate`. NEVER hardcode a
contract address (or signer, or key id) downstream of deploy.** Each deploy writes its actual
addresses (host-contracts → FHEVMHostAddresses.sol + .env on the host-addr volume; gateway →
GatewayAddresses.sol + .env on the gateway-addr volume). A `discover` step reads them; a
`regenerate` step threads them into every consumer (service envs, the relayer keyurl, the
mocked-payment approve, the e2e env). Proven the hard way: gateway proxy addresses are
**deploy-order-dependent** (deploying mocked-ZamaOFT first — same DEPLOYER nonce space — shifts
InputVerification 0x3b12→0x3576 and ProtocolPayment 0x1ceF→0x3b12). The env *templates* hardcode
mutually-inconsistent placeholders; the CLI overwrites them all from discovery. Every hour of
address-mismatch pain in this session was a violation of this principle.

Corollaries (each one line in the clean recipe; each a disaster when improvised):
- **kms-signer is DISCOVERED** from minio `VerfAddress` (step `kms-signer`, before any deploy) and
  threaded into the deploy envs — `kms-gen-keys` is non-deterministic, so it can't be hardcoded.
- **mocked-payment is mandatory wiring**, not optional: the gateway charges for input-proof
  verification (INPUT_VERIFICATION_PRICE), so `deployMockedZamaOFT` + mint + **approve the
  DISCOVERED ProtocolPayment** for the relayer's gateway tx-sender, else input-proof reverts.
- **a chain reset must reset EVERY listener's cursor for that chain** — connector
  `last_block_polled` (host KMSGeneration events AND gateway decryption events) AND coprocessor
  `gw_listener_last_block.last_block_num` — or a stale cursor ("last processed is ahead of current
  block, skipping") silently stalls the whole flow. The clean answer: **boot once from genesis,
  never reset a chain mid-flight.**
- gateway-sc / host-sc setup containers **share an addresses volume** (`/app/addresses`) so deploy's
  generated *.sol is visible to add-network / add-pausers (which compile contracts importing it).
- operational EOAs are mnemonic-prefunded (host: "adapt mosquito…", gateway: "coyote sketch…");
  only the kms-connector tx-sender needs explicit `anvil_setBalance` (a gateway key on the host chain).

**Conclusion: erc20-green needs a single clean genesis boot through this recipe** — the live cluster
in this session is a frankenstein from ~15 hand resets (every reset stranded another cursor/address),
which is precisely the imperative whack-a-mole the thin declarative driver exists to replace. The POC
proved the path end-to-end (keys → encryption → mint → input-proof submission); the residual failures
are all stale-cursor/stale-address decay, not stack bugs.

Remaining for the L2 golden: run the erc20 client **in-cluster** (service DNS).

14. **Use `@fhevm/sdk` (js-sdk), NOT `@zama-fhe/relayer-sdk` — supersedes finding 4's pin advice.**
    `@zama-fhe/relayer-sdk` is deprecated; its rewrite is `@fhevm/sdk` (repo: `sdk/js-sdk/src`,
    migration guide `sdk/js-sdk/docs/migration.md`). The e2e harness already supports both via a
    dual-SDK abstraction (`test/sdk/{fhevm-sdk,relayer-sdk}/`) gated on `RELAYER_SDK_VERSION`:
    **unset/empty → `@fhevm/sdk` (the default)**; set → old relayer-sdk. js-sdk **avoids both
    finding-4 gotchas**: it fetches the key from **`GET /v2/keyurl`** (`fetchFheEncryptionKeySource
    .ts:35`) — the path the v0.13.0-6 relayer serves 200 (no 0.4.2 pin) — and it **ships `_cjs`**
    (`sdk/js-sdk/src/package.json`, so no CJS-require break; the "no _cjs" note was pre-build).
    So: build `test-suite/e2e` Dockerfile with `RELAYER_SDK_VERSION=""` (builds + uses js-sdk),
    run as an in-cluster Job with the `staging` env → `RPC_URL=http://host-node:8545`,
    `RELAYER_URL=http://relayer:3000`, gateway at `gateway-node:8546`, and our deployed addresses.
    js-sdk config shape: `defineFhevmChain({ id: chainId, fhevm: { contracts: {acl, inputVerifier,
    kmsVerifier}, relayerUrl, gateway: {id, contracts:{decryption, inputVerification}} } })`.

15. **`AWS_ENDPOINT_URL` for the coprocessor S3 clients MUST be an IP, never the `minio`
    hostname — this was the last wall before erc20 user-decryption.** The captured staging
    `coprocessor-env` had `AWS_ENDPOINT_URL=http://minio:9000`. The sns-worker's AWS Rust SDK
    treats a *hostname* endpoint as virtual-hosted-style and prepends the bucket →
    `ct128.minio:9000`, which does NOT resolve in cluster DNS → `Failed to check bucket
    existence: dispatch failure`, so ct128 is never uploaded → the ciphertext never reaches the
    gateway → the relayer's user-decrypt readiness check times out (the 300s erc20 hang). An *IP*
    endpoint forces **path-style** (`10.96.x:9000/ct128`), which works. The working compose used
    `AWS_ENDPOINT_URL=http://172.18.0.2:9000` (the docker-bridge IP) for exactly this reason — the
    IP, not the value, is load-bearing. Clean fix (encoded): the `coprocessor` phase **discovers
    minio's ClusterIP at boot** (`Stack.serviceClusterIP("minio")`) and threads it into
    `coprocessor-env.AWS_ENDPOINT_URL` — same discover→regenerate principle, never hardcoded (the
    ClusterIP changes when minio is recreated). Confirmed live: sns-worker → `Bucket exists`, ct128
    objects appear in minio. NOTE this is the **upload** path (standard SDK, AWS_ENDPOINT_URL);
    the host-listener **download** path is separate (finding 10's `minio:9000`→`172.17.0.1`
    rewrite + DNAT) and unaffected by this change.

16. **`anvil --state` corrupts state.json → host-chain crashloop. Drop persistence entirely.**
    anvil dumps `state.json` **non-atomically** (in place), so ANY interrupted write truncates it,
    and on restart anvil refuses to boot (`invalid value … EOF while parsing`). On the next start
    the host chain is down (`host-node` ClusterIP `ECONNREFUSED`) and erc20 dies in the "before
    all" `initSigners` hook before it ever reaches FHE. Two rounds proved the fix path:
    - First tried dropping just `--state-interval` (which rewrites every N seconds) and keeping
      `--state` (dump on SIGTERM only). **Still corrupted** — proven live a second time, 18
      restarts. The dump-on-shutdown path is itself non-atomic and gets interrupted (eviction, a
      grace-period overrun, a graceful restart of a large ~20 MB state).
    - Final fix (encoded in `charts/anvil-node`): **no `--state` at all.** The recipe boots from
      genesis every time (`clean()` wipes the PVC), so in-memory chain state is the correct model:
      if anvil restarts it comes up clean instead of crashlooping, and a mid-run restart means a
      reboot regardless. Cross-restart persistence would require atomic dumps, which anvil lacks.
    NOTE: this contradicts the §0 "persistence finding" (a Docker-daemon restart wiped contracts).
    That finding was about surviving INFRA restarts across sessions; the genesis-boot model solves
    that differently (just reboot), and `--state` causes a *worse* failure than it prevents.

17. **`host_chains` MUST be seeded or the zkproof-worker can't verify input proofs → every
    transfer hangs (plaintext mint passes).** The coprocessor's `host_chains` table maps a host
    chain_id → its ACL address; the zkproof-worker reads it to verify encrypted-input proofs. If
    empty, the worker logs `Process zk-verify request … host_chain_id:"12345"` →
    `Internal error: Unknown chain ID: 12345` → `unrecoverable error, a restart required`, so
    `verify_proofs` stays `pending`, the relayer's input-proof GET returns HTTP 202 forever, the
    SDK's `encryptUint64` never returns, and the test dies at the 300s mocha timeout. The
    **plaintext** mint test (`should mint the contract`) has no encrypted input, so it passes —
    which is why "mint green / transfers hang" is the signature of this bug. Neither mechanism that
    *should* seed it fired on v0.13.0-6: the db-migration env-seed (`HOST_CHAINS_COUNT` +
    `HOST_CHAIN_<i>_{ID,NAME,ACL}`, see fhevm-cli `generate/env.ts`) only runs on a fresh-DB init,
    and the documented v0.13 "runtime self-seed" did not happen. Fix (encoded): the `coprocessor`
    phase seeds it explicitly after db-migration — `INSERT INTO host_chains (chain_id, name,
    acl_contract_address) VALUES (<cfg.hostChain.chainId>, 'host', <DISCOVERED acl>)` — threading
    the discovered ACL (discover→regenerate), before any proof request so the worker never caches
    the miss. VALIDATED live: after seeding + a zkproof-worker restart, `Proof verification
    successful` for every request.

    Finding 17 has one more requirement, proven live: the **zkproof-worker caches host_chains at
    startup**, and the coprocessor workloads are applied before the seed runs — so the recipe must
    **restart the zkproof-worker after seeding** (`ctx.restart`), else it never picks up chain-12345
    proofs (no error, just `verify_proofs` stuck `pending`, encryptUint64 hangs). Encoded.

18. **RESOLVED: transfer reverted on-chain with `InvalidSigner(address)` (`0xbf18af43`) — the host
    InputVerifier was deployed with a STALE gateway InputVerification address.** The host
    `InputVerifier` uses `EIP712UpgradeableCrossChain`, initialized with
    `verifyingContractSource = <gateway InputVerification address>` + `chainIDSource`. The
    coprocessor signs input attestations under the gateway's cross-chain domain; the host verifies
    against `verifyingContractSource`. `host-sc-env.INPUT_VERIFICATION_ADDRESS` carried the STALE
    template `0x3b12…` (not the order-dependent discovered `0x3576…`), so the host's domain
    mismatched and `ecrecover` returned a garbage, varying address → `InvalidSigner`. Same
    discover→regenerate class as the relayer-config bug. Fix (encoded in `render.ts`): thread the
    discovered gateway InputVerification into **host-sc-env** before host-deploy. VALIDATED: with
    findings 17+18 fixed, the erc20 functional suite passes 7/7 (mint, transfer×2, transferFrom,
    allowance, reject-not-allowed, should-not-transfer); 10 user-decrypts complete. The only
    remaining e2e failures are the `EncryptedERC20:HCU` block-cap tests, which mutate the protocol
    HCU rate-limiter (`setHCUPerBlock`) and revert `NotHostOwner` — they need the host-owner key
    (test/protocol-governance config), not erc20 token behavior.

    [historical] OPEN diagnosis (now resolved above): With finding 17 fixed, the transfer gets past encryption
    and the `transfer` tx itself reverts in `estimateGas`: `execution reverted: custom error
    0xbf18af43` = `InvalidSigner(address)`. The host FHEVMExecutor verifies the coprocessor's
    EIP-712 signature over the encrypted-input handles and rejects the recovered signer. Crucially
    the recovered address **varies per run** (`0x22d3c245…`, `0x86b011…`) and is neither the
    registered `COPROCESSOR_SIGNER_ADDRESS_0` (`0x6254A198…`, = the tx-sender wallet, correctly
    registered) nor any standard derived wallet. A fixed wrong *key* would recover the same address
    every time; a *varying* one means the signature is valid but the host reconstructs the signed
    message/domain differently than the coprocessor signed it (an EIP-712 domain or per-input field
    mismatch). The plaintext mint test never exercises this path. Resolving it needs the
    coprocessor's input-attestation signing code + the host FHEVMExecutor verification code (which
    domain fields — chainId, verifyingContract, contract/user binding — each side uses); it is NOT
    a config knob found so far. This is the sole remaining blocker between the clean genesis boot
    and erc20-green.

---

## 0a. Initial mechanics spike (2026-06-16) — superseded by §0; still corrects §1–§2

The offline render (§1–§2) *inferred* a registry blocker. A live run on a local kind
cluster shows that inference was wrong for local kind. What was actually run:

- **Cluster:** kind v0.32.0, Kubernetes v1.36.1, with the default `standard`
  StorageClass (rancher.io/local-path) present.
- **anvil-node:** `helm install` → pod `Running 1/1`, Service on 8545, **PVC bound to
  the default StorageClass**. Boots as-is, no overlay. Confirmed.
- **Private image pull:** created a `registry-credentials` secret from the local `gh`
  token (account Eikix, `read:packages`), then ran a pod on
  `ghcr.io/zama-ai/fhevm/host-contracts:v0.13.0` (~982 MB) → **pulled and Running 1/1 in
  ~50 s.** Private zama ghcr images pull straight into kind with our creds.
- **Coprocessor images are on ghcr too:** `tfhe-worker`, `host-listener`, `sns-worker`,
  `db-migration` all resolve on `ghcr.io/zama-ai/fhevm/coprocessor/*` (the same path the
  compose e2e uses).

**Correction.** The "private GHCR images need creds / can't pull / use `kind load`"
blocker in §1–§2 is wrong for local kind. With a `read:packages` token + a
`registry-credentials` secret, images pull directly — no `kind load`, no `hub.zama.org`
access. The only real registry nuance: the charts *default* the coprocessor/listener
images to `hub.zama.org` (the prod registry); repoint them to ghcr in a local overlay —
see `stack/values/kind-local.yaml`. `hub.zama.org` is reachable (HTTP 401 = up, needs
auth) but **not needed locally**.

**Still open (iterative, not a blocker):** a full multi-chart boot — apply
`kind-local.yaml`, add an in-cluster Postgres + minio, the two anvil chains, and drive
the `sc-deploy` Job → `sc-addresses` ConfigMap wiring to an e2e pass.

**Gate verdict: GREEN on the load-bearing mechanics (cluster + public-chart boot +
private-image pull all proven locally) → commit to direct-kind; compose stays only as
the documented fallback.**

---

## 1. Chart-on-kind spike: per-chart verdicts

### anvil-node — BOOTS AS-IS

`helm lint` clean (0 failures). `helm template` renders cleanly with all defaults.

No local overrides are required. All services are ClusterIP. The PVC (1 Gi RWO) has no
`storageClassName` — kind's default `standard` StorageClass satisfies it. The image
(`ghcr.io/foundry-rs/foundry:stable`) is public; no imagePullSecrets needed. The
commented-out nodeSelector/tolerations/affinity blocks reference AWS Karpenter but are
disabled and do not appear in rendered output.

Optional: set `network.mnemonic` to a known phrase for reproducible accounts. Not
required for boot.

**Kind verdict: boots without any overlay.**

---

### contracts — NEEDS LOCAL OVERRIDES

`helm lint` and `helm template` both succeed with defaults. The pod schedules on kind,
but the Job exits non-zero without real values for the application-level env vars (all
default to `""`).

Hard blockers on kind:

1. **imagePullSecrets hardcoded to `registry-credentials`** in both `sc-deploy-job.yaml`
   and `sc-deploy-statefulset.yaml` — not configurable via values. The image
   (`ghcr.io/zama-ai/fhevm/host-contracts`) is private. Requires either:
   (a) `kubectl create secret docker-registry registry-credentials ...` pre-created, or
   (b) image pre-loaded with `kind load docker-image`.

2. **`storageClassName: ""`** on the PVC. An empty string may be treated as "no
   StorageClass" by some Kubernetes versions (bypassing the default) rather than "use
   cluster default". Safe fix: set `persistence.volumeClaim.storageClassName: standard`
   in a local overlay.

Runtime blockers (pod starts, Job fails without these):
- `DEPLOYER_PRIVATE_KEY`, `PAUSER_SET_ADDRESS`, `PAUSER_SET_CONTRACT_ADDRESS`
- `PAUSER_ADDRESS_0..N`, `KMS_TX_SENDER_ADDRESS_0..N`, `KMS_SIGNER_ADDRESS_0..N`
- `KMS_NODE_IP_0..N`, `KMS_NODE_STORAGE_URL_0..N`
- `CHAIN_ID_GATEWAY`, `DECRYPTION_ADDRESS`, `INPUT_VERIFICATION_ADDRESS`
- `COPROCESSOR_SIGNER_ADDRESS_0..N`

**Kind verdict: boots with two overlay fixes (storageClassName + imagePullSecrets
pre-step); contracts deploy only once application env vars are also supplied.**

---

### coprocessor — NEEDS LOCAL OVERRIDES

`helm lint` and `helm template` both succeed. Only Job + Service ClusterIP + Deployment
— no LoadBalancer, no PVC, no Ingress.

Hard blockers on kind:

1. **imagePullSecrets hardcoded to `registry-credentials`** in every pod spec (not
   behind a values flag). Image source: `hub.zama.org/zama-protocol/zama-ai/fhevm/coprocessor/*`.
   Same pre-step required as for `contracts`.

2. **`commonConfig.databaseUrl` defaults to `""`** — `DATABASE_URL` is only injected
   when non-empty. Without it every component binary panics on startup. Must set in a
   local overlay: `commonConfig.databaseUrl: "postgresql://user:pass@postgres-svc/coprocessor"`.

Optional/disabled by default (not kind blockers):
- `snsWorker.enabled: false` — requires AWS S3 and credentials. Leave disabled.
- `txSender.wallet.awsKms.enabled: false` — requires AWS KMS. Leave disabled.
- `databaseAuthMode: password` (default) — the IAM path requires RDS + IRSA. Default is
  fine on kind.

**Kind verdict: boots with one overlay key (`commonConfig.databaseUrl`) plus the
registry-credentials pre-step. All optional AWS components are off by default.**

---

### kms-connector — NEEDS LOCAL OVERRIDES

`helm lint` and `helm template` both succeed. Renders 3 Deployments + 3 ClusterIP
Services + 1 db-migration Job. No PVC, no LoadBalancer, no Ingress.

Hard blockers on kind:

1. **imagePullSecrets hardcoded to `registry-credentials`** in all 4 pod specs (not
   configurable). Images on `ghcr.io/zama-ai` (private). Same pre-step required.

2. **`commonConfig.databaseUrl` DSN wiring broken**: the Deployments resolve
   `DATABASE_URL` as `postgresql://$(DATABASE_ENDPOINT)/connector` but `DATABASE_ENDPOINT`
   is not set by the chart itself — it must be injected via `commonConfig.env`. Without
   it all three apps start with a broken connection string. Fix in local overlay:
   ```yaml
   commonConfig:
     env:
       - name: DATABASE_ENDPOINT
         value: "postgres:5432"
   ```

3. **`kmsConnectorTxSender.wallet.secret`** (name: `kms-connector-tx-sender`, key:
   `kms-wallet`) is a `secretKeyRef` that must pre-exist in the namespace. Pod will fail
   with `CreateContainerConfigError` until the Secret exists.

4. **OTEL endpoint active by default** (`kmsConnectorTxSender.tracing.enabled: true`):
   tx-sender emits `OTEL_EXPORTER_OTLP_ENDPOINT` pointing at
   `http://otel-deployment-opentelemetry-collector.observability.svc.cluster.local:4317`
   which does not exist on kind. Set `kmsConnectorTxSender.tracing.enabled: false` in
   the overlay to suppress.

**Kind verdict: boots with three overlay keys + two kubectl pre-steps (registry secret +
wallet secret).**

Minimum local overlay:
```yaml
kmsConnectorTxSender:
  tracing:
    enabled: false
commonConfig:
  env:
    - name: DATABASE_ENDPOINT
      value: "postgres:5432"
```
Pre-steps:
```sh
kubectl create secret docker-registry registry-credentials \
  --docker-server=ghcr.io --docker-username=... --docker-password=...
kubectl create secret generic kms-connector-tx-sender \
  --from-literal=kms-wallet=<private-key-hex>
```

---

### listener — NEEDS LOCAL OVERRIDES

`helm lint` clean. `helm template` with no values renders only a ServiceAccount — every
other resource (Deployment, ConfigMap, Service, Secret) is gated on
`if .Values.listeners` or `if not .Values.externalSecret.enabled`.

Hard blockers on kind:

1. **`listeners` defaults to `[]`**: no Deployment, ConfigMap, or Service is emitted
   until at least one listener entry with `name` and `blockchain.chain_id` is provided.

2. **`externalSecret.enabled: true` by default**: assumes the External Secrets Operator
   is installed and has already provisioned `listener-secrets`. A vanilla kind cluster
   has no ESO. Must set `externalSecret.enabled: false` and supply `fallbackSecret.data`.

3. **Image `hub.zama.org/ghcr/zama-ai/fhevm/listener/listener-core`** is private (same
   registry pre-step as all other charts). The eRPC sidecar (`ghcr.io/erpc/erpc`) is
   public and does not need auth.

4. **`database-credentials` Secret**: the default `env` block references
   `secretKeyRef: {name: database-credentials, key: database-url}` — this Secret is
   never created by the chart and is not overridden by `fallbackSecret`. Must be
   pre-created separately.

**Kind verdict: boots with a 3-key overlay (one `listeners[]` entry,
`externalSecret.enabled: false` + `fallbackSecret.data`, registry pre-step) plus one
additional kubectl pre-step for `database-credentials`.**

---

## 2. Overall kind verdict

The full fhevm stack can be hosted on kind. There is no chart-level blocker that would
require structural changes to the Helm templates. Every blocking item is a configuration
gap (missing values overlay, imagePullSecrets pre-step, or a missing pre-existing Secret)
that is addressed at deploy time.

Summary of required pre-steps (shared across all charts):
```sh
# One-time per kind cluster namespace
kubectl create secret docker-registry registry-credentials \
  --docker-server=ghcr.io --docker-username=<GHCR_USER> --docker-password=<GHCR_TOKEN>
kubectl create secret docker-registry registry-credentials-hub \
  --docker-server=hub.zama.org --docker-username=<HUB_USER> --docker-password=<HUB_TOKEN>
kubectl create secret generic kms-connector-tx-sender \
  --from-literal=kms-wallet=<private-key-hex>
kubectl create secret generic database-credentials \
  --from-literal=database-url=postgresql://user:pass@postgres-svc/listener
```

The coprocessor, kms-connector, and listener charts also need an in-cluster PostgreSQL
instance — not included in the current chart set but trivially provided by a
`bitnami/postgresql` Helm release or a simple Deployment.

---

## 3. Scaffolded files

All files below live under `stack/` and
`.github/workflows/acceptance.yml` in the worktree. Files marked STUB are skeleton code
only — they do not run and contain no implementation.

| File | Idea demonstrated | Status |
|---|---|---|
| `stack/lib/stack.ts` | Full Stack API interface: `RolloutRunContext` promoted to typed kind+Helm surface; chaos/read-state primitives (`exec`, `sql`, `stop`, `start`, `restart`, `logs`, `waitForLog`, `chain`, `until`) | STUB |
| `stack/lib/helm.ts` | Thin wrapper over `helm upgrade --install --wait` with values-file merging and dry-run mode | STUB |
| `stack/lib/kubectl.ts` | Thin wrapper over `kubectl` for exec, scale, rollout-status, configmap reads | STUB |
| `stack/cli/main.ts` | CLI entry point: command routing (`up`, `down`, `rollout`, `state`, `test`) | STUB |
| `stack/cli/up.ts` | `up` command: load MANIFEST, call `helm upgrade --install` per chart in dependency order | STUB |
| `stack/cli/runbook.ts` | `rollout run <script>` command: load runbook module, construct Stack context, execute | STUB |
| `stack/cli/test.ts` | `test` command: invoke Stack API `test()` with a named profile | STUB |
| `stack/values/default.yaml` | Default values overlay illustrating the MANIFEST contract (Contract 1) — image.tag keys taken verbatim from real charts | STUB |
| `stack/values/two-of-three.yaml` | Scenario overlay: `NUM_COPROCESSORS=3`, coprocessor signer addresses, `kmsCoreEndpoints` indirection (Contract 2) | STUB |
| `stack/runbooks/drift.ts` | Built-in drift-check runbook: `up → state() → diff` | STUB |
| `stack/runbooks/v0.12-to-v0.13.ts` | Built-in rollout runbook: `up → upgrade(group) → snapshotContracts → test` | STUB |
| `stack/fhevm` | Symlink / reference anchor pointing at `test-suite/fhevm/` for IDE navigation | REAL |
| `stack/README.md` | Full design document: two-layer architecture, three data contracts, real values-key references, sc-addresses wiring, multi-coprocessor indexing, acceptance levels | REAL (grounded) |
| `.github/workflows/acceptance.yml` | Acceptance CI harness: `record-goldens` job (run once on the old CLI) + `acceptance` matrix (case × level, L0–L3), golden-artifact download, per-level diff steps, always-teardown | STUB (harness shape is real; all step bodies are PLACEHOLDERs) |

---

## 4. What is REAL vs STUB

### REAL and grounded (verified against repo source)

- `RolloutRunContext` interface — defined in
  `test-suite/fhevm/src/commands/rollout-run.ts` lines 53–65; used by the
  `v0.12-to-v0.13` runbook. The `Stack` interface in `lib/stack.ts` extends it.
- All values keys cited in `README.md` and the scaffolded values files — verified
  against `charts/*/values.yaml`.
- sc-deploy Job → `sc-addresses` ConfigMap → `configMapKeyRef` address wiring —
  verified in `charts/contracts/templates/sc-deploy-config.yaml` and
  `charts/coprocessor/templates/coprocessor-host-listener-deployment.yaml` lines
  172–190.
- `NUM_COPROCESSORS` + `COPROCESSOR_SIGNER_ADDRESS_x` indexing — in
  `charts/contracts/values.yaml` lines 127–134.
- `kmsCoreEndpoints` — `charts/kms-connector/values.yaml` line 201.
- `hostChainWsUrl`, `gatewayUrl`, `commonConfig` — `charts/coprocessor/values.yaml`
  lines 62–71.
- kms-core is external (no `charts/kms-core` directory exists).
- `dbMigration` as a Kubernetes Job — confirmed in coprocessor chart templates.
- `scUpgrade.enabled` / `upgradeCommands` path — `charts/contracts/values.yaml`.
- `readinessProbe` on all deployable components — confirmed in coprocessor,
  kms-connector, listener chart templates.
- Helm lint + template results for all five charts — run offline as part of this spike.

### STUB (not yet implemented)

- `lib/stack.ts` — interface only; no engine implementation.
- `lib/helm.ts`, `lib/kubectl.ts` — empty skeletons.
- `cli/*.ts` — command handlers are empty stubs.
- `values/*.yaml` — illustrative; not wired to any runner.
- `runbooks/*.ts` — empty skeletons; no Stack API calls.
- `.github/workflows/acceptance.yml` step bodies — all are `echo "PLACEHOLDER"`.
- Golden-master record/replay harness (L0–L3) — L0 runs locally; L1–L3 not written. No
  L2 `erc20` golden recorded yet (the missing behavioral oracle).
- **L0 was RED then RE-RECORDED → now GREEN (2026-06-17).** `run-l0.sh {default,two-of-three}`
  had drifted from the committed golden: the merged PR **#2796 "kms-connector multichain
  common config"** (which arrived via the rebase onto origin/main) changed the chart *after*
  the golden was recorded — the templates dropped `DATABASE_ENDPOINT`/`PGUSER`/`PGPASSWORD`
  and now emit `KMS_CONNECTOR_POLYGON_*` **unconditionally** (`kms-worker-deployment.yaml:76`,
  no `{{- if }}` guard), so the single-chain `default` case renders empty polygon env.
  **Resolution + audit trail:** this is the *merged upstream* chart's behavior, not a
  new-engine regression, and L0 is a **template-regression guard** — runtime correctness is
  L2's job (the goal: "L2 behavioral is the real equivalence, not L0 byte-diff"). The
  empty-polygon env is the multichain-**opt-in** design (the live single-chain connector this
  session ran healthy with polygon config absent), so it is an L2/runtime question, logged
  here, not an L0 blocker. Re-recorded with `run-l0.sh <case> --record` (helm v4.1.4):
  default = 25 docs / `b17931ea…`, two-of-three = 49 docs / `1480c88c…`; both now PASS.
  `acceptance.yml` `setup-helm` steps are pinned to **v4.1.4** (record + verify must use the
  same helm — helm 4 renders nil values differently from helm 3). L2 still owes: confirm the
  connector tolerates empty `POLYGON_*` at runtime (cluster), and record the erc20 pass-set.

### PROVEN live but NOT yet encoded (the gap `stack/lib` must close)

The §0 boot did all of these **by hand on `fhevm-p2`**; none is yet in `lib`:

- kind cluster bootstrap, `registry-credentials`, in-cluster Postgres + minio — done live.
- Boot dependency ordering (the 15-step recipe in §0) — proven, not codified.
- KMS-signer discovery + the `generateRuntime` placeholder-overwrite pattern — understood
  (§0 invariant 2), not implemented.
- FHE keygen DKG (kms-core + connector + host-side trigger) — finalized live, not codified.

---

## 5. Recommended next steps

### Step 1 — Finish the live boot to `erc20` (DONE through FHE keygen — see §0)

The full kind boot is no longer hypothetical: §0 records it run live, from an empty
cluster through a **finalized FHE keygen** (the recipe + invariants are in §0). What
remains on the live cluster (`fhevm-p2`):

1. **coprocessor** (db-migration + 9 svcs) against the generated keys —
   `AWS_ENDPOINT_URL=http://minio:9000`, expand `${VAR}` in `DATABASE_URL`,
   `FHE_KEY_ID` = the realized id from §0 step 12.
2. **relayer**.
3. **`erc20` e2e** → **record the first L2 golden** (the behavioral oracle the whole
   acceptance plan depends on).

### Step 2 — Extract the Stack API implementation in place

`lib/stack.ts` is currently an interface. Implement it against the real charts —
**encoding the §0 boot recipe (the 15 ordered phases) and its five invariants** (esp.
kms-signer discovery, the deploy→connector→trigger ordering, and the placeholder-overwrite
convention) — without introducing new abstractions:

- `up()`: shell out to `helm upgrade --install --wait` for each chart in order.
- `discovery()`: `kubectl get configmap sc-addresses -o json` → parse
  `ContractAddresses`.
- `refreshDiscovery()`: re-read `sc-addresses` ConfigMap and patch running Deployments.
- `exec()`, `sql()`, `stop()`, `start()`, `restart()`, `logs()`: thin wrappers over
  `kubectl exec/scale/rollout/logs`.
- `test()`: delegate to the existing test-suite runner (avoid reimplementing).

Keep the implementation in `stack/lib/` and make the existing
`test-suite/fhevm/src/commands/rollout-run.ts` entry point call through the new Stack
API rather than the Docker Compose layer directly. This gives a live integration point
before the Docker Compose layer is removed.

### Step 3 — Wire the acceptance.yml harness

Replace all `echo "PLACEHOLDER"` step bodies in
`.github/workflows/acceptance.yml` with real commands:

1. **L0 first**: `helm template` + normalize script. No cluster needed; can run in CI
   today once the CLI has a `template` subcommand that reads the MANIFEST.
2. Record goldens once from the current CLI using the `record-goldens` job
   (`workflow_dispatch` with `record_goldens: true`).
3. Add the workflow as a required check on PRs that touch `charts/*` or
   `test-suite/fhevm/src/`.
4. Promote to L1 once the kind boot is verified on a devbox. L2 and L3 follow naturally
   once the test and runbook paths are wired.

### Step 4 — Remove Docker Compose (final cutover)

Once L2 acceptance passes consistently, delete the Docker Compose orchestration layer
(`test-suite/fhevm/src/commands/rollout-run.ts` and its Docker-specific dependencies)
and the `acceptance.yml` migration harness. The Stack API backed by kind+Helm becomes
the sole engine.
