# GPU Coprocessor E2E Runbook

This runbook records the steps used to prepare H100 hosts for GPU coprocessor
worker testing, run the FHEVM e2e stack, and validate the current compressed XOF
keyset state.

The reference CI configuration is:

- `.github/actions/gpu_setup/action.yml`
- `.github/workflows/coprocessor-gpu-tests.yml`
- `.github/workflows/coprocessor-benchmark-gpu.yml`

## Host Setup

The CI GPU jobs use Ubuntu 22.04, CUDA 12.2, and GCC 11.

Install the base packages used by CI:

```bash
sudo apt update
sudo apt install -y \
  acl \
  cmake \
  cmake-format \
  docker-compose-v2 \
  docker.io \
  git-lfs \
  libclang-dev \
  libssl-dev \
  pkg-config \
  protobuf-compiler
```

For `tfhe-cuda-backend v0.14.0`, Ubuntu 22.04's apt CMake 3.22 is too old.
Install a newer CMake and keep `/usr/local/bin` ahead of `/usr/bin`:

```bash
cd /tmp
curl -fsSLO https://github.com/Kitware/CMake/releases/download/v3.30.8/cmake-3.30.8-linux-x86_64.tar.gz
sudo tar -xzf cmake-3.30.8-linux-x86_64.tar.gz -C /opt
sudo ln -sfn /opt/cmake-3.30.8-linux-x86_64/bin/cmake /usr/local/bin/cmake
sudo ln -sfn /opt/cmake-3.30.8-linux-x86_64/bin/ctest /usr/local/bin/ctest
cmake --version
```

Install CUDA 12.2 if it is not already present:

```bash
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-keyring_1.1-1_all.deb
sudo dpkg -i cuda-keyring_1.1-1_all.deb
sudo apt update
sudo apt -y install cuda-toolkit-12-2
nvidia-smi
```

Allow the current user to use Docker:

```bash
sudo usermod -aG docker "$USER"
sudo setfacl --modify user:"$USER":rw /var/run/docker.sock
```

Install Rust, SQLx CLI, Foundry, Bun, and safe-chain:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
cargo install sqlx-cli --version 0.7.2 --no-default-features --features postgres --locked

curl -L https://foundry.paradigm.xyz | bash
"$HOME/.foundry/bin/foundryup"

curl -fsSL https://bun.sh/install | bash

curl -fsSL https://github.com/AikidoSec/safe-chain/releases/download/1.5.3/install-safe-chain.sh | sh -s -- --ci
```

Export the GPU build environment:

```bash
source "$HOME/.cargo/env"
export CUDA_PATH=/usr/local/cuda-12.2
export PATH="/usr/local/bin:$HOME/.safe-chain/shims:$HOME/.safe-chain/bin:$HOME/.foundry/bin:$HOME/.cargo/bin:${CUDA_PATH}/bin:$HOME/.bun/bin:$PATH"
export LD_LIBRARY_PATH="${CUDA_PATH}/lib64:${LD_LIBRARY_PATH:-}"
export CUDA_MODULE_LOADER=EAGER
export CC=/usr/bin/gcc-11
export CXX=/usr/bin/g++-11
export CUDAHOSTCXX=/usr/bin/g++-11
export RUSTFLAGS="-C target-cpu=native"
export SQLX_OFFLINE=true
```

On the first H100 host, `/ephemeral` was used for disposable build caches, Docker
build scratch space, or image build contexts when the root disk was tight. Do not
store source code or artifacts that must survive the session there. On the
2026-07-08 replacement host there are two NVIDIA H100 PCIe GPUs and about 500GB
of root disk, so use the root volume normally; `/ephemeral` is optional scratch
only. This note is also recorded in `/home/ubuntu/.codex/config.toml`.

## Registry Access

The e2e stack pulls images from GHCR. Log in before starting the stack:

```bash
docker login ghcr.io
```

Local image builds may also need access to `cgr.dev/zama.ai/postgres:17`. If that
registry is not authenticated, use published GHCR images instead of `--build`.

## Build And Test GPU Workers

From the coprocessor engine directory:

```bash
cd /home/ubuntu/fhevm/coprocessor/fhevm-engine
source "$HOME/.cargo/env"
export CUDA_PATH=/usr/local/cuda-12.2
export PATH="$HOME/.safe-chain/shims:$HOME/.foundry/bin:$HOME/.cargo/bin:${CUDA_PATH}/bin:$HOME/.bun/bin:$PATH"
export LD_LIBRARY_PATH="${CUDA_PATH}/lib64:${LD_LIBRARY_PATH:-}"
export CUDA_MODULE_LOADER=EAGER
export CC=/usr/bin/gcc-11
export CXX=/usr/bin/g++-11
export CUDAHOSTCXX=/usr/bin/g++-11
export RUSTFLAGS="-C target-cpu=native"
export SQLX_OFFLINE=true

cargo test \
  -p tfhe-worker \
  -p sns-worker \
  -p zkproof-worker \
  --release \
  --features=gpu \
  -- \
  --test-threads=1
```

Build the worker binaries:

```bash
cargo build \
  -p tfhe-worker \
  -p sns-worker \
  -p zkproof-worker \
  --release \
  --features=gpu \
  --bins
```

Expected binaries:

```text
coprocessor/fhevm-engine/target/release/tfhe_worker
coprocessor/fhevm-engine/target/release/zkproof_worker
coprocessor/fhevm-engine/target/release/sns_worker
```

The GPU unit test result from this H100 host was:

- SNS worker: 13 passed.
- TFHE worker: 82 passed.
- ZK proof worker: 8 passed.

## Start The E2E Stack

Use the test-suite CLI with the published images:

```bash
cd /home/ubuntu/fhevm/test-suite/fhevm
export PATH="$HOME/.bun/bin:$PATH"
./fhevm-cli up --target sha --sha 519c60c --scenario multi-chain
```

Baseline CPU/container-worker e2e check:

```bash
./fhevm-cli test light
```

The baseline result from this host passed:

- `input-proof`
- `erc20`
- `light`

## Run Host GPU Workers Against The Stack

The published Docker worker containers can be replaced with host-built GPU
binaries for testing. Keep all other stack containers running.

Stop the Docker workers:

```bash
docker stop \
  coprocessor-tfhe-worker \
  coprocessor-zkproof-worker \
  coprocessor-sns-worker
```

Use host-local service endpoints:

```bash
export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/coprocessor
export RPC_URL=http://localhost:8545
export WS_URL=ws://localhost:8545
export GATEWAY_URL=http://localhost:8546
export GATEWAY_WS_URL=ws://localhost:8546
```

For MinIO, prefer the container IP from the Docker network when the worker binary
runs on the host:

```bash
MINIO_IP="$(docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' fhevm-minio)"
export AWS_ENDPOINT_URL="http://${MINIO_IP}:9000"
```

When starting host binaries, avoid metrics and health port conflicts with the
Docker stack. The following ports were used successfully:

- TFHE: health `18080`, metrics `19100`.
- ZK: health `18081`, metrics `19101`.
- SNS: health `18082`, metrics `19102`.

The exact worker arguments can change with the binary version. Inspect current
flags with:

```bash
coprocessor/fhevm-engine/target/release/tfhe_worker --help
coprocessor/fhevm-engine/target/release/zkproof_worker --help
coprocessor/fhevm-engine/target/release/sns_worker --help
```

Useful log strings that confirm GPU execution:

```text
GPU feature is enabled
num_gpus: 1
streaming_multiprocessors: 114
```

If returning to the published stack workers:

```bash
docker start \
  coprocessor-tfhe-worker \
  coprocessor-zkproof-worker \
  coprocessor-sns-worker
```

## Trigger A New KMS Keygen

The host contracts image contains the Hardhat task used by bootstrap:

```text
task:triggerKeygen
```

The compose service is `host-sc-trigger-keygen` in
`test-suite/fhevm/docker-compose/host-sc-docker-compose.yml`.

To trigger another default-parameter keygen against the running host chain:

```bash
cd /home/ubuntu/fhevm
docker rm host-sc-trigger-keygen >/dev/null 2>&1 || true

set -a
. .fhevm/runtime/env/versions.env
set +a

FHEVM_STATE_DIR=/home/ubuntu/fhevm/.fhevm \
HOST_VERSION="${HOST_VERSION:-519c60c}" \
KEYGEN_PARAMS_TYPE=0 \
docker compose -p fhevm \
  -f test-suite/fhevm/docker-compose/host-sc-docker-compose.yml \
  run --rm --no-deps host-sc-trigger-keygen
```

This emits `PrepKeygenRequest` from `KMSGeneration`. The connector then processes:

1. `PrepKeygenRequest`
2. `PrepKeygenResponse`
3. `KeygenRequest`
4. `KeygenResponse`
5. host listener key ingestion into the coprocessor DB

Monitor the path:

```bash
docker logs -f kms-connector-gw-listener
docker logs -f kms-connector-kms-worker
docker logs -f kms-connector-tx-sender
docker logs -f coprocessor-host-listener
docker logs -f kms-core
```

Check on-chain keygen events:

```bash
export PATH="$HOME/.foundry/bin:$PATH"
cast logs \
  --rpc-url http://localhost:8545 \
  --from-block 0 \
  --address "$(grep KMS_GENERATION_CONTRACT_ADDRESS .fhevm/runtime/addresses/host/.env.host | cut -d= -f2)" \
  'PrepKeygenRequest(uint256,uint8,bytes)'

cast logs \
  --rpc-url http://localhost:8545 \
  --from-block 0 \
  --address "$(grep KMS_GENERATION_CONTRACT_ADDRESS .fhevm/runtime/addresses/host/.env.host | cut -d= -f2)" \
  'KeygenRequest(uint256,uint256,bytes)'
```

Check connector state:

```bash
docker exec coprocessor-and-kms-db psql -U postgres -d kms-connector -c "
select 'prep' as t, count(*) from prep_keygen_requests
union all select 'keygen', count(*) from keygen_requests
union all select 'prep_resp', count(*) from prep_keygen_responses
union all select 'keygen_resp', count(*) from keygen_responses;
"
```

Check ingested coprocessor keys and compressed XOF availability:

```bash
docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -c "
select
  sequence_number,
  encode(key_id,'hex') as key_id,
  octet_length(compressed_xof_keyset) as xof_len,
  octet_length(sks_key) as sks_len,
  created_at
from keys
order by sequence_number;
"
```

Initial result observed on this host with KMS core `b9087af` and FHEVM images
`519c60c`:

```text
sequence 1: key_id ...0001, xof_len NULL, sks_len 370927991
sequence 2: key_id ...0002, xof_len NULL, sks_len 370927991
```

Then KMS/core and runtime services were upgraded in place:

```text
CORE_VERSION=v0.14.0-1
CONNECTOR_DB_MIGRATION_VERSION=v0.14.0-0
CONNECTOR_GW_LISTENER_VERSION=v0.14.0-0
CONNECTOR_KMS_WORKER_VERSION=v0.14.0-0
CONNECTOR_TX_SENDER_VERSION=v0.14.0-0
COPROCESSOR_DB_MIGRATION_VERSION=v0.14.0-0
COPROCESSOR_HOST_LISTENER_VERSION=v0.14.0-0
COPROCESSOR_GW_LISTENER_VERSION=v0.14.0-0
COPROCESSOR_TX_SENDER_VERSION=v0.14.0-0
COPROCESSOR_TFHE_WORKER_VERSION=v0.14.0-0
COPROCESSOR_ZKPROOF_WORKER_VERSION=v0.14.0-0
COPROCESSOR_SNS_WORKER_VERSION=v0.14.0-0
```

Notes from that upgrade:

- `ghcr.io/zama-ai/kms/core-service:v0.14.0-1` exists.
- FHEVM coprocessor and KMS connector `v0.14.0-1` tags were not present in GHCR
  for the checked package names; `v0.14.0-0` tags were present and used.
- KMS core `v0.14.0-1` changed `kms-gen-keys`: it now expects
  `kms-gen-keys --config-file <CONFIG_FILE>`. The full server config is not
  accepted because it contains server-only sections such as `[telemetry]`.
  `test-suite/fhevm/docker-compose/core-docker-compose.yml` was patched to
  generate a small keygen-only TOML in the entrypoint when this newer CLI is
  detected, while preserving the old argument form for older cores.

After the upgrade, a third keygen was triggered and ingested:

```text
sequence 1: key_id ...0001, xof_len NULL, sks_len 370927991
sequence 2: key_id ...0002, xof_len NULL, sks_len 370927991
sequence 3: key_id ...0003, xof_len NULL, sks_len 370927991
```

KMS core `v0.14.0-1` logged checks for
`PUB/CompressedXofKeySet/...0003`, but the actual store lines were still for
`PUB/ServerKey/...0003` and `PUB/PublicKey/...0003`. So keygen can be triggered
and ingested on the upgraded stack, but centralized KMS still stores legacy
`ServerKey` material only in this configuration. The GPU SNS worker currently
rejects this state with:

```text
GPU coprocessor cannot read a legacy ServerKey-format key
(compressed_xof_keyset is NULL)
```

Until `compressed_xof_keyset` is populated, GPU SNS e2e cannot run successfully
against this stack even though all three GPU worker unit test suites pass.

Further ingestion checks showed this is not the host listener fetching the wrong
object while a compressed XOF object exists:

```bash
docker exec fhevm-minio sh -c '
  ls /data/kms-public/PUB
  ls /data/kms-public/PUB/CompressedXofKeySet 2>&1
  ls /data/kms-public/PUB/ServerKey
'
```

Observed MinIO state:

```text
PUB contains CRS, PublicKey, ServerKey, VerfAddress, VerfKey
/data/kms-public/PUB/CompressedXofKeySet does not exist
PUB/ServerKey contains ...0001, ...0002, ...0003
```

The host listener source probes `CompressedXofKeySet` first and only falls back
to `ServerKey` if the XOF object cannot be downloaded:

```text
coprocessor/fhevm-engine/host-listener/src/kms_generation/mod.rs
```

The live host listener logs for key `...0003` also showed:

```text
Try downloading /CompressedXofKeySet/...0003
Key ...0003 not found
CompressedXofKeySet probe failed ... trying legacy ServerKey prefix
```

The activation staging table matched the final `keys` table:

```sql
select
  encode(key_id,'hex') as key_id,
  status,
  octet_length(key_content_compressed_xof_keyset) as staged_xof_len,
  octet_length(key_content_sks_key) as staged_sks_len
from kms_key_activation_events
order by created_at;
```

```text
...0001 activated staged_xof_len NULL staged_sks_len 370927991
...0002 activated staged_xof_len NULL staged_sks_len 370927991
...0003 activated staged_xof_len NULL staged_sks_len 370927991
```

The likely request-side cause is in the KMS connector keygen request builder:

```text
kms-connector/crates/kms-worker/src/core/event_processor/kms.rs
```

Both prep-keygen and keygen requests pass `UNCOMPRESSED_KEY_SET_CONFIG`, whose
standard config sets:

```text
compute_key_type: Cpu
compressed_key_config: CompressedNone
```

That explains why KMS core `v0.14.0-1` generated/stored `ServerKey` rather than
`CompressedXofKeySet` in this stack configuration.

## Local Connector Patch For Compressed XOF

The local KMS connector was patched and rebuilt to request compressed XOF key
material from KMS core:

- `kms-connector/crates/kms-worker/src/core/event_processor/kms.rs`
  requests `CompressedKeyConfig::CompressedAll` for prep-keygen and keygen.
- `kms-connector/crates/utils/src/types/db.rs` accepts
  `CompressedXofKeySet` from KMS core as a connector DB key type.
- `kms-connector/crates/kms-worker/src/core/event_processor/protocol_config.rs`
  can map the connector DB key type back to `CompressedXofKeySet` for KMS core
  protocol config requests.

The host does not have `cargo` installed by default. The connector images were
built with the GHCR Rust builder and the already-published runtime image as the
base, avoiding the `cgr.dev` runtime pull:

```bash
DOCKER_BUILDKIT=1 docker build \
  -t ghcr.io/zama-ai/fhevm/kms-connector/kms-worker:gpu-xof-local \
  -f - /home/ubuntu/fhevm <<'EOF'
# syntax=docker/dockerfile:1
ARG RUST_IMAGE_VERSION=1.91.0
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder
ARG CARGO_PROFILE=release
USER root
WORKDIR /app
COPY .git ./.git
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY host-contracts/rust_bindings ./host-contracts/rust_bindings
COPY shared ./shared
COPY kms-connector ./kms-connector
WORKDIR /app/kms-connector
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/kms-connector/target,sharing=locked \
    git config --global --add safe.directory /app && \
    cargo fetch && \
    cargo build --profile=${CARGO_PROFILE} -p kms-worker && \
    mkdir -p /out && \
    cp target/${CARGO_PROFILE}/kms-worker /out/
FROM ghcr.io/zama-ai/fhevm/kms-connector/kms-worker:v0.14.0-0
COPY --from=builder --chown=fhevm:fhevm /out/kms-worker /app/kms-connector/bin/kms-worker
EOF
```

Restart only the worker service with the local image:

```bash
set -a
. /home/ubuntu/fhevm/.fhevm/runtime/env/versions.env
set +a

FHEVM_STATE_DIR=/home/ubuntu/fhevm/.fhevm \
CONNECTOR_KMS_WORKER_VERSION=gpu-xof-local \
docker compose -p fhevm \
  -f /home/ubuntu/fhevm/test-suite/fhevm/docker-compose/kms-connector-docker-compose.yml \
  up -d --no-deps --force-recreate kms-connector-kms-worker
```

After triggering keygen again, KMS core stored the compressed XOF object:

```text
PUB/CompressedXofKeySet/0400000000000000000000000000000000000000000000000000000000000004
PUB/PublicKey/0400000000000000000000000000000000000000000000000000000000000004
```

The connector then needed the DB enum extension locally:

```sql
ALTER TYPE key_type ADD VALUE IF NOT EXISTS 'CompressedKeySet';
```

With that in place, `kms-worker` stored the keygen response with key digests:

```text
Public
CompressedKeySet
```

`tx-sender` also had to be rebuilt with the DB enum change to read the response.
However, the current gateway/host `IKMSGeneration.KeyType` enum only supports
`Server = 0` and `Public = 1`. Sending `CompressedKeySet = 3` reverts. Mapping
`CompressedKeySet` to `Server` inside `tx-sender` changes the signed EIP-712
payload and recovers the wrong signer, so that is not a valid connector-only
fix. The correct long-term fix needs KMS core and the contracts/connector to
agree on the signed key type for "compressed XOF server key" material.

## Local Backfill Used To Unblock GPU Key Loading

Because key `...0004` could not be activated on-chain with the current contract
enum, it was backfilled locally through the host-listener ingestion tables so
the GPU workers could load a compressed XOF key:

```sql
INSERT INTO host_chain_blocks_valid (
  chain_id, block_hash, block_number, block_status, fhe_event_count, allow_event_count
)
VALUES (
  12345,
  decode('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa','hex'),
  999999,
  'finalized',
  1,
  0
)
ON CONFLICT (chain_id, block_hash)
DO UPDATE SET block_status = 'finalized';

INSERT INTO kms_key_activation_events (
  chain_id, block_hash, block_number, transaction_hash, key_id,
  key_digest_server, key_digest_public, storage_urls, status
)
VALUES (
  12345,
  decode('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa','hex'),
  999999,
  decode('bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb','hex'),
  decode('0400000000000000000000000000000000000000000000000000000000000004','hex'),
  decode('bc36d88929ce5d7bdf7d4a5c1f9f71763cdd12f8ec2b00121a7da828b639706e','hex'),
  decode('4fed169f1260c2ffc6ca29b464b34cc4ff3be969ac71653285842cb33274dbbb','hex'),
  array['http://minio:9000/kms-public'],
  'pending'
)
ON CONFLICT (chain_id, block_hash, key_id)
DO UPDATE SET status = 'pending', retry_count = 0, last_error = NULL;
```

The host-listener downloaded `/CompressedXofKeySet/...0004`, decompressed it,
and activated the key:

```text
sequence 4: key_id ...0004, xof_len 444303777, sks_len 370927991
```

This is a local test unblocker only. It does not update on-chain
`KMSGeneration`, so relayer `/v2/keyurl` still serves the last on-chain key.

## Host GPU Worker Launch Notes

For host-run SNS, use the MinIO container IP rather than `localhost`; otherwise
the AWS SDK bucket checks can fail even though MinIO health checks pass:

```bash
MINIO_IP="$(docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' fhevm-minio)"
export AWS_ENDPOINT_URL="http://${MINIO_IP}:9000"
```

Observed GPU startup confirmations:

```text
tfhe_worker: GPU feature is enabled, num_gpus=1, streaming_multiprocessors=114
zkproof_worker: GPU feature is enabled, num_gpus=1, streaming_multiprocessors=114
sns_worker: GPU feature is enabled, num_gpus=1, streaming_multiprocessors=114
sns_worker: Decompressed compressed_xof_keyset to CudaServerKey
```

Running `./fhevm-cli test light` after the local backfill currently fails at
input-proof verification because relayer serves key `...0003` from on-chain
`KMSGeneration`, while GPU workers load latest DB key sequence `4`:

```text
zkproof_worker: Failed to verify proof: Invalid Proof(3, ZK proof verification failed)
relayer /v2/keyurl: PublicKey/...0003
workers: Latest key cached ...0004, seq=4
```

This means full e2e remains blocked until key `...0004` can be activated
on-chain, or the whole test path is locally pointed at the same key material.

## Relayer Version Trial

Trying the relayer from the same release line as the workers did not unblock
the key mismatch:

```bash
FHEVM_STATE_DIR=/home/ubuntu/fhevm/.fhevm \
RELAYER_VERSION=v0.14.0-0 \
RELAYER_MIGRATE_VERSION=v0.14.0-0 \
docker compose -p fhevm \
  -f test-suite/fhevm/docker-compose/relayer-docker-compose.yml \
  up -d --no-deps --force-recreate relayer-db-migration relayer
```

The image starts successfully and initializes `/v2/keyurl` from the host chain,
but still serves the on-chain key:

```text
relayer image: ghcr.io/zama-ai/fhevm/relayer:v0.14.0-0
/v2/keyurl: PublicKey/...0003
```

## Successful Clean Redeploy With Compressed XOF

The working local unblock was a clean redeploy using a pinned lock with current
`v0.14.0-0` coprocessor/relayer/connector images and KMS core `v0.14.0-1`,
plus local patches for the compressed XOF contract and connector gaps.

Current local machine default:

- use `/home/ubuntu/fhevm/.fhevm/state/locks/gpu-v014-all-services.json`;
- do not use `latest-supported` for GPU benchmarks, because it previously
  resolved to stale non-GPU `v0.11.0` images on this machine;
- the lock intentionally pins local patched images:
  `HOST_VERSION=fhevm-local`,
  `CONNECTOR_KMS_WORKER_VERSION=fhevm-local`, and
  `CONNECTOR_TX_SENDER_VERSION=fhevm-local`;
- stale local `v0.11.0` images were removed after the corrected deploy.

The local host-contracts interface was patched so
`IKMSGeneration.KeyType.CompressedKeySet` keeps numeric value `3`:

```solidity
enum KeyType {
    Server, // 0
    Public, // 1
    Reserved, // 2
    CompressedKeySet // 3
}
```

The stack was redeployed with local `kms-worker`, `tx-sender`, and
`host-contracts` overrides:

```bash
PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
test-suite/fhevm/fhevm-cli up \
  --lock-file /home/ubuntu/fhevm/.fhevm/state/locks/gpu-v014-all-services.json \
  --scenario multi-chain \
  --override kms-connector:kms-worker,tx-sender \
  --override host-contracts \
  --allow-schema-mismatch
```

The connector local build still hit unauthenticated `cgr.dev` runtime pulls, so
the previously built patched connector images were retagged as `fhevm-local`
and recorded in `.fhevm/state/state.json`, then the CLI was resumed:

```bash
docker tag ghcr.io/zama-ai/fhevm/kms-connector/kms-worker:gpu-xof-local \
  ghcr.io/zama-ai/fhevm/kms-connector/kms-worker:fhevm-local
docker tag ghcr.io/zama-ai/fhevm/kms-connector/tx-sender:gpu-xof-local \
  ghcr.io/zama-ai/fhevm/kms-connector/tx-sender:fhevm-local

PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
test-suite/fhevm/fhevm-cli up --resume
```

Immediately after `kms-connector-db-migration`, the connector DB enum was
extended:

```bash
docker exec -e PGPASSWORD=postgres coprocessor-and-kms-db \
  psql -U postgres -d kms-connector -v ON_ERROR_STOP=1 \
  -c "ALTER TYPE key_type ADD VALUE IF NOT EXISTS 'CompressedKeySet';"
```

The clean deploy completed and activated key `...0001` on-chain. Relayer and
the finalized host-chain state agreed:

```text
/v2/keyurl: PublicKey/0400000000000000000000000000000000000000000000000000000000000001
getActiveKeyId(): 0x0400000000000000000000000000000000000000000000000000000000000001
```

One host-listener ingestion gap remained in the released coprocessor image:
`ActivateKey` rows only populated `key_digest_public`; the compressed keyset
digest with key type `3` was not copied into `key_digest_server`. To unblock the
local test run, the digest from the on-chain `ActivateKey` event was copied into
the activation row and the partial public-key download was cleared so the poller
would retry the full activation:

```sql
UPDATE kms_key_activation_events
SET
  key_digest_server = decode(
    '4e442705216b146fd935f2ccd481803e724637d374d9d5c1b901b9282f98ee7a',
    'hex'
  ),
  key_content_public = NULL,
  key_content_sns_pk = NULL,
  key_content_sks_key = NULL,
  key_content_compressed_xof_keyset = NULL,
  status = 'pending',
  last_error = NULL,
  last_updated_at = NOW()
WHERE key_id = decode(
  '0400000000000000000000000000000000000000000000000000000000000001',
  'hex'
);
```

The host-listener then downloaded:

```text
PUB/CompressedXofKeySet/0400000000000000000000000000000000000000000000000000000000000001
PUB/PublicKey/0400000000000000000000000000000000000000000000000000000000000001
```

and populated the coprocessor `keys` table:

```text
compressed_xof_keyset: 444303777 bytes
sks_key:               370927991 bytes
```

On 2026-07-09 the restored machine was recovered with the same flow, but by
persisting the already-built local image tags directly in the lock to avoid
accidental rebuilds:

```json
{
  "HOST_VERSION": "fhevm-local",
  "CONNECTOR_KMS_WORKER_VERSION": "fhevm-local",
  "CONNECTOR_TX_SENDER_VERSION": "fhevm-local"
}
```

After `kms-connector-db-migration`, run the enum extension before keygen
responses are processed:

```bash
docker exec -e PGPASSWORD=postgres coprocessor-and-kms-db \
  psql -U postgres -d kms-connector -v ON_ERROR_STOP=1 \
  -c "ALTER TYPE key_type ADD VALUE IF NOT EXISTS 'CompressedKeySet';"
```

If the released host-listener leaves `key_digest_server` empty, copy the
current `CompressedKeySet` digest from the `kms-connector-tx-sender` log into
the pending activation row and clear partial content so the listener retries.
Example from the 2026-07-09 deployment:

```sql
UPDATE kms_key_activation_events
SET
  key_digest_server = decode(
    '0b636c088be658bd0116fac4e5fdfd390115006ac45bae699aa698d0184b7b71',
    'hex'
  ),
  key_content_public = NULL,
  key_content_sns_pk = NULL,
  key_content_sks_key = NULL,
  key_content_compressed_xof_keyset = NULL,
  status = 'pending',
  last_error = NULL,
  last_updated_at = NOW()
WHERE key_id = decode(
  '0400000000000000000000000000000000000000000000000000000000000001',
  'hex'
);
```

The expected validation query is:

```sql
select encode(key_id,'hex') as key_id,
       octet_length(pks_key) as pks,
       octet_length(sks_key) as sks,
       octet_length(compressed_xof_keyset) as xof
from keys;
```

For the 2026-07-09 recovery, it returned:

```text
key_id ...0001, pks 33050, sks 370927991, xof 444303777
```

During the clean reset later on 2026-07-09, the same released host-listener
ingestion gap appeared again. The connector tx-sender emitted this current
`CompressedKeySet` digest for key `...0001`:

```text
b9d0c8284affe46901220531654982435dcf185bc7ac5ad269e22280efc02050
```

The same repair was applied with that digest. The listener then moved the
activation row from `pending` to `ready` and finally `activated`; validation
again returned:

```text
key_id ...0001, pks 33050, sks 370927991, xof 444303777
```

Host GPU workers should be started through the local systemd launcher so they
survive the shell/tool session:

```bash
/home/ubuntu/fhevm/test-suite/fhevm/scripts/gpu-workers.sh start
/home/ubuntu/fhevm/test-suite/fhevm/scripts/gpu-workers.sh status
/home/ubuntu/fhevm/test-suite/fhevm/scripts/gpu-workers.sh stop
```

The launcher maps TFHE to GPU0 and ZK/SNS to GPU1, uses the local v0.14 GPU
binaries in `coprocessor/fhevm-engine/target/release`, and sets batch sizes to
1000 for TFHE dependence chains and SNS work items.

With Docker CPU worker containers stopped, the host GPU workers were started
from `coprocessor/fhevm-engine/target/release` with the same compose channels
and non-conflicting local ports. SNS needs the signer flags from compose:

```bash
--signer-type=private-key --private-key="$TX_SENDER_PRIVATE_KEY"
```

All three workers confirmed GPU execution:

```text
tfhe_worker: GPU feature is enabled, num_gpus=1, streaming_multiprocessors=114
zkproof_worker: GPU feature is enabled, num_gpus=1, streaming_multiprocessors=114
sns_worker: GPU feature is enabled, num_gpus=1, streaming_multiprocessors=114
sns_worker: Decompressed compressed_xof_keyset to CudaServerKey
```

During the clean reset later on 2026-07-09, the runtime-only launcher was
removed along with `.fhevm`. The launcher now lives at
`test-suite/fhevm/scripts/gpu-workers.sh`, so use that tracked script after a
reset rather than restoring stale runtime files. After stopping Docker CPU
workers and starting the host services, health checks passed on ports `18080`,
`18081`, and `18082`, and SNS again decompressed the compressed XOF keyset to
`CudaServerKey`.

The light e2e suite passed with the host GPU workers active:

```bash
PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
test-suite/fhevm/fhevm-cli test light
```

Result:

```text
[pass] input-proof
[pass] erc20
[pass] light
```

The clean reset later on 2026-07-09 was validated the same way after switching
to host GPU workers:

```text
[pass] input-proof (16s)
[pass] erc20 (40s)
[pass] light (56s)
```

## ERC20 GPU Benchmark

The opt-in benchmark profile is:

```bash
PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
test-suite/fhevm/fhevm-cli test --network=staging erc20-benchmark
```

It defaults to 1000 independent encrypted ERC20 transfers. Each transfer uses a
unique sender and recipient wallet, and all input proofs are generated before
transfer submission. The profile emits a single JSON line prefixed with
`ERC20_BENCHMARK_REPORT`.

Useful overrides are passed through by `fhevm-cli` when they use the
`ERC20_BENCH_` prefix:

```bash
PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
ERC20_BENCH_TRANSFERS=1000 \
ERC20_BENCH_TRANSFER_CONCURRENCY=100 \
ERC20_BENCH_PROOF_CONCURRENCY=16 \
ERC20_BENCH_MINT_BATCH_SIZE=25 \
test-suite/fhevm/fhevm-cli test --network=staging erc20-benchmark
```

Proof generation goes through the relayer and can transiently return HTTP 500
under load. The benchmark retries proof generation with:

```bash
ERC20_BENCH_PROOF_RETRIES=5
ERC20_BENCH_PROOF_RETRY_DELAY_MS=1000
```

To save and reuse pre-generated input proofs, set `ERC20_BENCH_PROOF_CACHE` to a
path inside the e2e container:

```bash
PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
ERC20_BENCH_PROOF_CACHE=/tmp/erc20-benchmark-proofs.json \
test-suite/fhevm/fhevm-cli test --network=staging erc20-benchmark
```

The cache stores sender private keys, recipient addresses, amount handles, input
proofs, chain id, transfer count, transfer amount, and the benchmark contract
address. It is reusable only while that contract still exists on the same chain
and the benchmark metadata matches. Treat the cache file as sensitive key
material. For cached runs, proof generation is skipped and
`proofVerifiedToSnsReady` is reported as unavailable; `minedToSnsReady` and
throughput remain valid for the current run.

On this H100 host, the full 1000-transfer uncached run completed with lower
proof concurrency and an extended timeout:

```bash
PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
ERC20_BENCH_PROOF_CACHE=/tmp/erc20-benchmark-proofs-1000.json \
ERC20_BENCH_TIMEOUT_MS=7200000 \
ERC20_BENCH_PROOF_CONCURRENCY=4 \
ERC20_BENCH_PROOF_RETRIES=8 \
ERC20_BENCH_PROOF_RETRY_DELAY_MS=2000 \
test-suite/fhevm/fhevm-cli test --network=staging --no-hardhat-compile erc20-benchmark
```

The cache was copied out of the container to:

```text
.fhevm/runtime/erc20-benchmark-proofs-1000.json
```

Result:

```text
transferCount: 1000
walletCount: 2000
snsReadyTransfersPerSecond: 3.9432798624583985
wallSeconds: 253.596
minedToSnsReady p50/p95/p99: 120608ms / 197124ms / 199287ms
submittedToSnsReady p50/p95/p99: 142844ms / 201112ms / 201121ms
proofVerifiedToSnsReady p50/p95/p99: 1344829ms / 2385831ms / 2475836ms
proofVerificationTimestampsObserved: 1000
```

The transfer submission phase for that run produced exactly 1000 indexed host
chain transactions over blocks `15946..16107`:

```text
blocks: 162
min/p50/p90/max transactions per block: 1 / 5 / 8 / 80
average transactions per block: 6.17
```

The host node uses Anvil interval mining with `--block-time 1`. The peak block
contained 80 transfer transactions and used `29.55M / 30M` gas, so 100 encrypted
ERC20 transfers per block requires a higher Anvil gas limit, plus a longer
interval mining window or demand mining so enough pending transactions can be
batched before the block is sealed.

For live experiments, Anvil can be adjusted without restarting the host chain:

```bash
cast rpc evm_setIntervalMining 3 --rpc-url http://localhost:8545
cast rpc anvil_setBlockGasLimit 0x3938700 --rpc-url http://localhost:8545
```

`evm_setIntervalMining` takes seconds. Passing `3000` sets a 3000-second
interval, not 3000ms. Restore the default e2e settings with:

```bash
cast rpc evm_setIntervalMining 1 --rpc-url http://localhost:8545
cast rpc anvil_setBlockGasLimit 0x1c9c380 --rpc-url http://localhost:8545
```

A cached-proof run with 3s interval mining, 60M gas, and
`ERC20_BENCH_TRANSFER_CONCURRENCY=200` completed successfully:

```text
proofCache.reused: true
snsReadyTransfersPerSecond: 2.7543049786816796
wallSeconds: 363.068
minedToSnsReady p50/p95/p99: 224190ms / 355607ms / 359403ms
submittedToSnsReady p50/p95/p99: 287001ms / 363060ms / 363065ms
transfer blocks: 88
min/p50/p90/max transactions per transfer block: 1 / 8.5 / 11.3 / 173
average transactions per transfer block: 11.36
```

To maximize worker availability, use the benchmark's manual transfer mining
mode. This leaves mint/setup transactions on normal interval mining, then
temporarily stretches interval mining only during the transfer burst, disables
automine, submits all transfers, explicitly mines pending transfers, and
restores interval mining afterward:

```bash
cast rpc anvil_setBlockGasLimit 0x3b9aca00 --rpc-url http://localhost:8545

PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
ERC20_BENCH_PROOF_CACHE=/tmp/erc20-benchmark-proofs-1000.json \
ERC20_BENCH_TIMEOUT_MS=7200000 \
ERC20_BENCH_TRANSFER_CONCURRENCY=1000 \
ERC20_BENCH_TRANSFER_SUBMISSION_MODE=manual-mine \
ERC20_BENCH_MANUAL_MINE_MAX_BLOCKS=20 \
ERC20_BENCH_MANUAL_MINE_INTERVAL_SECONDS=3600 \
ERC20_BENCH_MANUAL_MINE_RESTORE_INTERVAL_SECONDS=1 \
ERC20_BENCH_MINT_BATCH_SIZE=100 \
test-suite/fhevm/fhevm-cli test --network=staging --no-hardhat-compile erc20-benchmark

cast rpc evm_setIntervalMining 1 --rpc-url http://localhost:8545
cast rpc anvil_setBlockGasLimit 0x1c9c380 --rpc-url http://localhost:8545
cast rpc evm_mine --rpc-url http://localhost:8545
```

On this H100 host, that packed all 1000 transfer transactions into a single
host-chain block:

```text
proofCache.reused: true
transferSubmissionMode: manual-mine
manualMineBlocks: 1
manualMineIntervalSeconds: 3600
transfer block: 21805
transactions in transfer block: 1000
snsReadyTransfersPerSecond: 4.685025720791208
wallSeconds: 213.446
minedToSnsReady p50/p95/p99: 152192ms / 196052ms / 200214ms
submittedToSnsReady p50/p95/p99: 164861ms / 208237ms / 212285ms
```

The corresponding durable DB timings for transfer block `21805` were:

```text
transfer transactions: 1000 in block 21805
PBS rows for the transfer block: 2000
PBS created_at: all rows at 2026-07-07 18:56:39.790687 UTC
PBS completed_at span: 2026-07-07 18:58:25.853365..19:00:00.637886 UTC
```

An earlier manual-mine run without stretching interval mining saturated the GPU
but still let Anvil interval-mine during submission, so the 1000 transfers were
split over 7 blocks. Set `ERC20_BENCH_MANUAL_MINE_INTERVAL_SECONDS` to a large
value when single-block packing matters.

For TFHE-worker-only throughput, use the `computations` table as the completion
signal, not `pbs_computations`. The TFHE worker marks `computations.is_completed`
and inserts output ciphertext rows; the SNS worker later consumes those outputs
and marks `pbs_computations.is_completed`. Pause SNS during this run so the
measurement excludes SNS upload/noise-squashing:

```bash
SNS_PID=$(pgrep -f '^coprocessor/fhevm-engine/target/release/sns_worker|^/.*sns_worker' | head -1)
kill -STOP "$SNS_PID"

cast rpc anvil_setBlockGasLimit 0x3b9aca00 --rpc-url http://localhost:8545

PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
ERC20_BENCH_COMPLETION_TARGET=computations-completed \
ERC20_BENCH_PROOF_CACHE=/tmp/erc20-benchmark-proofs-1000.json \
ERC20_BENCH_TIMEOUT_MS=7200000 \
ERC20_BENCH_TRANSFER_CONCURRENCY=1000 \
ERC20_BENCH_TRANSFER_SUBMISSION_MODE=manual-mine \
ERC20_BENCH_MANUAL_MINE_MAX_BLOCKS=20 \
ERC20_BENCH_MANUAL_MINE_INTERVAL_SECONDS=3600 \
ERC20_BENCH_MANUAL_MINE_RESTORE_INTERVAL_SECONDS=1 \
ERC20_BENCH_MINT_BATCH_SIZE=100 \
test-suite/fhevm/fhevm-cli test --network=staging --no-hardhat-compile erc20-benchmark

kill -CONT "$SNS_PID"
cast rpc evm_setIntervalMining 1 --rpc-url http://localhost:8545
cast rpc anvil_setBlockGasLimit 0x1c9c380 --rpc-url http://localhost:8545
cast rpc evm_mine --rpc-url http://localhost:8545
```

On this H100 host, a clean TFHE-only run produced:

```text
completionTarget: computations-completed
transfer block: 23038
transactions in transfer block: 1000
computations rows in transfer block: 5000
computations completed: 5000
computations pending: 0
computations created_at: 2026-07-07 19:17:42.285060 UTC
computations max completed_at: 2026-07-07 19:17:47.425754 UTC
dbCreatedToCompletedSeconds: 5.141
computationsPerSecond: 972.5734292939117
transferEquivalentPerSecond: 194.51468585878234
wallSeconds: 72.651
computationsCompletedTransfersPerSecond: 13.76443545167995
minedToComputationsCompleted p50/p95/p99: 63093ms / 64127ms / 64239ms
```

For that same transfer block, `pbs_computations` had 2000 rows and 0 completed
while SNS was paused, confirming that the benchmark stopped on TFHE completion
rather than SNS completion.

### ERC20 Steady-State Low-Load Runs

For steady-state latency measurements below system capacity, use the
`steady-rate` submission mode. The benchmark disables automine, sets interval
mining to 1 second, submits `ERC20_BENCH_STEADY_TRANSFERS_PER_BLOCK`
transactions, waits for that block's receipts, then submits the next batch.
The proof cache can be larger than the requested transfer count; the benchmark
uses the first `ERC20_BENCH_TRANSFERS` cached entries.

Common configuration used for the 2026-07-08 runs:

```bash
PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
ERC20_BENCH_COMPLETION_TARGET=sns-ready \
ERC20_BENCH_PROOF_CACHE=/tmp/erc20-benchmark-proofs-1000.json \
ERC20_BENCH_TRANSFER_SUBMISSION_MODE=steady-rate \
ERC20_BENCH_TIMEOUT_MS=3600000 \
ERC20_BENCH_MINT_BATCH_SIZE=60 \
ERC20_BENCH_MANUAL_MINE_RESTORE_INTERVAL_SECONDS=0 \
test-suite/fhevm/fhevm-cli test --network=staging --no-hardhat-compile erc20-benchmark
```

Scenario-specific overrides and outcomes:

```text
X=1 tx/block, 60 blocks
ERC20_BENCH_TRANSFERS=60
ERC20_BENCH_TRANSFER_CONCURRENCY=1
ERC20_BENCH_STEADY_TRANSFERS_PER_BLOCK=1
blocks: 3619..3678, blockCount=60, min/p50/max tx per block=1/1/1
snsReadyTransfersPerSecond: 0.9738994935722634
wallSeconds: 61.608
minedToSnsReady p50/p95/p99: 29103ms / 56247ms / 59259ms
submittedToSnsReady p50/p95/p99: 30128ms / 57091ms / 60588ms

X=2 tx/block, 60 blocks
ERC20_BENCH_TRANSFERS=120
ERC20_BENCH_TRANSFER_CONCURRENCY=2
ERC20_BENCH_STEADY_TRANSFERS_PER_BLOCK=2
blocks: 3681..3740, blockCount=60, min/p50/max tx per block=2/2/2
snsReadyTransfersPerSecond: 1.961008612096155
wallSeconds: 61.193
minedToSnsReady p50/p95/p99: 28845ms / 55868ms / 58838ms
submittedToSnsReady p50/p95/p99: 29877ms / 56909ms / 60170ms

X=5 tx/block, 60 blocks
ERC20_BENCH_TRANSFERS=300
ERC20_BENCH_TRANSFER_CONCURRENCY=5
ERC20_BENCH_STEADY_TRANSFERS_PER_BLOCK=5
blocks: 3746..3805, blockCount=60, min/p50/max tx per block=5/5/5
snsReadyTransfersPerSecond: 4.879953152449737
wallSeconds: 61.476
minedToSnsReady p50/p95/p99: 29196ms / 56136ms / 59082ms
submittedToSnsReady p50/p95/p99: 29993ms / 57189ms / 60446ms
```

All three runs reused `/tmp/erc20-benchmark-proofs-1000.json`, so
`proofVerifiedToSnsReady` was intentionally unavailable in the benchmark report.
The original `minedToSnsReady` and `submittedToSnsReady` values above were
observer-time measurements: the harness only began polling SNS readiness after
all receipts from the 60-second steady-rate feed had been collected. That makes
early transfers inherit most of the feeder duration and explains the roughly
30-second p50. It is not the per-transfer TFHE/SNS latency.

After adding a retained result-handle CT128 completion timestamp from
`pbs_computations.completed_at`, a repeat X=1 run on the two-H100 machine showed:

```text
X=1 tx/block, 60 blocks
blocks: 3868..3927, blockCount=60, min/p50/max tx per block=1/1/1
snsReadyTransfersPerSecond: 0.978266186229273
wallSeconds: 61.333

observer-time minedToSnsReady p50/p95/p99:
29029ms / 56063ms / 59023ms

result-handle CT128 completion, using pbs_computations.completed_at:
minedToSnsCt128Computed p50/p95/p99:
171ms / 442ms / 2364ms
submittedToSnsCt128Computed p50/p95/p99:
1173ms / 1464ms / 3654ms

stageBreakdown.computations:
total=300 completed=300 dbCreatedToCompletedSeconds=58.887
completedItemsPerSecond=5.094503031229303

stageBreakdown.pbsComputations:
total=120 completed=120 dbCreatedToCompletedSeconds=59.021
completedItemsPerSecond=2.0331746327578317

stageBreakdown.tfheCiphertexts:
total=120 completed=120 dbCreatedToCompletedSeconds=58.887
completedItemsPerSecond=2.0378012124917215

stageBreakdown.snsCt128Computed:
total=60 completed=60 dbCreatedToCompletedSeconds=56.765
completedItemsPerSecond=1.0569893420241345
```

The small negative minimum in `minedToSnsCt128Computed` on this run was a
host/container/DB clock observation artifact of about 50 ms. For exact
post-upload SNS readiness, add an update timestamp to `ciphertext_digest` or
poll readiness concurrently while feeding steady-rate transactions.

The report includes:

- throughput to SNS ciphertext readiness,
- proof-accepted-to-SNS-ready latency, using `input_handles.created_at` as the
  durable proof acceptance timestamp,
- mined-to-SNS-ready latency, using the receipt observation time as the mined
  timestamp.
- result-handle CT128 completion latency, using
  `pbs_computations.completed_at` for the transfer result handle,
- stage breakdowns for `computations`, `pbs_computations`, TFHE ciphertext rows,
  and result-handle CT128 completion.

When validating changes against a prebuilt `test-suite/e2e` image, copy the new
benchmark spec and modified ERC20 contract into the running
`fhevm-test-suite-e2e-debug` container, then install `pg` in that container.
Future images built from this branch include the dependency directly.

### 2026-07-08 Two-H100 Cached SNS Run

On the replacement two-H100 host, run TFHE on GPU0 and ZK/SNS on GPU1. Keep the
generated proof cache in the test container at `/tmp` while running, then copy it
back to the host runtime directory:

```bash
docker cp \
  fhevm-test-suite-e2e-debug:/tmp/erc20-benchmark-proofs-1000.json \
  .fhevm/runtime/erc20-benchmark-proofs-1000.json
chmod 664 .fhevm/runtime/erc20-benchmark-proofs-1000.json
```

The cache is deployment-specific. The benchmark validates that
`contractAddress` still has code on the current host chain; after a redeploy,
the old cache is ignored and proofs must be regenerated. To avoid losing a long
proof-generation run, the benchmark writes the proof cache before minting setup
balances.

For prebuilt or restored test-suite containers, verify `pg` is installed before
running the benchmark DB polling path:

```bash
docker exec fhevm-test-suite-e2e-debug sh -lc \
  'cd /app/test-suite/e2e && node -e "require(\"pg\")" || npm install pg@^8.22.0 --no-save'
```

Use interval restore `0` for manual-mine benchmark runs and restore automine
after interval mining. Setting `evm_setIntervalMining 1` disables automine on
Anvil; verify with `anvil_getAutomine` before leaving the stack idle.

Successful cached run command:

```bash
cast rpc evm_setIntervalMining 0 --rpc-url http://localhost:8545
cast rpc evm_setAutomine true --rpc-url http://localhost:8545
cast rpc anvil_setBlockGasLimit 0x3b9aca00 --rpc-url http://localhost:8545

PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
ERC20_BENCH_PROOF_CACHE=/tmp/erc20-benchmark-proofs-1000.json \
ERC20_BENCH_TIMEOUT_MS=7200000 \
ERC20_BENCH_TRANSFER_CONCURRENCY=1000 \
ERC20_BENCH_TRANSFER_SUBMISSION_MODE=manual-mine \
ERC20_BENCH_MANUAL_MINE_MAX_BLOCKS=20 \
ERC20_BENCH_MANUAL_MINE_INTERVAL_SECONDS=3600 \
ERC20_BENCH_MANUAL_MINE_RESTORE_INTERVAL_SECONDS=0 \
ERC20_BENCH_MINT_BATCH_SIZE=25 \
test-suite/fhevm/fhevm-cli test --network=staging --no-hardhat-compile erc20-benchmark
```

Result from the successful run:

```text
proofCache.reused: true
transferSubmissionMode: manual-mine
manualMineBlocks: 1
transfer block: 3419
transactions in transfer block: 1000
snsReadyTransfersPerSecond: 6.701424722896087
wallSeconds: 149.222
minedToSnsReady p50/p95/p99: 88112ms / 134773ms / 138869ms
submittedToSnsReady p50/p95/p99: 97727ms / 144148ms / 148148ms
```

For the same transfer block, the TFHE worker completed all DB computations:

```text
computations rows in transfer block: 5000
computations completed: 5000
computations created_at min: 2026-07-08 15:04:41.219471 UTC
computations completed_at max: 2026-07-08 15:05:52.139647 UTC
DB created-to-completed span: 70.920176s
```

### Large-Batch TFHE Worker Stability

When increasing TFHE work acquisition, use the worker settings to maximize work
availability rather than searching for a small "safe" value:

```bash
CUDA_VISIBLE_DEVICES=0 RUST_BACKTRACE=1 ./target/release/tfhe_worker \
  --run-bg-worker \
  --database-url=postgresql://postgres:postgres@localhost:5432/coprocessor \
  --pg-pool-max-connections=10 \
  --worker-polling-interval-ms=1000 \
  --work-items-batch-size=100 \
  --key-cache-size=32 \
  --dependence-chains-per-batch=1000 \
  --coprocessor-fhe-threads=16 \
  --tokio-threads=16 \
  --health-check-port=18091 \
  --metrics-addr=0.0.0.0:19191
```

An observed instability with `--dependence-chains-per-batch=1000` was caused by
unbounded DCID logging in the TFHE worker and dependence-chain lock manager.
Each acquire/extend/release path serialized hundreds or thousands of 32-byte
chain IDs into JSON log lines, and the large run either stalled or disappeared
before useful scheduler diagnostics were emitted. The local fix bounds these
logs to a count plus a three-ID sample, adds coarse stage timings around graph
build, input fetch, scheduler execution, and result upload, and keeps selected
CUDA key and execution GPU index aligned in the scheduler.

A dirty-state reproduction after this fix drained two interrupted benchmark
blocks using `--dependence-chains-per-batch=1000`:

```text
blocks: 3542 and 3553
final computations: 5000/5000 completed for each block
final errors: 0 for each block
final dependence_chain state: 1000 processed for each block
instrumented worker span: 2026-07-08 16:18:12..16:19:45 UTC
GPU0 utilization during compute: repeatedly 79-100%
```

This was not a clean throughput benchmark because it resumed already-dirty
blocks from earlier failed runs, but it validated that high dependence-chain
batching can run to completion once logging is bounded and lock release avoids
marking chains processed while eligible computations remain.

## Confidential Auction Bid Benchmark

The opt-in auction profile is:

```bash
test-suite/fhevm/fhevm-cli test --network=staging auction-benchmark
```

It extracts the confidential bid kernel from the token auction contract into a
minimal benchmark contract. Non-coprocessor dependencies such as KYC, auction
lifecycle checks, factories, and full ERC7984 wrapping are skipped. The
benchmark preserves the encrypted bid path: external encrypted input,
remaining-quantity cap, encrypted payment amount, encrypted payment simulation,
payment confirmation, quota update, and encrypted requested-quantity aggregation
per price level.

The default workload is 500 bidders with 2 bids each. Bid prices are sampled
from a normal distribution between `0.01` and `0.2`, represented as integer
price values `10000..200000` with a `5000` tick. Inputs are generated before
submission so proof generation does not interfere with coprocessor throughput.
Set `AUCTION_BENCH_PROOF_CACHE` to save and reuse those inputs on the same
deployment:

```bash
PATH="$HOME/.bun/bin:$HOME/.foundry/bin:$PATH" \
AUCTION_BENCH_PROOF_CACHE=/tmp/auction-benchmark-proofs-500x2.json \
AUCTION_BENCH_TIMEOUT_MS=7200000 \
AUCTION_BENCH_SUBMISSION_MODE=manual-mine \
AUCTION_BENCH_MANUAL_MINE_INTERVAL_SECONDS=3600 \
AUCTION_BENCH_MANUAL_MINE_RESTORE_INTERVAL_SECONDS=0 \
test-suite/fhevm/fhevm-cli test --network=staging --no-hardhat-compile auction-benchmark
```

The report is printed as `AUCTION_BENCHMARK_REPORT`. Important environment
overrides use the `AUCTION_BENCH_` prefix, including
`AUCTION_BENCH_BIDDERS`, `AUCTION_BENCH_BIDS_PER_BIDDER`,
`AUCTION_BENCH_BID_CONCURRENCY`, `AUCTION_BENCH_PROOF_CONCURRENCY`,
`AUCTION_BENCH_COMPLETION_TARGET`, and `AUCTION_BENCH_DATABASE_URL`.

When `AUCTION_BENCH_BIDS_PER_BIDDER > 1`, the benchmark sends bids for a single
bidder sequentially while keeping different bidders parallel. This avoids nonce
races such as `replacement transaction underpriced` when many bids are
submitted concurrently from the same wallet.

Observed 2026-07-08 two-H100 fresh run:

```text
proofCache.reused: false
proofCache.path: /tmp/auction-benchmark-proofs-500x2-fresh.json
bid block: 3617
transactions in bid block: 1000
computations rows in bid block: 13968
computations completed: 13968
dbCreatedToCompletedSeconds: 253.870
completedItemsPerSecond: 55.020285973135856
bidEquivalentPerSecond: 3.9390239098751327
wallSeconds: 268.384
minedToCompleted p50/p95/p99: 256467ms / 257509ms / 257624ms
submittedToCompleted p50/p95/p99: 267442ms / 268270ms / 268271ms
```

That run required manually unblocking stale nonzero `dependency_count` values
for the remaining block-local dependence chains after the first TFHE pass. The
rows were all `updated`, `worker_id IS NULL`, allowed, and not errored, but were
not schedulable because `dependency_count` stayed greater than zero.

## 2026-07-09 Full `bench-e2e-basic` Attempt on v0.14.0-00

Invocation requested:

```bash
python3 /home/ubuntu/.codex/skills/bench-e2e-basic/scripts/bench_e2e_basic.py \
  run --repo /home/ubuntu/fhevm --label v0.14.0-00
```

The v0.14 GPU deployment was active and correctly mapped:

- TFHE worker: host systemd `fhevm-gpu-tfhe.service`, GPU0.
- ZK worker: host systemd `fhevm-gpu-zk.service`, GPU1.
- SNS worker: host systemd `fhevm-gpu-sns.service`, GPU1.
- Docker CPU worker containers were not running.

The full matrix did not complete. Partial report directories:

- `.fhevm/reports/bench-e2e-basic/20260709-125448Z-v0.14.0-00`
- `.fhevm/reports/bench-e2e-basic/20260709-125554Z-v0.14.0-00`
- `.fhevm/reports/bench-e2e-basic/20260709-125627Z-v0.14.0-00`

Initial failures were container/source sync issues in the e2e image:

- Missing `pg` package in the e2e container. The skill installs it during
  prepare when absent.
- Missing local SDK support files under `test/sdk/fhevm-sdk` and
  `test/sdk/unified`. The skill now copies the full local `test/sdk` tree.
- The e2e image's `@zama-fhe/relayer-sdk@0.5.0-alpha.2` could not deserialize
  the active v0.14 public key from `/v2/keyurl`:

```text
TFHEError: Impossible to fetch public key: wrong relayer url.
Details: "invalid value: integer `1`, expected variant index 0 <= i < 1"
```

The same deserialization error was reproduced with
`@zama-fhe/relayer-sdk@0.5.0-rc.1`, so simply bumping the npm package did not
unblock fresh proof generation.

The saved ERC20 proof cache at
`.fhevm/runtime/erc20-benchmark-proofs-1000.json` contains 1200 entries but is
not a portable full input cache. It stores handles and input proofs, not the
verified ciphertext DB rows/S3 objects produced by ZK verification. After a
deployment/key/context change:

- The cached contract address was absent on Anvil and had to be restored with
  runtime bytecode.
- The `Ownable` owner slot and FHE coprocessor config slots had to be restored.
- Cached proofs then failed with `InvalidSigner(address)`, indicating they were
  generated against a different KMS/input-verifier context.
- A preverified workaround was attempted by submitting cached handles with empty
  proofs and manually marking ACL permissions for `(handle, sender)` and
  `(handle, contract)`. This allowed transactions to be mined, but TFHE worker
  logged `Missing input to compute transaction - skipping` because the actual
  ciphertext material was still absent.

Conclusion: to make proof caches reusable across restarts, cache and restore
the full verified-input material, not only proof calldata. At minimum that
means the handle, proof, originating contract context, and the post-ZK
`ciphertexts`/object-store material that TFHE consumes. Without that, the full
fixed-rate e2e matrix requires working fresh proof generation against the
current v0.14 `/v2/keyurl` material.

## 2026-07-09 Full `bench-e2e-basic` Run on v0.14.0-01

Started after the clean v0.14 GPU reset was validated with
`fhevm-cli test light`.

Pre-run state:

- KMS connector uses local patched `fhevm-local` images for `kms-worker` and
  `tx-sender`.
- Coprocessor `keys` table has compressed XOF material for key `...0001`
  (`pks=33050`, `sks=370927991`, `xof=444303777`).
- Docker CPU coprocessor workers are stopped.
- Host GPU workers are active and healthy: TFHE on GPU0, ZK and SNS on GPU1.
- Runner plan expands to 35 scenarios. Fixed-rate scenarios are ordered
  high-to-low by the runner so the largest proof cache is generated first and
  reused by smaller runs; this still covers ERC20 TPS 1..20 and auction TPS
  1..7.

Command:

```bash
python3 /home/ubuntu/.codex/skills/bench-e2e-basic/scripts/bench_e2e_basic.py \
  run --repo /home/ubuntu/fhevm --label v0.14.0-01
```

First attempt created partial report
`.fhevm/reports/bench-e2e-basic/20260709-140822Z-v0.14.0-01` and was stopped
after the ERC20 benchmark proof workers hit the old relayer-SDK public-key
deserialization failure:

```text
TFHEError: Impossible to fetch public key: wrong relayer url.
Details: "invalid value: integer `1`, expected variant index 0 <= i < 1"
Version: @zama-fhe/relayer-sdk@0.5.0-rc.1
```

Pitfall: the main e2e instance path had already switched to `FhevmSdk`, but the
ERC20 benchmark child proof worker still imported `RelayerSdk` directly. Local
fix: change `test-suite/e2e/test/encryptedERC20/erc20ProofWorker.ts` to create
`FhevmSdk` and pass `protocolConfigAddress` through the worker SDK config.

The first one-transfer smoke check after that patch failed locally with
`ReferenceError: protocolConfigAddress is not defined` because the benchmark
file passed the field without importing it from `../instance`. Import
`protocolConfigAddress` alongside the other exported e2e addresses.

After importing `protocolConfigAddress`, a one-transfer ERC20 benchmark smoke
run passed with `FhevmSdk` proof generation:

```text
[erc20-benchmark] generated input proofs {"generated":1,"total":1,"worker":0}
ERC20_BENCHMARK_REPORT ... minedToComputationsCompleted p50=11ms,
minedToSnsCt128Computed p50=218ms
[pass] erc20-benchmark (20s)
```

Observation to investigate after the full run completes: during the
`v0.14.0-01` ERC20 proof/verification phase, the host GPU ZK worker
`fhevm-gpu-zk.service` used roughly 26-32 GB RSS. Prior expected usage was
closer to 8-12 GB. The Docker CPU `coprocessor-zkproof-worker` was stopped, so
this is the host GPU worker process, not a stale container worker.

Follow-up on the ZK worker memory issue: the local host worker launcher was
running ZK with `--worker-thread-count=16` in
`.fhevm/runtime/gpu-workers/start-gpu-workers.sh`, while the checked-in
coprocessor compose files use `--worker-thread-count=4` and the binary default
is `8`. ZK memory scales with worker count because each spawned verifier worker
sets up/caches its GPU verification context/key material. The high-memory state
was:

```text
fhevm-gpu-zk.service PID 1244384
RSS ~= 25.3 GiB
GPU1 ZK process memory ~= 26034 MiB
GPU1 total with SNS ~= 30644 MiB
```

The local persisted launcher was changed to `--worker-thread-count=4`, then
only `fhevm-gpu-zk.service` was restarted on GPU1. After settling:

```text
fhevm-gpu-zk.service PID 1717190
MemoryCurrent=8640286720
RSS ~= 8.1 GiB
GPU1 ZK process memory ~= 6924 MiB
GPU1 total with SNS ~= 11534 MiB
curl http://127.0.0.1:18081/healthz -> healthy
```

Use `4` ZK worker threads for the two-H100 host unless deliberately testing ZK
proof-verification parallelism. The previous 16-thread setting was the cause of
the 26 GB ZK GPU/RSS observation, not a stale CPU container or a new key-format
regression.

The second full attempt wrote
`.fhevm/reports/bench-e2e-basic/20260709-141236Z-v0.14.0-01`. It completed all
ERC20 fixed-rate scenarios (`1..20 TPS`) and all four ERC20 1000-transfer
single-block max-throughput targets. The run then stopped after three
consecutive auction failures because the auction benchmark did not yet support
the runner's `AUCTION_BENCH_SUBMISSION_MODE=steady-rate`:

```text
AUCTION_BENCH_SUBMISSION_MODE must be one of await-receipt, burst, manual-mine,
got steady-rate
```

Local auction benchmark fixes made after that failure:

- Add `steady-rate` submission mode, using one mined block per bid batch.
- Add auction proof-cache wiring in the skill runner.
- Size the reusable auction cache for the largest fixed-rate auction case
  (`/tmp/auction-benchmark-proofs-420.json`) and allow smaller auction runs to
  reuse a prefix of the cache.
- Remove the unsupported auction `sns-ready` max-throughput target from the
  default matrix. The auction benchmark currently measures `computations`,
  `tfhe-ciphertexts`, and `pbs-completed`; it has no SNS/ciphertext128
  completion signal.
- Copy auction benchmark support files into the test-suite container, including
  the generated TypeChain `types` directory.
- Deploy `ConfidentialAuctionBidBench` through
  `ConfidentialAuctionBidBench__factory` instead of `ethers.getContractFactory`.
  Hardhat can delete ad hoc copied artifacts during test startup, while the
  TypeChain factory embeds ABI and bytecode.
- Use `FhevmSdk.create(...)` for auction input proof generation. The relayer
  SDK path fails against the compressed XOF public key with the same
  `invalid value: integer 1` deserialization error seen in the old ERC20 proof
  worker.
- Fix a sparse-array bug in auction steady-rate batching: `mapPreparedBidsByBidder`
  indexes results by global bid index, so batch-level callers must filter out
  holes before awaiting receipts.

Auction smoke history:

- `20260709-152858Z-v0.14.0-01-auction-smoke`: failed, missing
  `ConfidentialAuctionBidBench` artifact in the container.
- `20260709-152939Z-v0.14.0-01-auction-smoke2`: failed, copied artifact was
  root-owned and Hardhat could not unlink it.
- `20260709-153008Z-v0.14.0-01-auction-smoke3` and
  `20260709-153042Z-v0.14.0-01-auction-smoke4`: failed because Hardhat removed
  the ad hoc copied artifact before lookup.
- `20260709-153149Z-v0.14.0-01-auction-smoke5`: factory deployment worked, then
  auction proof generation hit the stale relayer-SDK public-key error.
- `20260709-153314Z-v0.14.0-01-auction-smoke6`: generated and persisted a
  60-entry auction cache, then failed on the sparse steady-rate batch result.
- `20260709-153632Z-v0.14.0-01-auction-smoke7`: passed. It reused the cached
  inputs, submitted 60 independent auction bids as 60 one-bid blocks, and wrote
  `AUCTION_BENCHMARK_REPORT` with `wallSeconds=61.389`.

Auction-only matrix attempt
`.fhevm/reports/bench-e2e-basic/20260709-153841Z-v0.14.0-01-auction` was then
started with:

```bash
python3 /home/ubuntu/.codex/skills/bench-e2e-basic/scripts/bench_e2e_basic.py \
  run --repo /home/ubuntu/fhevm --only auction --label v0.14.0-01-auction
```

The first scenario, `auction-fixed-07tps`, generated and persisted the full
420-entry proof cache:

```text
/tmp/auction-benchmark-proofs-420.json
.fhevm/runtime/auction-benchmark-proofs-420.json
```

It submitted 420 bids as 60 blocks with exactly 7 bids per block:

```text
transactionsPerBlock min/p50/p99/max = 7/7/7/7
```

The scenario then stalled waiting for `computations-completed`:

```text
completed=5378 total=6268 pending=890
```

The pending rows were all `is_allowed=true`; all non-allowed computations for
the 7 TPS block range completed:

```sql
is_allowed=false total=2940 completed=2940 pending=0
is_allowed=true  total=3328 completed=2438 pending=890
```

The pending operation IDs were:

```text
0  FheAdd        pending=545
1  FheSub        pending=138
25 FheIfThenElse pending=207
```

Dependence-chain state showed many chains stuck in `processing` with
`dependency_count=0`, so they should have been runnable. Restarting
`fhevm-gpu-tfhe.service` did not advance the DB; GPU0 remained saturated and no
new `completed_at` values appeared. The run was interrupted instead of waiting
for the one-hour scenario timeout.

After interrupting the runner, the Hardhat process inside the Docker exec did
not terminate cleanly and required `sudo kill -9` on the host PIDs. TFHE was
then stopped intentionally to avoid burning GPU on the abandoned pending rows:

```bash
sudo systemctl stop fhevm-gpu-tfhe.service
```

Current state after this interruption:

- `fhevm-gpu-tfhe.service`: inactive
- `fhevm-gpu-zk.service`: active
- `fhevm-gpu-sns.service`: active
- DB has 890 pending computation rows from the stalled auction 7 TPS attempt.

Before another benchmark run, reset the deployment or clear the benchmark DB
state rather than simply restarting TFHE, otherwise the worker will resume
spinning on the stale auction rows. The likely next investigation is why the
stateful auction workload leaves `is_allowed=true` FHE Add/Sub/IfThenElse rows
stuck in runnable dependence chains. A useful reduction is to compare:

- auction 1 TPS, known to pass;
- auction 2..7 TPS one at a time;
- a version with fewer shared encrypted storage updates, for example removing
  or isolating the per-price and holding-wallet aggregate writes, to determine
  whether same-block writes to shared encrypted storage are the trigger.

### Lightweight DB Cleanup To Try Next

Prefer a lightweight cleanup before doing another full Docker volume reset. The
full reset works, but it discards KMS/core state and forces keygen plus the
local compressed-XOF repairs again. For benchmark reruns on the same deployment,
try preserving key material and only clearing transient benchmark/worker state.

The important rows to preserve are:

- `coprocessor.keys`, especially `pks_key`, `sks_key`, and
  `compressed_xof_keyset`;
- activated `coprocessor.kms_key_activation_events` rows for the current key;
- KMS connector keygen/key material state unless intentionally testing keygen.

Tables that should be safe candidates for truncation are benchmark-owned timing
tables, worker queues, computation rows, ciphertext/output material tied to old
benchmark transactions, transaction-sender queues, listener event cursors or
pending event rows, and relayer/test-suite transient DB state. Verify table
names against the live schema before running cleanup SQL:

```bash
docker exec -e PGPASSWORD=postgres coprocessor-and-kms-db \
  psql -U postgres -d coprocessor -c '\dt'
```

Do not blindly truncate every table in `coprocessor`; that would remove
`keys`. The intended cleanup shape is:

```sql
BEGIN;
-- Preserve keys and activated KMS key rows.
-- TRUNCATE benchmark_timing_events, computations, pbs_computations, ... RESTART IDENTITY CASCADE;
COMMIT;
```

After cleanup, restart only the non-keygen runtime services and GPU workers,
then validate that `select count(*) from keys;` is still nonzero and that no
`computations` rows remain with `is_completed = false`.

If the host chain itself still contains stale benchmark blocks near the current
tip, do not restart the host listener against an empty block-history table. On
2026-07-09 this caused the listener to process the latest stale auction block,
detect missing ancestors, and backfill its 50-block reorg recovery window. The
symptom was `computations` being repopulated immediately while `verify_proofs`
and `input_handles` stayed empty. The working lightweight reset sequence was:

```bash
test-suite/fhevm/scripts/gpu-workers.sh stop
docker stop coprocessor-host-listener coprocessor-host-listener-consumer coprocessor-host-listener-poller

# Mine more empty blocks than the host-listener reorg recovery window, currently 50.
curl -fsS -X POST -H 'Content-Type: application/json' \
  --data '{"jsonrpc":"2.0","method":"anvil_mine","params":["0x3c"],"id":1}' \
  http://127.0.0.1:8545

docker exec listener-redis redis-cli FLUSHALL

latest_hex="$(curl -fsS -X POST -H 'Content-Type: application/json' \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  http://127.0.0.1:8545 | node -pe "JSON.parse(fs.readFileSync(0,'utf8')).result")"
latest=$((latest_hex))

docker exec -e PGPASSWORD=postgres coprocessor-and-kms-db \
  psql -U postgres -d coprocessor -v ON_ERROR_STOP=1 -c "
TRUNCATE
  allowed_handles, allowed_handles_branch, benchmark_timing_events,
  bridge_handle_events, ciphertext_digest, ciphertext_digest_branch,
  ciphertexts, ciphertexts_branch, ciphertexts128, ciphertexts128_branch,
  computations, computations_branch, delegate_user_decrypt, dependence_chain,
  drift_revert_signal, fallback_granted_events, handle_bridged_events,
  host_chain_blocks_valid, host_chain_consumer_blocks,
  host_listener_poller_state, input_blobs, input_handles, pbs_computations,
  pbs_computations_branch, transactions, verify_proofs
RESTART IDENTITY CASCADE;
INSERT INTO host_listener_poller_state(chain_id,last_caught_up_block)
VALUES (12345,$latest);
"

docker start coprocessor-host-listener coprocessor-host-listener-consumer coprocessor-host-listener-poller
test-suite/fhevm/scripts/gpu-workers.sh start
```

After this sequence, it is acceptable for `host_chain_blocks_valid` to contain
about 50 empty padding blocks. `computations`, `verify_proofs`, and
`input_handles` should remain zero until the next benchmark submits fresh
transactions.

This matters for proof caches too. ERC20 and auction proof caches are
deployment-specific: after a full volume reset and new KMS key/context, the
benchmark correctly rejects the old cache metadata and regenerates proofs. A
lightweight cleanup that preserves `keys` and activation rows should allow
future runs to reuse caches instead of spending time on CPU proof generation.
However, the current JSON proof caches only store the contract calldata needed
to resubmit already-generated encrypted inputs. They do not store the full ZK
worker input or the post-verification DB/object-store material. If cleanup
truncates `input_handles`, `ciphertexts`, or `input_blobs`, replaying cached
transaction calldata on the same chain may not emit fresh gateway proof
requests because the on-chain handles were already verified earlier. In that
state the host listener can ingest `VerifyInput` events and create
`computations`, but `verify_proofs`, `input_handles`, and `ciphertexts` remain
empty, leaving TFHE with no input material to consume.

Do not try to reconstruct `verify_proofs` from the benchmark JSON
`inputProof` field. That field is the compact proof calldata passed to the
contract, not the full `ciphertextWithZKProof` blob expected by
`zkproof_worker`. A direct DB seed attempt failed with
`error deserializing ciphertext: "the size limit has been reached"` and did not
restore usable input material. For future cache reuse after cleanup, either
preserve the verified input tables/object-store state or add a dedicated cache
that snapshots/restores the full verified-input material. If those rows/objects
are already gone, regenerate proofs against the current deployment; on this host
use proof-generation concurrency 10.

On 2026-07-09, the first `v0.14.0-02` run used the stale
`.fhevm/runtime/erc20-benchmark-proofs-1000.json` after a full key reset. The
cache was deployment-incompatible, so the benchmark fell back to generation.
With `ERC20_BENCH_PROOF_CONCURRENCY=60`, multiple child proof workers hit
V8/WASM fatal errors (`unreachable code` and
`Check failed: current == end_slot_index`). The run was interrupted, orphaned
proof workers were killed, and the local `/bench-e2e-basic` skill was updated
to use `ERC20_BENCH_PROOF_CONCURRENCY=10`.

The next `v0.14.0-02` attempt generated a deployment-compatible 960-entry ERC20
cache during the 16 TPS fixed-rate scenario and copied it back to
`.fhevm/runtime/erc20-benchmark-proofs-1000.json`. The 1000-transfer max
scenario then exposed a benchmark harness issue: ERC20 proof cache reuse was
all-or-nothing, so a 960-entry compatible cache was rejected instead of reused
and extended. The local ERC20 benchmark was patched to treat compatible caches
as partially reusable: attach the cached ERC20 contract, reuse the cached
senders/recipients/proofs, generate only missing tail entries at concurrency 10,
and rewrite the cache at the larger transfer count.

The same run completed all ERC20 scenarios, but auction proof generation at the
default `AUCTION_BENCH_PROOF_CONCURRENCY=16` hung at 275/300 generated proofs:
Node worker threads were sleeping, no DB work was pending, and both GPUs were
idle. The auction benchmark was patched to support compatible partial proof
caches and periodic checkpoint writes, and the local `/bench-e2e-basic` skill
was changed to set `AUCTION_BENCH_PROOF_CONCURRENCY=10` for both fixed-rate and
max-throughput auction scenarios. If auction generation is interrupted again,
preserve `/tmp/auction-benchmark-proofs-420.json` from the test-suite container
or copy it back to `.fhevm/runtime/` before retrying.

## 2026-07-09 Reset Back to Known-Good v0.14 GPU Deployment

Started after the `v0.14.0-00` full benchmark attempt left the deployment in an
inconsistent state. The reset target is the previously working configuration
from "Successful Clean Redeploy With Compressed XOF":

- lock file: `.fhevm/state/locks/gpu-v014-all-services.json`;
- local patched image tags:
  `HOST_VERSION=fhevm-local`,
  `CONNECTOR_KMS_WORKER_VERSION=fhevm-local`,
  `CONNECTOR_TX_SENDER_VERSION=fhevm-local`;
- released images:
  coprocessor/connector/relayer `v0.14.0-0`,
  KMS core `v0.14.0-1`,
  gateway/listener/test-suite `519c60c`;
- host GPU workers only: TFHE on GPU0, ZK and SNS on GPU1;
- Docker CPU worker containers stopped before tests.

Important pitfall from the failed full-suite attempt: do not try to repair the
old deployment by reusing the existing ERC20 proof cache unless the deployment
key/context and verified ciphertext material are also restored. Handles and
proof calldata alone are not enough for TFHE.

Reset execution notes:

- Stopped host GPU workers before touching Docker state.
- Ran `test-suite/fhevm/fhevm-cli down`; it removed all FHEVM containers and
  the persistent Docker volumes (`fhevm_db`, `fhevm_fhevm_kms_core_keys`,
  `fhevm_keys-cache`, `fhevm_minio_secrets`, `fhevm_redis_data`,
  `fhevm_relayer_postgres_data`).
- Verified no FHEVM containers, volumes, or networks remained.
- Preserved lock files under `.fhevm/state/locks`.
- Moved old proof caches under `.fhevm/runtime/cache-archive/` only. They are
  deliberately not active runtime caches for the fresh deployment.
- Started the clean deploy with the pinned lock and overrides. Applying the
  `CompressedKeySet` enum extension during early `base`/`gateway-deploy` is too
  early: `kms-connector` database does not exist until the connector migration
  step.
- The reset again hit the known `cgr.dev` auth failure when the CLI tried to
  rebuild overridden KMS connector services:

```text
failed to fetch anonymous token:
https://cgr.dev/token?scope=repository%3Azama.ai%2Fglibc-dynamic%3Apull
401 Unauthorized
```

Do not rebuild here. The local patched images already exist as
`ghcr.io/zama-ai/fhevm/kms-connector/kms-worker:fhevm-local` and
`ghcr.io/zama-ai/fhevm/kms-connector/tx-sender:fhevm-local`; resume the CLI
after confirming those tags.

During the 2026-07-09 reset, those `fhevm-local` connector image tags were
missing after the failed Docker build. Attempting a host cargo rebuild of
`kms-worker` and `tx-sender` reached `kms-grpc` but failed with:

```text
protoc failed: metastore-status.v1.proto: This file contains proto3 optional
fields, but --experimental_allow_proto3_optional was not set.
```

If this happens, fix the protobuf compiler/build-script environment before
building replacement connector images.

On this Ubuntu image, `/usr/bin/protoc` is `libprotoc 3.12.4`. The patched
connector source can be built on the host by adding
`.protoc_arg("--experimental_allow_proto3_optional")` to the transient Cargo
checkout at:

```text
/home/ubuntu/.cargo/git/checkouts/kms-2da0d388649b87b8/b9087af/core/grpc/build.rs
```

This is a local build workaround only. After the host binaries are built, avoid
the blocked `cgr.dev` runtime base by creating `fhevm-local` connector images
from the already-pulled released runtime images and copying in only the rebuilt
`kms-worker`/`tx-sender` binaries.

If the KMS `build.rs` patch still does not propagate the flag, patch the local
Cargo registry copy of `prost-build` in the `protoc` command construction block
to append:

```rust
cmd.arg("--experimental_allow_proto3_optional");
```

Then rebuild. This is also only a host-machine workaround for rebuilding the
patched connector images.

During the 2026-07-09 reset both `prost-build-0.13.5` and `prost-build-0.14.3`
were present in `~/.cargo/registry/src`; patch both if unsure which one Cargo
will reuse.

The same host rebuild then hit a Rust query-depth limit in
`alloy-signer-aws-1.0.38`. The local workaround was to add the following to the
crate root at
`~/.cargo/registry/src/index.crates.io-*/alloy-signer-aws-1.0.38/src/lib.rs`:

```rust
#![recursion_limit = "256"]
```

The `tx-sender` binary can hit the same limit in its async main on this host.
For the local patched build, add the same crate-level attribute at the top of
`kms-connector/crates/tx-sender/src/bin/tx_sender.rs`.

With those local build workarounds, the patched connector binaries built and
the local runtime images were created successfully from released v0.14.0-0
bases:

```text
ghcr.io/zama-ai/fhevm/kms-connector/kms-worker:fhevm-local sha256:06e95da210336d046ae3e0ce329d1bdb6e2194d901e0752b221e916308569ef7
ghcr.io/zama-ai/fhevm/kms-connector/tx-sender:fhevm-local  sha256:4c8648769430a6f2d0dacdc6a51f0f32b4eb238055c5793adef6a99ea00eb72c
```

`fhevm-cli up --resume` still tried to build services listed in
`state.overrides`, even after the local image tags existed and the generated
compose file pointed at them. To continue the reset without `cgr.dev` access,
remove only the `kms-connector` override entry from `.fhevm/state/state.json`
and remove the generated `build:` stanzas from
`.fhevm/runtime/compose/kms-connector.yml`. Keep
`CONNECTOR_KMS_WORKER_VERSION=fhevm-local` and
`CONNECTOR_TX_SENDER_VERSION=fhevm-local` in the persisted version bundle.

Also set `pull_policy: never` on those two generated connector services. Without
it, `docker compose up` may try to pull `:fhevm-local` from GHCR and fail with
`manifest unknown` instead of using the local image tag.

## 2026-07-09 Benchmark Timing Accuracy Fix

The fixed-rate benchmark reports previously mixed test-container `Date.now()`
timestamps with Postgres worker timestamps. This can produce negative latency
values when the container and DB clocks differ, and it can overstate latency
when the harness assigns an end time from a later polling loop.

Local benchmark changes made after observing negative/outlier latencies:

- ERC20 and auction benchmarks now create/use a benchmark-owned
  `benchmark_timing_events` table in the coprocessor Postgres DB. Harness
  events such as `run_started`, `tx_submitted`, `receipt_observed`,
  `sns_ready_observed`, and `completion_observed` are recorded with Postgres
  `clock_timestamp()`.
- ERC20 compute latency now starts from `transactions.created_at`, which is the
  host-listener DB timestamp for the mined host transaction, instead of local
  receipt polling time. Worker stage ends use DB timestamps from
  `computations.completed_at`, `ciphertexts.created_at`, and
  `pbs_computations.completed_at`.
- Both ERC20 and auction reports include proof verification timing from
  `verify_proofs.created_at` to `verify_proofs.verified_at`, grouped by host
  transaction ID.
- ERC20 end-to-end latency now includes
  `proofVerifiedToSnsCt128Computed`, using `verify_proofs.verified_at`
  (falling back to `input_handles.created_at` when needed) to
  `pbs_computations.completed_at` for the transfer result handle. This is the
  preferred "input proof verified to SNS/noise-squashed CT available" metric.
- Auction latency is now per bid. The previous benchmark assigned every bid
  the same scenario-tail completion timestamp from the polling loop. The fixed
  benchmark groups completion timestamps by `transaction_id` so distributions
  show each bid's own completion rather than the last bid in the run.
- The main `latency` section in reports uses DB timestamps. Local-clock deltas
  are retained under `harnessClockDiagnostics` only for drift debugging.
- The `bench-e2e-basic` skill now exports raw timing rows to
  `timing-events/*.json` beside each scenario report. Use the report's
  `benchmarkRunId` to query the same rows directly:

```sql
SELECT *
FROM benchmark_timing_events
WHERE run_id = '<benchmarkRunId>'
ORDER BY event_at;
```

## 2026-07-10 Wave2 Rebase And GPU Smoke

The benchmark history was preserved on
`e2e-erc20-transfer-benchmark` at `9eae65572`. A separate branch,
`e2e-erc20-transfer-benchmark-wave2`, was created and rebased onto
`origin/antoniu/block_context_rerand_wave2` at `9d8ffaddb`.

Important rebase resolutions:

- Keep wave2 block/fork-context selection and boundary rerandomization. The
  benchmark multi-DCID acquisition now locks up to
  `--dependence-chains-per-batch` chains, selects the earliest eligible block
  context among them, and executes the complete same-block closure.
- Keep `--work-items-batch-size` as a legacy CLI compatibility option. Wave2
  does not use it to truncate a block context because partial block execution
  would violate the block-scoped execution invariant.
- Use a typed runtime `query_as` for the merged multi-DCID block-context query.
  The old local v0.14 database does not contain all wave2 migrations, so it
  cannot be used to regenerate the complete workspace SQLx offline cache.
- Match `test-suite/e2e/package.json` to the repository lock file (`pg
  ^8.20.0` and one `@types/pg` dependency). A conflict resolution that selected
  `pg ^8.22.0` made the host-listener build script fail in `npm ci`.
- Return the execution GPU index with each scheduler partition result and set
  the matching server key on the async scheduler thread before propagating
  working ciphertexts. Without this, a wave2 GPU run panicked after key load
  with `The server key was not properly initialized`.

The existing long-running v0.14 deployment is not a wave2 deployment: its
published host listener does not populate the new branch-context pipeline and
its database lacks later wave2 migrations. Do not replace only its TFHE worker
with a wave2 binary. The smoke tests below therefore used the wave2 test
harness, which creates an isolated Postgres database and applies the branch's
current migrations; the existing deployment and proof caches were left intact.

GPU test fixtures are stored with Git LFS. A checkout containing 130-byte
pointer files fails misleadingly with `the size limit has been reached`. Fetch
the three files required by the block-scoped TFHE tests before running them:

```bash
git lfs pull --include='coprocessor/fhevm-engine/fhevm-keys/xof-keyset,coprocessor/fhevm-engine/fhevm-keys/xof-cks,coprocessor/fhevm-engine/fhevm-keys/pp' --exclude=''
```

Validated on GPU0 (`NVIDIA H100 PCIe`, 114 streaming multiprocessors):

```bash
cd /home/ubuntu/fhevm/coprocessor/fhevm-engine
CUDA_VISIBLE_DEVICES=0 TFHE_WORKER_EVENT_TYPE_MATRIX=uint64 \
  cargo test -p tfhe-worker --release --features=gpu --lib \
  tests::block_scoped::test_block_scoped_simple_add -- \
  --exact --nocapture --test-threads=1

CUDA_VISIBLE_DEVICES=0 TFHE_WORKER_EVENT_TYPE_MATRIX=uint64 \
  cargo test -p tfhe-worker --release --features=gpu --lib \
  tests::block_scoped::test_cross_chain_same_block_closure -- \
  --exact --nocapture --test-threads=1
```

Both tests passed. The simple three-operation graph reported 29 ms scheduler
time. The cross-chain test acquired two DCIDs together, completed its
three-operation graph in 49 ms scheduler time, uploaded all three results, and
marked both chains processed. OTLP export errors in these test logs are
expected because the isolated harness does not start a trace collector.

## 2026-07-10 Wave2 Coprocessor Redeploy With Preserved KMS

The wave2 benchmark branch cannot run against the published v0.14.0-0
coprocessor listeners or the partially migrated v0.14 database. The working
upgrade kept the existing KMS core, KMS connector, relayer, MinIO key material,
gateway, host chain, and key activation state, and rebuilt only coprocessor
components.

The preserved key state was:

- KMS core `v0.14.0-1`, connector images `fhevm-local`, and relayer
  `v0.14.0-0`;
- key sequence `1`, gateway key ID `04...01`;
- one activated key event and one CRS;
- a 444,303,777-byte `compressed_xof_keyset` in `coprocessor.keys`.

Build all coprocessor binaries from the wave2 checkout:

```bash
cd /home/ubuntu/fhevm/coprocessor/fhevm-engine
env \
  CUDA_PATH=/usr/local/cuda-12.2 \
  LD_LIBRARY_PATH=/usr/local/cuda-12.2/lib64 \
  CUDA_MODULE_LOADER=EAGER \
  CC=/usr/bin/gcc-11 \
  CXX=/usr/bin/g++-11 \
  CUDAHOSTCXX=/usr/bin/g++-11 \
  RUSTFLAGS='-C target-cpu=native' \
  SQLX_OFFLINE=true \
  cargo build \
    -p tfhe-worker -p sns-worker -p zkproof-worker \
    -p host-listener -p gw-listener -p transaction-sender \
    --release --features=gpu --bins
```

The `cgr.dev` runtime bases are still unavailable without separate registry
authentication. The local runtime build context at
`.fhevm/runtime/local-images/wave2/` copies the host-built listener/sender
binaries and current migration files into the already-pulled v0.14.0-0 runtime
images. The resulting tags are:

```text
ghcr.io/zama-ai/fhevm/coprocessor/db-migration:wave2-local
ghcr.io/zama-ai/fhevm/coprocessor/host-listener:wave2-local
ghcr.io/zama-ai/fhevm/coprocessor/gw-listener:wave2-local
ghcr.io/zama-ai/fhevm/coprocessor/tx-sender:wave2-local
```

Persist those tags in `.fhevm/runtime/env/versions.env` and
`.fhevm/state/state.json`. Do not change the KMS connector, KMS core, or relayer
versions during this coprocessor-only redeploy.

For the existing single-operator test chain, 60 empty blocks were mined and
block `6804` was selected as the common branch activation and wave2 cutover:

```text
FHEVM_BRANCH_ACTIVATION_BLOCK=6804
FHEVM_BRANCH_CUTOVER_BLOCK=6804
```

Both values are persisted in `.fhevm/runtime/env/coprocessor.env`, which is
loaded by all listener containers and host worker units. Before migration, stop
the host GPU workers and all five coprocessor listener/sender containers, flush
`listener-redis`, truncate transient workload tables while preserving `keys`,
`crs`, `kms_key_activation_events`, `kms_crs_activation_events`,
`gw_listener_last_block`, `host_chains`, and `versioning`, then seed
`host_listener_poller_state` at the padded host-chain tip.

The local migration image successfully applied the seven missing branch-context
migrations `20260610150100` through `20260610150700` and the four migrations
`20260709100000` through `20260710120000`. Verify all of the following before
starting workers:

```sql
SELECT max(version) FROM _sqlx_migrations; -- 20260710120000
SELECT to_regclass('public.s3_canonical_repair_queue');
SELECT count(*), max(octet_length(compressed_xof_keyset)) FROM keys;
SELECT count(*) FROM kms_key_activation_events WHERE status = 'activated';
```

The old benchmark JSON caches were moved to
`.fhevm/runtime/cache-archive/20260710-pre-wave2-6804/`. They must not be copied
back into the test container after verified input tables are truncated. The
first full wave2 suite must regenerate and checkpoint deployment-compatible
caches with proof concurrency 10.

### Wave2 Benchmark Table Selection

After cutover, completed work and ciphertext publication timestamps live in
the branch tables. A benchmark that queries only the legacy tables will report
zero progress even while the GPU pipeline is complete. The wave2 benchmark
queries use:

- `computations_branch` for TFHE completion;
- `ciphertexts_branch` for TFHE ciphertext materialization;
- `pbs_computations_branch` for SNS CT128 computation;
- `ciphertext_digest_branch` for completed S3 upload/readiness.

`transactions`, `input_handles`, `verify_proofs`, and
`benchmark_timing_events` remain shared/non-branch tables.

### Cold-Start GPU Key Handoff

Two thread-local server-key handoffs are required by the wave2 TFHE path:

1. When a scheduler partition completes, install the partition GPU's server
   key on the Tokio continuation before propagating its outputs.
2. After boundary inputs are materialized in `spawn_blocking`, install the same
   GPU server key on the Tokio continuation before calling
   `DFComponentGraph::add_input`.

Without the second handoff, the first real job after a TFHE worker restart can
panic with `The server key was not properly initialized`, even though later
unit-test jobs may pass because their test thread already has a key. The exact
GPU block-scoped simple-add test passes after both handoffs.

### SNS Immediate Upload Dispatch

The initial wave2 smoke showed CT128 computation in 124 ms but SNS readiness
only after 107 seconds. `compute_task` dispatched `UploadJob::Normal` before
the SNS database transaction committed `pbs_computations_branch.is_completed`.
The uploader's provenance guard correctly rejected that early job, leaving the
row for the 120-second regular DB rescan.

Dispatch normal upload jobs only after the transaction has persisted CT128,
marked PBS complete, inserted the digest row, and committed. Channel pressure
or S3 failure still falls back to the durable database rescan. Validation:

- SNS GPU unit tests: 31 passed, 3 ignored;
- 100-ciphertext SNS batch execution: passed;
- fresh cold-start ERC20 smoke: passed end to end;
- smoke SNS upload/readiness followed CT128 computation by about 0.5 seconds,
  rather than waiting for the 120-second recovery tick.

The cold smoke still included about 13 seconds for the TFHE worker's first
444 MB compressed-XOF key load. Run one smoke before latency benchmarks so the
full suite measures warm steady-state execution rather than process startup.

### Backup Pitfall

`pg_dump -Fc` of the complete coprocessor database failed while serializing the
large `keys` row because Postgres could not enlarge a COPY string buffer beyond
roughly 742 MB. Do not treat this as key corruption. For this in-place cleanup,
the key/CRS/activation tables were excluded from truncation and verified before
and after migration. A logical schema/workload dump excluding the four large
key-material tables was stored under
`.fhevm/reset-backups/20260710-153957Z-wave2-coprocessor/`.

## Known Caveats

- The `fhevm-cli test` flow expects the Docker stack to exist and may inspect
  Docker worker logs. Replacing workers with host processes is useful for GPU
  validation but can confuse stack readiness assumptions.
- `./fhevm-cli up --build` may fail without `cgr.dev` registry access because
  it pulls `cgr.dev/zama.ai/postgres:17`.
- Host-run worker processes need non-conflicting health and metrics ports.
- For host-run workers, `localhost:9000` may not work for MinIO. Use the
  `fhevm-minio` container IP on the Docker network.
- Keygen confirmation takes time because the connector waits for block
  confirmations before forwarding later stages.
