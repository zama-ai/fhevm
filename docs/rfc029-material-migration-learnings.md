# RFC-029 Material Migration Learnings

This note is a handoff for a future clean implementation of the one-time
compressed-key material migration. It records the end-to-end behavior, the
constraints that mattered, and where the current PR drifted from RFC-029.

## Outcome

The feature is about preparing live Zama networks for GPU-compatible key
material without changing the active FHE key id and without breaking byte-level
ciphertext consensus.

The live network today uses CPU coprocessors and legacy server-key material.
GPU production is not live yet. The GPU path exists in CI and test
environments, where the system can start with compressed XOF key material from
the beginning. The production migration is different: it introduces compressed
material for the already-active key while live CPU coprocessors continue to run
for some time.

Consensus is the core constraint. The old and new key material are compatible
with the same secret key, but the concrete ciphertext bytes can differ. A
correct implementation must make every coprocessor choose the same material for
the same operation from consensus-visible data, never from local ingestion
timing or table recency.

## Current Material Cases

There are several distinct states that were easy to confuse:

- Existing live production/testnet rows: `compressed_xof_keyset` is empty.
  Coprocessors use legacy `sks_key`. This is the current deployed state.
- GPU CI/native compressed-key rows: `compressed_xof_keyset` is populated from
  the beginning. This is not a migration state. The test expects the normal key
  path to use compressed material because there is no live legacy-to-compressed
  cutover in progress.
- Migration material published but not scheduled: compressed material exists for
  the active key, but live consensus must still behave as if the migration has
  not activated. The presence of compressed bytes alone cannot mean "use them."
- Scheduled but pre-boundary work: both materials exist, but operations anchored
  before their boundary still use legacy material.
- Post-boundary work: operations anchored at or after their boundary use the new
  material. CPU workers may still execute by decompressing the compressed XOF
  material; GPU production can come later.
- Long-tail SnS work: old ciphertexts can become SnS-eligible long after the
  migration. Old material must remain resolvable for that path.

The important lesson is that "compressed bytes exist" and "compressed bytes are
the selected consensus material" are not the same event on live networks.

## End-To-End Flow

Governance first asks KMS to produce compressed material for the existing active
key. The key id does not change. The secret key does not change. The material
bytes do change.

KMS and the connector produce or publish a KMS-signed result for that existing
key. We went back and forth on whether this should be expressed as a distinct
migration keygen path or as a branch inside the normal keygen path. The
important distinction is lifecycle semantics: this operation is not normal key
activation and must not be mistaken for key rotation.

Coprocessors ingest finalized material publication, download the compressed
material, verify it according to the same trust model used for key material, and
store it locally. At this point the material is available, but not necessarily
selected by live operations.

Governance then schedules the cutover boundaries. Host-chain compute is selected
by host-chain block number. Input verification is selected by Gateway event
block number. Switch-and-Squash is selected by the source ciphertext material
label because SnS has no block context of its own.

Workers evaluate material selection per task. If the selected material is not
available, the worker must stop and retry rather than substitute the other
material. Fallback would turn material unavailability into byte divergence.

## RFC-029 Versus This PR

RFC-029 describes the migration as a material-version change under one key id,
with `materialVersion 0` for legacy bytes and `materialVersion 1` for the new
format. It emphasizes that the version is internal to coprocessors and must not
appear in public request identities, handles, seeds, attestations, or decryption
flows.

The RFC places the cutover record in `ProtocolConfig` as the canonical stored
Ethereum record. This PR evolved toward `KMSGeneration` holding the relevant
governance surface. The motivation was local component ownership: the feature
was repeatedly discussed as KMS generation material lifecycle rather than
multi-host protocol configuration.

The RFC specifies `KMSGeneration.addKeyMaterials(keyId, materialVersion, ...)`
as an append of material-version-1 digests for an existing key. The PR explored
several alternatives: direct material publication without KMS consensus, using
normal keygen with migration context, typed migration keygen events, split
migration responses, and eventually a clearer typed migration response path.
The unresolved design question is how much distinct surface is worth keeping
versus reusing normal keygen response mechanics internally.

The RFC says material append does not emit `ActivateKey` and does not change
`activeKeyId`. This remained a hard constraint in the PR discussion. Using
normal activation semantics for migration was considered risky because it makes
"activate" mean two different lifecycle outcomes.

The RFC expects coprocessors to store and load material by `(keyId,
materialVersion)`. The PR ultimately moved toward a single physical home for
compressed material in `keys.compressed_xof_keyset`, because the existing schema
already had that column and a second migrated column made the read path harder
to reason about.

The RFC asks SnS tasks to copy the source ciphertext material version onto the
task row at enqueue time. The PR at one point relied on joining source
ciphertexts when executing SnS. That was reviewed as acceptable only under
current storage invariants, but it is a meaningful difference from the RFC
because the RFC treats task-local pinning as the authority.

The RFC specifies first-finalized-request-wins canonical input storage for
cross-boundary identical input blobs. The PR discussion largely treated the
existing `ON CONFLICT DO NOTHING` canonicalization as sufficient only because
material selection is derived from the finalized Gateway block and duplicates
should resolve to the same canonical choice. This area should be read carefully
against the exact current schema.

The RFC assumes manual orchestration and monitoring are acceptable. The PR
temporarily accumulated staging tables, schedule promotion, orphan cancellation,
and reorg-aware logic. Later review pushed back on that complexity and favored
finalized-only event ingestion for this one-time migration.

## Component Responsibilities

KMSGeneration is where governance intent and KMS material publication became
visible in the PR. The ambiguity was whether migration should be a first-class
contract path or a clearly marked branch of the existing keygen response path.
The hard constraint is that migration must not be confused with key rotation.

KMS connector receives the chain-visible request, coordinates with KMS core, and
publishes the response. We learned that duplicating the whole connector flow for
migration is heavy, but hiding migration inside `extraData` or implicit keygen
context is also hard to audit.

Host listener / coprocessor ingestion consumes finalized events and updates the
local database. The listener-core did not provide finalized-only event delivery
at the time of the PR. The team discussed moving this concern into listener-core
but considered it too much for this feature. The practical learning was that
this one-off feature should avoid owning generic reorg machinery if finalized
polling can provide the needed event.

TFHE worker produces ciphertexts from host-chain computations. Its selection
input is host chain id plus host block number. We hit complexity when batches
were grouped by material version with nondeterministic map iteration; the
learning is that boundary-straddling work must stay deterministic and easy to
read.

ZK proof worker produces input ciphertexts from Gateway requests. Its selection
input is the finalized Gateway block number. It must treat unavailable selected
material as retryable unavailability, not as a reason to use legacy material.

SnS worker has no independent block boundary. It follows the material label of
the source ciphertext. Old-material SnS must remain possible indefinitely.

The key cache stores runtime key objects, not just raw database bytes. A
compressed XOF keyset decompresses to the `tfhe::ServerKey` used by CPU code,
and GPU code can also derive GPU server keys from the compressed material. The
cache became confusing when the migration label leaked into public APIs; the
useful learning is that boundary-specific cache details should stay internal to
the key-loading layer.

## Decisions That Changed

We started close to the RFC language: material versions, versioned schedules,
and versioned key material.

We then discovered that treating the feature as a generic material-version
framework made the PR hard to review. The feature is one-time and specifically
about compressed XOF material for GPU enablement.

We considered a separate `migrated_xof_keyset` column. That made it easier to
tell staged migration material apart from native compressed material, but it
created two homes for the same compressed key material and made future reads
less natural.

We considered staging schedule application and adding cancellation/orphan logic.
That made the coprocessor own too much lifecycle machinery for a one-off
finalized event.

We considered using normal keygen and activation paths. Reusing mechanics looked
attractive, but overloading activation semantics was concerning because
activation normally means changing the active key, while RFC-029 explicitly does
not.

We introduced a typed migration keygen path to make intent clear. Later we
questioned whether the typed surface had grown too large and whether the normal
keygen response internals could be reused while keeping the external lifecycle
clear.

We renamed coprocessor concepts away from broad "material migration" wording
toward key-material policy and legacy-key cutover wording. That was a readability
reaction: most future code should read as default key material with temporary
legacy override, not as a permanent migration framework.

## Constraints To Preserve

- Same key id throughout the migration.
- No key rotation semantics.
- No SDK, relayer, request-handle, seed, attestation, or decryption-flow changes.
- Material selection must be derived from finalized, consensus-visible inputs.
- Local ingestion timing must never decide material selection.
- The presence of compressed bytes in the database is not enough to select them
  during live migration.
- GPU CI/native compressed-key startup is a separate case from live migration.
- Post-boundary unavailability must halt or retry, not downgrade.
- Old material must remain available for long-tail SnS.
- Generated bindings and WASM artifacts are not the design signal when judging
  PR complexity.

## Code Map For Orientation

The next agent should use this map to understand the current PR attempt, not to
report back with a code-structure summary. The expected output should be about
design: whether the flow, responsibilities, lifecycle semantics, and consensus
boundaries are right.

Contract and governance surface:

- `host-contracts/contracts/KMSGeneration.sol`: chain-visible governance and
  publication surface for key generation, migration key generation, key
  material addition, and scheduling.
- `host-contracts/contracts/interfaces/IKMSGeneration.sol`: interface shape
  that downstream components compile against.
- `host-contracts/test/kmsGeneration/kmsGeneration.t.sol`: contract-level tests
  for the added migration behavior.
- `host-contracts/tasks/migrateKeyMaterials.ts`: operational task glue used by
  the rollout to drive the migration through the contract.
- `host-contracts/rust_bindings/src/kms_generation.rs` and
  `host-contracts/rust_bindings/src/ikms_generation.rs`: generated ABI bindings;
  useful to confirm event/function shapes, not useful as design evidence.

KMS connector path:

- `kms-connector/connector-db/migrations/20260630115900_migration_keygen_event_type.sql`
  and `20260630120000_migration_keygen_requests.sql`: connector persistence for
  migration keygen request tracking.
- `kms-connector/crates/gw-listener/src/core/ethereum.rs` and `publish.rs`:
  where Gateway/Ethereum events enter the connector request flow.
- `kms-connector/crates/kms-worker/src/core/event_picker/*`: selection and
  notification of pending KMS work.
- `kms-connector/crates/kms-worker/src/core/event_processor/kms.rs` and
  `processor.rs`: conversion from picked events to KMS-core work.
- `kms-connector/crates/kms-worker/src/core/kms_response_publisher.rs`: response
  publication path back on-chain.
- `kms-connector/crates/tx-sender/src/core/ethereum.rs` and
  `kms_response_picker/picker.rs`: transaction-sending side of the response
  lifecycle.
- `kms-connector/crates/utils/src/types/*` and `tests/db/requests.rs`: shared
  request/response/event types and DB helpers.

Coprocessor database and ingestion:

- `coprocessor/fhevm-engine/db-migration/migrations/20260624120000_material_version.sql`:
  database shape for ciphertext material labels, Gateway block context, local
  compressed-key state, and cutover storage.
- `coprocessor/fhevm-engine/db-migration/migrations/20260625120000_key_material_events.sql`:
  finalized key-material download queue.
- `coprocessor/fhevm-engine/gw-listener/src/gw_listener.rs`: where Gateway
  block metadata is persisted for verification requests.
- `coprocessor/fhevm-engine/host-listener/src/database/ingest.rs`: finalized
  event ingestion entrypoint.
- `coprocessor/fhevm-engine/host-listener/src/kms_generation/database.rs` and
  `mod.rs`: KMSGeneration event handling, material download queueing, local key
  storage update, and cutover ingestion.
- `coprocessor/fhevm-engine/.sqlx/*.json`: generated SQLx metadata; useful for
  build consistency, not design evidence.

Coprocessor key loading and selection:

- `coprocessor/fhevm-engine/fhevm-engine-common/src/key_material_policy.rs`:
  central material-selection policy and vocabulary used by workers.
- `coprocessor/fhevm-engine/fhevm-engine-common/src/db_keys.rs`: key loading,
  deserialization/decompression, GPU-vs-CPU key material handling, and cache
  behavior.
- `coprocessor/fhevm-engine/fhevm-engine-common/src/lib.rs`: module export only.

Worker behavior:

- `coprocessor/fhevm-engine/tfhe-worker/src/tfhe_worker.rs`: host-chain compute
  selection, execution grouping, and ciphertext material labeling.
- `coprocessor/fhevm-engine/zkproof-worker/src/verifier.rs`: Gateway-block input
  selection and input ciphertext material labeling.
- `coprocessor/fhevm-engine/sns-worker/src/executor.rs`: SnS execution behavior
  around source material labels.
- `coprocessor/fhevm-engine/sns-worker/src/keyset.rs`: SnS keyset loading for
  default and compressed material.
- `coprocessor/fhevm-engine/sns-worker/src/aws_upload.rs` and `lib.rs`: SnS
  output plumbing and task data shape touched by the migration.
- `coprocessor/fhevm-engine/tfhe-worker/src/tests/db_key_cache.rs`: cache/read
  regression coverage.

Rollout and CI proof:

- `test-suite/fhevm/rollouts/rfc029-material-migration/run.ts`: end-to-end
  rollout driver.
- `test-suite/fhevm/rollouts/rfc029-material-migration/versions.ts`: pinned
  image/version inputs for the rollout.
- `test-suite/fhevm/scenarios/rfc029-cutover.yaml`: rollout scenario.
- `test-suite/fhevm/src/rollout-rfc029.test.ts`: test-suite entrypoint.
- `.github/workflows/test-suite-stateful-rollout.yml`: CI workflow hook for the
  rollout.
- `test-suite/fhevm/templates/env/.env.kms-connector`: connector environment
  inputs needed by the rollout.

## What A Future Agent Should Read

- RFC-029 in `zama-ai/tech-spec#478`.
- RFC-028 for compressed keygen context.
- PR #2902 history for implementation attempts and rollback points.
- The Slack thread around finalized listener support:
  `https://zama-ai.slack.com/archives/C06D8AVHL0N/p1782811539972849`.
- Coprocessor and `guild-copro-gpu` Slack discussion around GPU rollout and
  the distinction between GPU CI and live production migration.
- Current repo code around KMSGeneration, kms-connector keygen/response
  handling, host-listener KMS generation ingestion, the three coprocessor
  workers, and key-cache loading.
