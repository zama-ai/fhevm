# KMS Context Switch & Key Resharing Runbook

Operator guide for the two governance-driven KMS-committee lifecycle operations on a
live environment, where KMS nodes are run by several external
parties:

| Operation                                          | What changes                                               | On-chain entrypoint                    | Party metadata needed            |
| -------------------------------------------------- | ---------------------------------------------------------- | -------------------------------------- | -------------------------------- |
| **1. Key resharing (epoch rotation)**              | New key shares, same committee                             | `defineNewEpochForCurrentKmsContext()` | **None** (no-arg call)           |
| **2. Context switch (committee change/node swap)** | Committee membership and/or metadata; includes a resharing | `defineNewKmsContextAndEpoch(...)`     | Yes — the **new** committee (§2) |

Both run on the canonical host (Ethereum) only, and they can only ever complete there: the
KMS connectors watch Ethereum's `ProtocolConfig` and no other chain. Non-canonical hosts run
`ProtocolConfigReplica`, which has no `defineNew*` functions. They are updated by mirroring
the result of the canonical operation instead (§5).

Audience: whoever coordinates the governance operation with the external KMS parties.

---

## 1. Version prerequisites

The reference release is [`v0.14.0-1`](https://github.com/zama-ai/fhevm/tree/v0.14.0-1)
— the first release where every component is context-switch aware. Check the deployed
versions **before** scheduling any window; the standard decrypt path working proves
nothing about the switch path.

| Component                           | Minimum version              | Artifact                                                                           | How to check the deployment                         |
| ----------------------------------- | ---------------------------- | ---------------------------------------------------------------------------------- | --------------------------------------------------- |
| Host contracts (`ProtocolConfig`)   | ProtocolConfig `>=0.2.0`     | `ghcr.io/zama-ai/fhevm/host-contracts:v0.14.0-1`                                   | `cast call $PROTOCOL_CONFIG "getVersion()(string)"` |
| Gateway contracts (`GatewayConfig`) | GatewayConfig `>=0.7.0`      | `ghcr.io/zama-ai/fhevm/gateway-contracts:v0.14.0-1`                                | `cast call $GATEWAY_CONFIG "getVersion()(string)"`  |
| KMS connector (all parties)         | fhevm `v0.14.0-1`            | `ghcr.io/zama-ai/fhevm/kms-connector/{gw-listener,kms-worker,tx-sender}:v0.14.0-1` | Confirm running parties image tag                   |
| KMS core (all parties)              | kms `v0.14.0-1` or later     | `ghcr.io/zama-ai/kms/core-service:v0.14.0-1`                                       | Confirm running parties image                       |
| Relayer                             | relayer `v0.14.0-1` or later | `ghcr.io/zama-ai/fhevm/relayer:v0.14.0-1`                                          | Confirm running image tag                           |

## 2. Preparing the KMS node party metadata

A context switch registers the new committee on-chain as one `KmsNodeParams` entry
per KMS node (struct in `host-contracts/contracts/shared/Structs.sol`). Each entry is the node's
full protocol identity — how the chain authenticates it, how the other parties'
cores reach it, and where it publishes its public key material:

- **two distinct wallets**: the _tx-sender_ is the **KMS connector's** wallet. It
  pays gas. It submits no lifecycle confirmations in this version. The _signer_ is the
  **KMS core's** signing key. It signs key-material attestations and the lifecycle
  confirmations. It also signs the core's responses to gRPC commands, such as keygens and
  decryptions. A common mistake is swapping them.
- **network endpoints**: the core's MPC endpoint (peer-to-peer between parties) and
  the public vault (S3 bucket) where the core publishes its verification address and
  TLS CA cert;
- **MPC metadata**: the positional `partyId` (assigned by the coordinator) and the
  `mpcIdentity` string, which must byte-match the party's own core configuration.

Most fields only the party itself can provide — collect them with the questionnaire
(§2.1), verify each answer against the party's public storage (Annex B.2), then
assemble and validate the env file (Annex B.3).

| Field             | Format                                                                                                                                                                                                                                           | Provided by                      | Used for (end to end)                                                                                                                                                                                                                    | Validation                                                                                                |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `txSenderAddress` | 20-byte EVM address (`0x` + 40 hex)                                                                                                                                                                                                              | **KMS party**                    | The **connector's** wallet. Host chain: sends no switch confirmations in this version (§4.2 step 4). Gateway chain: broadcasts decryption responses. `Decryption` requires the response sender to be this registered node                | Party signs a message with it, or sends a test tx; must be funded on the host chain **and** gateway chain |
| `signerAddress`   | 20-byte EVM address of the **core's** signing key                                                                                                                                                                                                | **KMS party** (verifiable by us) | Signs the lifecycle confirmations (epoch activation is unanimous per signer) and EIP-712 decryption responses (verified per pinned context on the gateway). The core publishes it at `<storageUrl>/<storagePrefix>/VerfAddress/<handle>` | Fetch from their public storage and compare (Annex B.2)                                                   |
| `ipAddress`       | URL `scheme://host:port` — host may be a **DNS name** or IP; the **port is mandatory and must be non-default** (`:443`/`:80` are normalized away by the core's URL parser and rejected as "missing port"). E.g. `http://kms.party.example:50001` | **KMS party**                    | The core's MPC endpoint. Consumed **only by the other parties' cores**: each peer parses host+port and dials it for MPC traffic (reshares, decryption sessions).                                                                         | Reachable from every _other_ party's core (peer-to-peer, not from us)                                     |
| `storageUrl`      | Base URL of the party's public vault (S3-compatible bucket)                                                                                                                                                                                      | **KMS party**                    | Where the core publishes its public material. Emitted in `ActivateEpoch` and served per key/CRS result by `KMSGeneration` → the relayer's `/v2/keyurl`, so clients know where to download public keys and CRS                            | `curl` returns objects (Annex B.2)                                                                        |
| `partyId`         | `int32`, contiguous `1..n`, **positional**                                                                                                                                                                                                       | **Coordinator (us)**             | The party's MPC role index (the core builds its role map from it and rejects out-of-range or duplicate ids). A swap keeps the dropped node's id                                                                                          | Annex B.3 validator checks contiguity + duplicates                                                        |
| `mpcIdentity`     | Free-form string; must **byte-match** the `mpc_identity` in the party's core config                                                                                                                                                              | **KMS party**                    | The party's logical identity in the MPC layer: self-recognition (which roster slot is "me" — a mismatch aborts the reshare), TLS trust-root lookup (peer cert verification is keyed by it), and message routing                          | Must byte-match their config — ask them to paste it, do not derive it                                     |
| `caCert`          | `bytes` = the party's TLS CA as a regular PEM file, verbatim (`-----BEGIN CERTIFICATE-----` … `-----END CERTIFICATE-----`, usually newline-terminated); carried as `0x` + hex of those exact bytes in the env                                    | **KMS party** (verifiable)       | The party's **long-lived** CA: peers verify mutual-TLS MPC connections against it (looked up by `mpcIdentity`). Published at `<storagePrefix>/CACert/<handle>`; **not** regenerated by reshares or context switches                      | Fetch + `openssl x509` parse (Annex B.2); the hex must be byte-exact — it feeds the context anchor        |
| `storagePrefix`   | String key prefix inside the vault (e.g. `PUB-p3`)                                                                                                                                                                                               | **KMS party**                    | Namespaces the party's objects in its vault: `<storageUrl>/<storagePrefix>/VerfAddress/<handle>`, `…/CACert/<handle>`, and the public key material clients download                                                                      | The `VerfAddress`/`CACert` fetches above implicitly validate it                                           |

Committee-level values (set once, not per node):

| Value            | Env var (host)                                                                                   | Notes                                                                                                                                                                                                                                                                                                                                                                  |
| ---------------- | ------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Thresholds       | `PUBLIC_DECRYPTION_THRESHOLD`, `USER_DECRYPTION_THRESHOLD`, `KMS_GEN_THRESHOLD`, `MPC_THRESHOLD` | Committee size must satisfy `n = 3 * MPC_THRESHOLD + 1` **exactly** — every KMS core re-checks this equality when it ingests the new context and rejects the switch otherwise (the contract itself only checks `1 ≤ t ≤ n`)                                                                                                                                            |
| Software version | `KMS_SOFTWARE_VERSION`                                                                           | The KMS core release version all parties run                                                                                                                                                                                                                                                                                                                           |
| PCR values       | `KMS_PCR_VALUES`                                                                                 | JSON array `[{"pcr0":"0x…","pcr1":"0x…","pcr2":"0x…"}, …]`; enclave measurements from the KMS release (zeros only on non-enclave dev stacks). The array may hold several entries — for a rolling KMS core upgrade include **both the old and the new release's measurements**, so nodes on different versions keep attesting each other while the upgrade is in flight |

### 2.1 Questionnaire to send external KMS parties

> Copy-paste; one reply per party.

```text
For the upcoming KMS committee update on <ENV>, please provide:

1. Connector tx-sender address (the wallet your kms-connector broadcasts with):
2. Core signer address (your kms-core signing key address / VerfAddress):
3. Core MPC endpoint URL (reachable by the other KMS parties):
4. Public vault base URL (S3-compatible store where your core publishes public material):
5. Public vault storage prefix (e.g. PUB-p3):
6. mpc_identity — the EXACT `mpc_identity` string from your kms-core config:
7. TLS CA certificate (PEM, the one your core publishes under CACert/):
8. KMS core version you are running / will run at the switch window:

Notes:
- (1) and (2) are different keys: (1) is your kms-connector's wallet and pays gas.
  The contract authenticates confirmations by (2), not by (1). (2) is your
  kms-core's signing key. It signs key-material attestations and the core's
  responses to gRPC commands (keygens, decryptions). Do not swap them.
- (6) must be copied verbatim from your config, not retyped.
- (7) is your long-lived TLS CA — reshares and context switches do not regenerate
  it. If you plan to rotate it, tell the coordinator: registering the new cert
  requires a committee-metadata context switch.
- Your tx-sender (1) must be funded on both the host chain and the gateway chain.
```

---

## 3. Key resharing (epoch rotation, same committee)

The simplest operation: the committee keeps its membership and generates fresh key
shares under a new epoch. **No node params, no env file, no party coordination
beyond a heads-up** — every current member reshares with itself as both sender and
receiver.

1. **Heads-up to all parties**: connectors and cores must be up, tx-senders funded.
2. **Broadcast** — via a DAO proposal where governance owns the contract
   (production), or directly with `DEPLOYER_PRIVATE_KEY` where it does not
   (devnet/test stacks):

   ```bash
   cd host-contracts
   # DAO path — produces calldata for the governance proposal, never broadcasts:
   npx hardhat task:buildDefineNewEpochForCurrentKmsContextCalldata --network <network>

   # No-DAO path — broadcasts with DEPLOYER_PRIVATE_KEY:
   npx hardhat task:defineNewEpochForCurrentKmsContext --network <network>
   ```

3. **After the broadcast**: the contract emits `NewKmsEpoch` for the same context
   (`kmsContextId == previousContextId`). Each connector drives its
   core through the reshare. Activation needs a `confirmEpochActivation` from every
   signer, with its key/CRS results and its signed
   `EpochActivationConfirmation`. Once the signatures are unanimous the contract emits
   `ActivateEpoch`.
   **The connector does not submit this confirmation in this version.** kms-grpc returns
   no confirmation signature, so the tx-sender marks the result failed and sends nothing.
   No other service submits it. The new epoch stays Pending, and a rotation cannot
   complete until planning-blockchain#1369 lands.
4. **Watch**: `task:kmsContextSwitchStatus` (Annex B.1) until "fully live". The
   task labels what it sees on-chain — `same-set-rotation` is _this_ flow (a new
   epoch issued under the same, still-active context), as opposed to
   `context-switch` (a pending context newer than the active one, §4) and `idle`
   (nothing in flight). Alternatively poll `getCurrentKmsContextAndEpoch` for the
   epoch id to advance. In this version the epoch stays PENDING and the epoch id does
   not advance (step 3).
5. **Verify**: run a decryption smoke test **after** the epoch id advances. The
   relayer must already be epoch-aware (§1) or requests will carry the stale epoch.
   In this version the rotation never reaches this step.
6. **Gateway: nothing to do.**
7. **Replicas**: mirror the new epoch onto every non-canonical host chain (§5).

## 4. Context switch (committee change / node swap)

The full operation: governance opens the switch on the host chain **and registers
the new committee on the gateway** (before host-side activation — see step 3 of
§4.2 for why), both committees confirm, and a reshare transfers the key material to
the new committee. For a node swap remember: the incoming node **inherits the outgoing node's
`partyId`** (party ids are positional — Annex A); `n` and the thresholds stay valid
(`n = 3t + 1`).

### 4.1 Preconditions

- §1 version matrix green across **all** parties (old and new members).
- **Incoming node(s)**: core running and reachable by the _other parties'_ cores,
  started **without any pre-configured committee peers** (the switch itself
  delivers the committee topology to it), public storage exposing
  `VerfAddress`/`CACert`, connector running. Tx-sender funded on the **host chain**
  and on the **gateway chain**. On the gateway chain the node starts broadcasting
  decryption-response transactions the moment it serves.
- **Outgoing node(s)**: keep online whenever possible — they are needed twice.
  On-chain, the creation quorum requires **`n − t`** previous-committee
  confirmations; in the MPC reshare the old nodes are the key-material senders and
  the protocol likewise needs `n − t` of them. Both phases therefore tolerate up
  to `t` dead-or-silent previous nodes, so an unrecoverable outgoing node does not
  block the switch — but every avoidable absence eats into that budget.

### 4.2 Execute

1. **Collect and validate the new committee's metadata**: questionnaire (§2.1),
   per-party verification (Annex B.2), env file assembly + validation (Annex B.3).
   For continuing members, reuse their registered values verbatim — but note the
   contract stores only four fields: `getKmsNodesForContext(<currentContextId>)`
   returns tx-sender, signer, `ipAddress` and `storageUrl`. The MPC-metadata
   fields (`partyId`, `mpcIdentity`, `caCert`, `storagePrefix`) are recovered from
   the current context's `NewKmsContext` event, which carries the full
   `KmsNodeParams` and is integrity-checked by the anchor
   (`getKmsContextAnchor`).
2. **Broadcast on the host** — via a DAO proposal where governance owns the
   contract (production), or directly with `DEPLOYER_PRIVATE_KEY` where it does
   not (devnet/test stacks):
   ```bash
   cd host-contracts
   # DAO path — produces calldata for the governance proposal, never broadcasts:
   npx hardhat task:buildDefineNewKmsContextAndEpochCalldata --network <network>
   # No-DAO path — broadcasts with DEPLOYER_PRIVATE_KEY:
   npx hardhat task:defineNewKmsContextAndEpoch --network <network>
   ```
   The calldata task prints the future id up front —
   `newContextId (set as the Gateway proposal's KMS_CONTEXT_ID): …`, computed as
   `current + 1` in the `0x07…` domain — precisely so the gateway proposal (next
   step) can be prepared in parallel. The tx leaves both the context and its epoch
   **PENDING**.
3. **Register the new committee on the Gateway — before host-side activation**:

   ```bash
   cd gateway-contracts
   export KMS_CONTEXT_ID=<the newContextId printed in step 2>   # must match the host
   # DAO path — builds the cross-chain proposal triple, never broadcasts:
   npx hardhat task:buildUpdateKmsContextProposal --verify-context-id true --network <network>
   # No-DAO path — broadcasts with DEPLOYER_PRIVATE_KEY:
   npx hardhat task:updateKmsContext --network <network>
   ```

   This uses the **gateway** env spellings (`KMS_NODE_IP_ADDRESS_{i}`,
   `KMS_GENERATION_THRESHOLD` — see the trap note in Annex B.3).
   `--verify-context-id` pre-flights the on-chain guard: the id must strictly
   advance the gateway's current one.

   **Why before activation:** the gateway's `Decryption` contract verifies each
   response against the context id pinned to its request. Once the new committee
   activates on the host it answers under the new context id; until the gateway
   knows that id, those responses are rejected (`NotKmsSigner` /
   `DecryptionContextMismatch`) — and activation timing is not under operator
   control (it fires on the last activation confirmation). Registering the
   pending context in advance closes that window: the old context **stays
   registered** on the gateway, so in-flight requests keep verifying against the
   old committee while it is still the one answering. Two caveats:
   - **Requests with empty or v0 `extraData` can get stuck.** Such a request names no
     context, so the gateway pins it to the context that is current when it arrives.
     The connectors cannot see that pinned context, and their response names no context
     either, so the gateway reads the current context again when the response arrives.
     After `updateKmsContext` the current context is the new one. It no longer matches
     the pinned one, every response is rejected, and the request is never answered. The
     requester loses the fee. Run `updateKmsContext` when no such requests are in
     flight, or accept that loss.
   - **The gateway pointer cannot go back.** `updateKmsContext` only accepts a context id
     higher than the one already registered (`KmsContextAlreadyRegistered`). If the host
     switch is cancelled with `destroyKmsContext` after this step, you cannot point the
     gateway back to the old context. Instead, start the retry switch, which gets a
     higher context id, and register that id on the gateway.

4. **Phase 1, creation quorum**: every signer (old and new
   committees) needs a `ContextCreationConfirmation` signature submitted through
   `confirmKmsContextCreation`. When **all new + (n − t) old** signers have confirmed
   the same digest, the context becomes CREATED and the contract emits the
   `NewKmsEpoch` that starts the reshare.
   **The connectors do not submit these confirmations in this version.** kms-grpc
   returns no confirmation signature, so the tx-sender marks each confirmation failed
   and sends nothing. No other service submits them. The context stays PENDING,
   the contract never emits `NewKmsEpoch`, and phase 2 never starts. A context switch cannot
   complete until planning-blockchain#1369 lands.
5. **Phase 2, reshare and activation**: outgoing nodes send (Set 1),
   incoming nodes receive (Set 2), continuing nodes do both. Then every
   **new-committee** signer needs a `confirmEpochActivation`. Unanimity on the
   same `epochMaterialHash` and `extraData` triggers `ActivateEpoch`. The context becomes ACTIVE and
   `getCurrentKmsContextAndEpoch` advances.
6. **Watch throughout**:
   ```bash
   npx hardhat task:kmsContextSwitchStatus --network <network> --from-block <block>
   ```
   While PENDING it lists exactly which new signers are outstanding and the
   old-side `n − t` target. It counts confirmations of every digest and warns when
   signers confirmed different `extraData`. A signer that confirmed other `extraData`
   cannot confirm again. This tells you _which party_ to chase. In this version every
   signer stays outstanding (step 4).

### 4.3 Verify

- `getCurrentKmsContextAndEpoch()` returns the new context + its epoch.
- Decryption smoke test passes.
- Evidence the **incoming node actually serves**: its core logs show reshare
  completion and it answers decryption sessions (party logs / metrics). On staging
  only, the conclusive test is forcing it into the quorum: stop `t` continuing
  members so the live set is exactly `2t + 1` _including_ the new node, and decrypt
  — this is what the E2E `kms-context-switch` profile automates. **Do not do this
  on mainnet.**
- **Mirror the switch onto every non-canonical host** (§5). Until then, those chains still
  return the previous pair from `getCurrentKmsContextAndEpoch` and still list the previous
  committee.
- Only after all of the above: the outgoing party may decommission its node.
  Coordinate retention of its old key material per policy (it remains part of the
  previous context's history).

## 5. Mirroring onto non-canonical hosts (replicas)

Ethereum is the canonical host: §3 and §4 run there and nowhere else. Every other host
chain (e.g. Polygon) runs `ProtocolConfigReplica`, which has no lifecycle: no quorum, no
reshare and no `KMSGeneration`. Its only write paths are four `onlyACLOwner` mirror
methods. `mirrorKmsContextAndEpoch` and `mirrorKmsEpoch` import state Ethereum has already
finalized and land it as immediately `Active`. `mirrorKmsContextDestruction` and
`mirrorKmsEpochDestruction` copy a canonical `destroyKmsContext` / `destroyKmsEpoch`.
Mirroring a switch activates the new context without retiring the previous one, so a context
destroyed on Ethereum stays trusted on every replica until its destruction is mirrored there
as well (§5.6).
**Bringing each replica forward after a canonical rotation, and retiring on each replica
what Ethereum retired, is the operator's job** — nothing on-chain does it for you, and
nothing flags a replica that fell behind.

The tasks below are defined in `host-contracts/tasks/mirrorKmsContext.ts`.

### 5.1 Which task follows which operation

| Canonical operation                      | Run on each replica                                                                         | Replica method it calls       |
| ---------------------------------------- | ------------------------------------------------------------------------------------------- | ----------------------------- |
| Key resharing / epoch rotation (§3)      | `task:buildMirrorKmsEpochCalldata` · `task:mirrorKmsEpoch`                                  | `mirrorKmsEpoch`              |
| Context switch (§4)                      | `task:buildMirrorKmsContextAndEpochCalldata` · `task:mirrorKmsContextAndEpoch`              | `mirrorKmsContextAndEpoch`    |
| Context destroyed (cancel or retirement) | `task:buildMirrorKmsContextDestructionCalldata` · `task:mirrorKmsContextDestruction` (§5.6) | `mirrorKmsContextDestruction` |
| Epoch destroyed (cancel or retirement)   | `task:buildMirrorKmsEpochDestructionCalldata` · `task:mirrorKmsEpochDestruction` (§5.6)     | `mirrorKmsEpochDestruction`   |

As everywhere else in this runbook, `build…Calldata` is the DAO path (prints the target
and calldata, never broadcasts) and the bare task broadcasts with
`DEPLOYER_PRIVATE_KEY`.

**No ids or committee data are passed on the command line** for the two activation mirrors.
Those methods do take a context
id, an epoch id and — for a context switch — the node set, thresholds, software version and
PCR values, but the task reads all of it from canonical and encodes the arguments itself. The
only inputs you supply are the canonical connection flags below.

These tasks are for a replica that is **already deployed and initialized**. Standing up a
_new_ replica is a different path — `task:exportCanonicalProtocolConfig` then
`task:deployProtocolConfigFromCanonical` (`initializeFromCanonical`), documented in the
host-contracts README. Use the mirror tasks for every rotation after that.

### 5.2 Preconditions

- **Canonical must have activated already.** Both tasks read canonical's _active_
  context/epoch, so they can only run after §3 step 4 / §4 phase 2 completes. There is no
  way to pre-stage a mirror.
- **The default read is canonical's latest _finalized_ block**, not its head — a
  finalized block cannot be reorged out, which keeps the read reproducible. On Ethereum
  that trails the head by ~13 minutes (two consensus-layer epochs — not KMS epochs). Run the
  task earlier than that and it still sees the pre-rotation state, stopping with
  `Replica … is already at context <id>, which is >= canonical's <id>`. That message means
  **"canonical's rotation is not finalized yet"**, not "nothing to do" — wait and re-run.
  Pass `--block-number` to pin the read to a specific canonical height that is already finalized.
  Heights above the latest finalized block are rejected; wait for finalization and re-run.
  The tasks also re-fetch the selected block after reading and abort if its hash changed.
- **No committee metadata to collect.** Unlike §4, there is no questionnaire and no
  `committee.env`: everything the replica needs is read back from canonical. Only two
  connection parameters are required (`--canonical-rpc-url`,
  `--canonical-protocol-config-address`).
- **Replica address**: `PROTOCOL_CONFIG_CONTRACT_ADDRESS` for the replica chain, or
  `--use-internal-proxy-address` to resolve it from `addresses/`.
- **Authorization**: all four mirror methods are `onlyACLOwner` on the replica chain, so the
  broadcaster (or the DAO executing the calldata) must be that chain's ACL owner.
- **Ids only move forward, and gaps are fine.** The replica rejects any context/epoch id
  that is not strictly greater than its current one, so a mirror cannot roll it back or be
  applied twice. A replica that missed several canonical rotations does not have to replay
  them one by one: it jumps straight to canonical's current pair in a single call.
- **Gateway: nothing to do.** The gateway registers a context once, from the canonical
  flow (§4.2 step 3). It is not per-replica.

### 5.3 Mirror a context switch

> **Run by** the replica-chain operator (or its DAO) · **needs** the `host-contracts` repo,
> a canonical RPC url, the replica network config · **when** after canonical activation is
> finalized · **access** read-only on canonical; one `onlyACLOwner` write on the replica.

```bash
cd host-contracts
export PROTOCOL_CONFIG_CONTRACT_ADDRESS=<ProtocolConfigReplica proxy>

# DAO path — prints target + calldata, never broadcasts:
npx hardhat task:buildMirrorKmsContextAndEpochCalldata --network <replica-network> \
  --canonical-rpc-url <ethereum-rpc-url> \
  --canonical-protocol-config-address <canonical ProtocolConfig>

# No-DAO path — broadcasts with DEPLOYER_PRIVATE_KEY:
npx hardhat task:mirrorKmsContextAndEpoch --network <replica-network> \
  --canonical-rpc-url <ethereum-rpc-url> \
  --canonical-protocol-config-address <canonical ProtocolConfig>
```

There is no committee to re-type: `mirrorKmsContextAndEpoch` needs the full
`KmsNodeParams` plus the software version and PCR values, which the contract does not store,
so the task recovers them from canonical's `NewKmsContext` event and checks them against the
on-chain `contextInfoHash` anchor before using them. A wrong or tampered canonical RPC cannot
slip a different committee past that check. Thresholds come from the same event, so the
replica receives the values the switch was defined with.

Three errors are worth knowing:

- `reports version "ProtocolConfigReplica v…"; expected "ProtocolConfig v…"` — the
  `--canonical-protocol-config-address` points at a replica, not at Ethereum's `ProtocolConfig`.
- `has no context anchor recorded` — the address runs `ProtocolConfig` but holds no anchor for the
  active context. This is normally a replica proxy that still runs `ProtocolConfig`.
- `is already at context …` — either canonical's rotation is not finalized yet (§5.2) or this
  replica is already up to date.

### 5.4 Mirror an epoch rotation

Same-set rotations carry no committee data, so this path is just the two ids — no event
read, no anchor check:

```bash
cd host-contracts
export PROTOCOL_CONFIG_CONTRACT_ADDRESS=<ProtocolConfigReplica proxy>

# DAO path:
npx hardhat task:buildMirrorKmsEpochCalldata --network <replica-network> \
  --canonical-rpc-url <ethereum-rpc-url> \
  --canonical-protocol-config-address <canonical ProtocolConfig>

# No-DAO path:
npx hardhat task:mirrorKmsEpoch --network <replica-network> \
  --canonical-rpc-url <ethereum-rpc-url> \
  --canonical-protocol-config-address <canonical ProtocolConfig>
```

If the replica is not on canonical's active context yet, the task refuses with
`… is at context <a>, but canonical's active context is <b>` and tells you to run
`task:mirrorKmsContextAndEpoch` first — do that (§5.3), then re-run this.

### 5.5 Verify mirroring results

```bash
CANON_PC=<canonical ProtocolConfig>;    CANON_RPC=<ethereum-rpc-url>
REPL_PC=$PROTOCOL_CONFIG_CONTRACT_ADDRESS;  REPL_RPC=<replica-rpc-url>

# The pair must be identical on both chains.
cast call $CANON_PC "getCurrentKmsContextAndEpoch()(uint256,uint256)" --rpc-url $CANON_RPC
cast call $REPL_PC  "getCurrentKmsContextAndEpoch()(uint256,uint256)" --rpc-url $REPL_RPC

# And so must the stored node set and thresholds for that context id.
cast call $REPL_PC "getKmsNodesForContext(uint256)((address,address,string,string)[])" <contextId> --rpc-url $REPL_RPC
cast call $REPL_PC "getMpcThresholdForContext(uint256)(uint256)" <contextId> --rpc-url $REPL_RPC
```

The MPC metadata (`partyId`, `mpcIdentity`, `caCert`, `storagePrefix`), software version and
PCR values are not stored on-chain — read them from the replica's
`MirrorKmsContextAndEpoch` event, where they match canonical's `NewKmsContext` exactly.
Worth knowing: the bootstrap path (`initializeFromCanonical`) could not recover them and
filled placeholders, so the first mirrored switch is where a replica's event history starts
carrying the real values.

**Upgrade precondition (ProtocolConfig v0.4.0)**: settle any in-flight lifecycle operation
first, or destroy it with `destroyKmsContext` or `destroyKmsEpoch`. A pending epoch would keep its v0.3.0
per-signer flags, so its signers could not confirm again. A pending context would have no node config
hash, so its signed creation confirmation would commit to a zero hash. An epoch or a context
destroyed before the upgrade can still collect signed destruction confirmations.
Canonical never destroys its latest active context or epoch. A mirror call copies only those
ids, so it never copies a destroyed id. `ProtocolConfigReplica` has no destruction method. A
destruction on canonical does not reach a replica that mirrored that id earlier.

`task:kmsContextSwitchStatus` is a canonical-side tool. Pointed at a replica it reports
`idle`, because it tracks the pending/quorum path that replicas never run — that is the
expected output, not a problem.

### 5.6 Retire on every replica what Ethereum destroyed

A canonical `destroyKmsContext` or `destroyKmsEpoch` does not reach the replicas: a mirrored
context stays `Active` there, and `KMSVerifier` keeps accepting its signers, until the
destruction is mirrored on each replica. For every canonical destruction, take the id from
canonical's `KmsContextDestroyed` / `KmsEpochDestroyed` event and run on each replica:

```bash
# Context destroyed — DAO path, then no-DAO path:
npx hardhat task:buildMirrorKmsContextDestructionCalldata --network <replica-network> --context-id <id>
npx hardhat task:mirrorKmsContextDestruction --network <replica-network> --context-id <id>

# Epoch destroyed — DAO path, then no-DAO path:
npx hardhat task:buildMirrorKmsEpochDestructionCalldata --network <replica-network> --epoch-id <id>
npx hardhat task:mirrorKmsEpochDestruction --network <replica-network> --epoch-id <id>
```

- **Mirror cancellations too.** The replica records the id of a cancelled switch or reshare
  as destroyed, so it never activates it later.
- **A context destruction covers its epochs.** Mirror an epoch destruction only after a
  canonical `destroyKmsEpoch`.
- **The replica refuses to destroy its active context or epoch.** That revert means the
  replica is behind: mirror the successor first (§5.3 or §5.4).

Do this with the same urgency as the canonical call: for a context destroyed because it was
compromised, every replica trusts it until this step is done.

## 6. Quick reference

```text
Host tasks (host-contracts/, reads PROTOCOL_CONFIG_CONTRACT_ADDRESS):
  task:buildDefineNewKmsContextAndEpochCalldata   DAO calldata, context switch
  task:defineNewKmsContextAndEpoch                direct broadcast (DEPLOYER_PRIVATE_KEY)
  task:buildDefineNewEpochForCurrentKmsContextCalldata   DAO calldata, epoch rotation (no-arg)
  task:defineNewEpochForCurrentKmsContext         direct broadcast
  task:buildDestroyKmsContextCalldata             DAO calldata, cancel/retire a context (--context-id)
  task:destroyKmsContext                          direct broadcast
  task:buildDestroyKmsEpochCalldata               DAO calldata, cancel/retire an epoch (--epoch-id)
  task:destroyKmsEpoch                            direct broadcast
  (Destroying a context also retires its epochs, so no destroyKmsEpoch call is needed
  after destroyKmsContext. Use destroyKmsEpoch only for an epoch whose context stays live.)
  task:kmsContextSwitchStatus                     read-only progress (--from-block!)

Replica mirror tasks (host-contracts/, run on each non-canonical chain after the canonical
operation; the context-switch and epoch tasks take --canonical-rpc-url +
--canonical-protocol-config-address, optional --block-number):
  task:buildMirrorKmsContextAndEpochCalldata      DAO calldata, mirror a context switch
  task:mirrorKmsContextAndEpoch                   direct broadcast
  task:buildMirrorKmsEpochCalldata                DAO calldata, mirror an epoch rotation
  task:mirrorKmsEpoch                             direct broadcast
  task:buildMirrorKmsContextDestructionCalldata   DAO calldata, mirror a context destruction (--context-id)
  task:mirrorKmsContextDestruction                direct broadcast
  task:buildMirrorKmsEpochDestructionCalldata     DAO calldata, mirror an epoch destruction (--epoch-id)
  task:mirrorKmsEpochDestruction                  direct broadcast

Gateway tasks (gateway-contracts/, reads GATEWAY_CONFIG_ADDRESS, KMS_CONTEXT_ID):
  task:buildUpdateKmsContextProposal              DAO proposal triple (--verify-context-id)
  task:updateKmsContext                           direct broadcast

Quorum cheat-sheet:
  context creation  = ALL new signers + (n - t) old signers, on one digest
  epoch activation  = ALL new-committee signers, unanimous on one digest
  destruction       = (n - t) active-committee signers, on one digest, emits a QuorumReached event
  committee size    = 3 * MPC_THRESHOLD + 1, party ids contiguous 1..n
```

---

## Annex A — Concepts and invariants

- **Context** = a KMS committee (node set + thresholds + software version + PCR values),
  identified by a domain-tagged id (`0x07…NN`). **Epoch** = one key-material generation
  under a context (`0x08…NN`). `getCurrentKmsContextAndEpoch()` returns the active pair.
- **Lifecycle states**: context `Pending → Created → Active`; epoch `Pending → Active`.
  Governance only _opens_ a switch. The KMS connectors do not submit context creation or
  epoch activation confirmations in this version (planning-blockchain#1369). The connectors
  have no code that submits the two destruction confirmations. No other service submits them.
- **Node identity is a `(txSenderAddress, signerAddress)` pair.**
  All four confirmation functions (`confirmKmsContextCreation`, `confirmEpochActivation`,
  `confirmKmsContextDestruction`, `confirmKmsEpochDestruction`) authenticate the node by
  the EIP-712 signature of its **signer** (RFC 037). Anyone may submit the signature, and
  the contract deduplicates and counts **by signer**.
- **Destruction confirmations**: after `destroyKmsContext` or `destroyKmsEpoch`, each signer
  of the active committee signs a `ContextDestructionConfirmation` or
  `EpochDestructionConfirmation`. Each signer confirms once per destroyed id. The canonical
  contract counts the confirmations per destroyed id and digest. When `n − t` signers of
  the active committee confirm one digest, it emits
  `KmsContextDestructionQuorumReached` or `KmsEpochDestructionQuorumReached` once for that
  digest. The DAO does not rotate before a destruction reaches the quorum (RFC 037).
- **Quorums**:
  - Context creation: **ALL new-committee signers** + **(n − t)
    previous-committee signers** (n = previous committee size, t = previous MPC
    threshold; floored at 1). More than `t` so faulty nodes can never approve a
    switch alone; at most `n − t` because anything higher would let a dead node
    block the switch forever. Under the `n = 3t + 1` topology this quorum works
    out to `n − t = (3t + 1) − t = 2t + 1` — a supermajority of the previous
    committee.
  - The MPC reshare needs the same `n − t` old-committee senders, so both phases
    tolerate up to **t** dead-or-silent previous nodes.
  - Epoch activation: **unanimous**. Every new-committee signer must sign the _same_
    digest (`epochMaterialHash` and `extraData`). One divergent signature splits the confirmations.
    Each signer confirms once, so the epoch stays Pending until governance settles it. Use `destroyKmsEpoch` for a same-set reshare. Use
    `destroyKmsContext` for a context switch, because `destroyKmsEpoch` reverts there.
- **One switch in flight at a time, enforced by the contract.**
  `defineNewKmsContextAndEpoch` / `defineNewEpochForCurrentKmsContext` revert with
  `KmsLifecycleOperationInFlight` while another switch or reshare is in flight. Settle the
  in-flight one first. Complete it, or cancel it. Cancel a context switch with
  `destroyKmsContext` and a same-set reshare with `destroyKmsEpoch`.
- **Destroying a context retires its epochs with it.** `destroyKmsContext` clears only a
  paired epoch that is still `Pending`. The earlier epoch of a retired context keeps the
  `Active` value in storage, but nothing can use it: the only view that reports an epoch,
  `isValidEpochForContext`, requires its context to be live, and so do the KMS verifier and
  the connectors. No action is needed for that epoch. On replicas the context destruction
  must be mirrored too (§5.6).
- **Party IDs are positional and contiguous `1..n`.** The KMS core rejects any id outside
  that range. In a node swap, the incoming node **takes the dropped node's party id** —
  only the identity fields (addresses, `mpcIdentity`, cert, storage prefix) change.
- **Canonical vs replica.** The lifecycle above exists only on the canonical host
  (Ethereum). Other host chains run `ProtocolConfigReplica`, a read-replica: no quorum
  path, no `KMSGeneration`, and the owner mirror methods as its only write path — `mirrorKmsContextAndEpoch` /
  `mirrorKmsEpoch`, which import already-finalized canonical state as immediately `Active`,
  and `mirrorKmsContextDestruction` / `mirrorKmsEpochDestruction`, which copy canonical
  destructions. The only on-chain guard on activation is that ids strictly increase, which
  prevents a rollback but not a replica left behind. Keeping every replica current, and
  mirroring every destruction, is operational discipline (§5).
- **`mpcIdentity` must equal the identity the core itself is configured with** (its
  `mpc_identity` in the core/peers config). A mismatch makes the reshare role-resolution
  fail on the on-chain-vs-config merge and the reshare never starts (observed failure:
  PRSS "number of parties ≤ threshold").

## Annex B — Commands and scripts

All host-side reads work with `cast` against the `ProtocolConfig` proxy
(`PROTOCOL_CONFIG_CONTRACT_ADDRESS`).

### B.1 Read the live state

> **Run by** anyone · **needs** `cast` + host RPC url (the hardhat task also needs the
> `host-contracts` repo) · **when** before, during and after any operation ·
> **access** read-only `eth_call`/event queries — no keys, no gas, no writes.

```bash
PC=$PROTOCOL_CONFIG_CONTRACT_ADDRESS
RPC=<host-chain-rpc-url>

# Active context + epoch (domain-tagged uint256s: 0x07…, 0x08…)
cast call $PC "getCurrentKmsContextAndEpoch()(uint256,uint256)" --rpc-url $RPC

# Current committee full node records for a context id
cast call $PC "getKmsNodesForContext(uint256)((address,address,string,string)[])" <contextId> --rpc-url $RPC

# Anchor hash for a context (used to cross-check reinitialize inputs)
cast call $PC "getKmsContextAnchor(uint256)(uint256,bytes32)" <contextId> --rpc-url $RPC
```

Richer, event-indexed progress view (states, outstanding confirmations, destroyed-switch detection):

```bash
cd host-contracts
npx hardhat task:kmsContextSwitchStatus --network <network> --from-block <recent-block>
# flow: idle (nothing in flight) | context-switch (pending context newer than the
#       active one) | same-set-rotation (new epoch under the same active context)
# contextState: PENDING (lists outstanding new signers + the n-t old-side target)
#               CREATED (creation quorum reached, epoch still PENDING)
# epochState:   PENDING | DESTROYED (an activated epoch shows as flow: idle)
# aborted:      true + reason if governance destroyed the pending context or epoch
#               (context-destroyed via destroyKmsContext, epoch-destroyed via destroyKmsEpoch)
# fullyLive:    true when the active context and epoch are valid and nothing is in flight
# destruction:  active-committee destruction confirmations of a destroyed context or
#               epoch, against the n-t target. quorumReached is read from the
#               contract's *DestructionQuorumReached event
```

### B.2 Verify a party's claimed parameters (coordinator-side)

```bash
#!/usr/bin/env bash
# verify-kms-party.sh — cross-check one party's questionnaire answers against
# their public storage.
# Run by:  the coordinator, once per party (a party may self-check its own vault)
# When:    after the questionnaire replies, before assembling committee.env
# Access:  outbound GETs to the party's PUBLIC vault only — no keys, no chain
#          access, no filesystem writes. Exit 0 = all checks passed.
# Usage:   ./verify-kms-party.sh <storageUrl> <storagePrefix> <claimedSigner>
#          (after review: chmod 500 verify-kms-party.sh — the script needs no write bit)
set -euo pipefail

CURL=(curl -sfS --max-time 30)

STORAGE_URL=${1:?usage: verify-kms-party.sh <storageUrl> <storagePrefix> <claimedSigner>}
PREFIX=${2:?missing storagePrefix}
CLAIMED_SIGNER=$(printf '%s' "${3:?missing claimedSigner}" | tr '[:upper:]' '[:lower:]')
[[ "$CLAIMED_SIGNER" =~ ^0x[0-9a-f]{40}$ ]] || {
  echo "FAIL: claimed signer is not a 20-byte hex address"; exit 1; }

# The core publishes one object per key handle under VerfAddress/ and CACert/.
# Object keys are whitelisted to a safe charset before being used or printed.
echo "--- objects under $PREFIX ---"
HANDLES=$("${CURL[@]}" "$STORAGE_URL/?prefix=$PREFIX/VerfAddress/&list-type=2" \
  | grep -oE '<Key>[^<]+</Key>' | sed -E 's#</?Key>##g' \
  | grep -E '^[A-Za-z0-9._/-]+$') || {
    echo "FAIL: cannot list $STORAGE_URL under $PREFIX"; exit 1; }
echo "$HANDLES"

MATCHED=0
for h in $HANDLES; do
  ADDR=$("${CURL[@]}" "$STORAGE_URL/$h" | tr -d '[:space:]' | tr '[:upper:]' '[:lower:]')
  [[ "$ADDR" =~ ^0x[0-9a-f]{40}$ ]] || {
    echo "FAIL: $h does not contain an address"; exit 1; }
  echo "VerfAddress ($h): $ADDR"
  [ "$ADDR" = "$CLAIMED_SIGNER" ] && MATCHED=1
done
[ "$MATCHED" -eq 1 ] || { echo "FAIL: no published VerfAddress matches $CLAIMED_SIGNER"; exit 1; }
echo "OK: signer matches claim"

# CA cert: capture the byte-exact hex used on-chain (bytes caCert), then parse the
# cert from that hex — nothing is written to disk.
CERT_KEY=$("${CURL[@]}" "$STORAGE_URL/?prefix=$PREFIX/CACert/&list-type=2" \
  | grep -oE '<Key>[^<]+</Key>' | sed -E 's#</?Key>##g' \
  | grep -E '^[A-Za-z0-9._/-]+$' | head -1)
[ -n "$CERT_KEY" ] || { echo "FAIL: no object under $PREFIX/CACert/"; exit 1; }
CERT_HEX=$("${CURL[@]}" "$STORAGE_URL/$CERT_KEY" | xxd -p | tr -d '\n')
printf '%s' "$CERT_HEX" | xxd -r -p | openssl x509 -noout -subject -dates -fingerprint
printf 'KMS_NODE_CA_CERT hex: 0x%s\n' "$CERT_HEX"
```

The `subject=` / `notBefore=` / `Fingerprint=` lines are for review (the script only
fails if the cert is missing or unparsable):

- **subject** — typically embeds the party's MPC identity; compare it with the
  `mpc_identity` they pasted (do not _derive_ the identity from it; ask for the
  config value);
- **dates** — `notAfter` must outlive the planned context (the committee's mutual
  TLS runs on this CA; an expired cert is a committee-wide outage);
- **fingerprint** — confirm it with the party over a trusted channel; that catches a
  tampered vault object, which the fetch alone cannot.

The machine-consumed output is the final `KMS_NODE_CA_CERT hex: 0x…` line — paste it
verbatim into `committee.env`.

### B.3 Assemble and validate the env file (coordinator-side)

The host and gateway tasks read the committee from indexed `KMS_*` env vars. Build one
env file per operation and validate it before broadcasting:

```bash
# committee.env — one block per node, index 0..NUM_KMS_NODES-1
NUM_KMS_NODES=4
KMS_TX_SENDER_ADDRESS_0=0x…
KMS_SIGNER_ADDRESS_0=0x…
KMS_NODE_IP_0=http://…:50001            # host task name
KMS_NODE_IP_ADDRESS_0=http://…:50001    # gateway task name (same value!)
KMS_NODE_STORAGE_URL_0=https://…
KMS_NODE_PARTY_ID_0=1
KMS_NODE_MPC_IDENTITY_0=…
KMS_NODE_CA_CERT_0=0x…
KMS_NODE_STORAGE_PREFIX_0=PUB-p1
# … repeat for _1, _2, _3 …
PUBLIC_DECRYPTION_THRESHOLD=3
USER_DECRYPTION_THRESHOLD=3
MPC_THRESHOLD=1
KMS_GEN_THRESHOLD=3
KMS_GENERATION_THRESHOLD=3              # gateway name for KMS_GEN_THRESHOLD (same value!)
KMS_SOFTWARE_VERSION=…
# single quotes required: the file is also `source`d, which would strip inner double quotes
KMS_PCR_VALUES='[{"pcr0":"0x…","pcr1":"0x…","pcr2":"0x…"}]'
```

> **Two env-naming traps** (the host and gateway repos differ):
> host `KMS_NODE_IP_{i}` vs gateway `KMS_NODE_IP_ADDRESS_{i}`, and
> host `KMS_GEN_THRESHOLD` vs gateway `KMS_GENERATION_THRESHOLD`. Set both spellings.

```bash
#!/usr/bin/env bash
# validate-committee-env.sh — structural checks on committee.env before any broadcast.
# Run by:  the coordinator
# When:    last step before broadcasting, and again after every edit
# Access:  reads the env file only — no network, no chain, no keys, no writes.
#          The env file is SOURCED (executed): only run this on a committee.env
#          you assembled yourself, never on a file received from a third party.
# Usage:   ./validate-committee-env.sh committee.env
#          (after review: chmod 500 validate-committee-env.sh — the script needs no write bit)
set -euo pipefail
# shellcheck disable=SC1090
source "${1:?usage: validate-committee-env.sh committee.env}"

fail() { echo "FAIL: $*"; exit 1; }

int_re='^[0-9]+$'
# Numeric committee-level values: set AND integer, checked before any arithmetic.
for v in NUM_KMS_NODES MPC_THRESHOLD PUBLIC_DECRYPTION_THRESHOLD \
         USER_DECRYPTION_THRESHOLD KMS_GEN_THRESHOLD; do
  [[ "${!v:-}" =~ $int_re ]] || fail "$v must be set to an integer (got '${!v:-}')"
done
for v in KMS_SOFTWARE_VERSION KMS_PCR_VALUES; do
  [ -n "${!v:-}" ] || fail "$v is unset"
done

n=$NUM_KMS_NODES
[ "$n" -eq "$((3 * MPC_THRESHOLD + 1))" ] || fail "n must be 3*MPC_THRESHOLD+1"

# The KMS core ingests only MPC_THRESHOLD: a threshold of t or less lets t corrupt nodes meet it.
for v in PUBLIC_DECRYPTION_THRESHOLD USER_DECRYPTION_THRESHOLD KMS_GEN_THRESHOLD; do
  [ "${!v}" -gt "$MPC_THRESHOLD" ] || fail "$v must be greater than MPC_THRESHOLD (got ${!v})"
done

# Gateway spelling must be present and equal to its host twin.
[ "${KMS_GENERATION_THRESHOLD:-}" = "$KMS_GEN_THRESHOLD" ] ||
  fail "KMS_GENERATION_THRESHOLD (gateway spelling) must equal KMS_GEN_THRESHOLD"

ids=(); txs=(); signers=(); mpcs=()
for i in $(seq 0 $((n - 1))); do
  for v in KMS_TX_SENDER_ADDRESS_$i KMS_SIGNER_ADDRESS_$i KMS_NODE_IP_$i \
           KMS_NODE_STORAGE_URL_$i KMS_NODE_PARTY_ID_$i KMS_NODE_MPC_IDENTITY_$i \
           KMS_NODE_CA_CERT_$i KMS_NODE_STORAGE_PREFIX_$i; do
    [ -n "${!v:-}" ] || fail "$v is unset"
  done
  # Gateway spelling must be present and equal to its host twin.
  v=KMS_NODE_IP_$i; g=KMS_NODE_IP_ADDRESS_$i
  [ "${!g:-}" = "${!v}" ] || fail "$g (gateway spelling) must equal $v"
  for v in KMS_TX_SENDER_ADDRESS_$i KMS_SIGNER_ADDRESS_$i; do
    [[ "${!v}" =~ ^0x[0-9a-fA-F]{40}$ ]] || fail "$v is not a 20-byte hex address"
  done
  v=KMS_NODE_CA_CERT_$i
  [[ "${!v}" =~ ^0x([0-9a-fA-F]{2})+$ ]] || fail "$v is not 0x-prefixed hex bytes"
  v=KMS_NODE_PARTY_ID_$i
  [[ "${!v}" =~ $int_re ]] || fail "$v must be an integer (got '${!v}')"
  ids+=("${!v}")
  v=KMS_TX_SENDER_ADDRESS_$i; txs+=("${!v}")
  v=KMS_SIGNER_ADDRESS_$i;    signers+=("${!v}")
  v=KMS_NODE_MPC_IDENTITY_$i; mpcs+=("${!v}")
done

# Party ids must be exactly the set {1..n} (contiguous, positional).
sorted=$(printf '%s\n' "${ids[@]}" | sort -n | tr '\n' ' ')
expect=$(seq 1 "$n" | tr '\n' ' ')
[ "$sorted" = "$expect" ] || fail "party ids {${ids[*]}} != {1..$n}"

# No duplicate identities across nodes.
check_dupes() {
  local label=$1; shift
  local dupes
  dupes=$(printf '%s\n' "$@" | tr '[:upper:]' '[:lower:]' | sort | uniq -d)
  [ -z "$dupes" ] || fail "duplicate $label: $dupes"
}
check_dupes "tx-sender addresses" "${txs[@]}"
check_dupes "signer addresses" "${signers[@]}"
check_dupes "mpc identities" "${mpcs[@]}"

echo "OK: $n nodes, ids contiguous 1..$n, no duplicate identities, gateway spellings consistent"
```
