# RUST_IMAGE_VERSION is injected by CI from rust-toolchain.toml (see coprocessor-docker-build.yml)
ARG RUST_IMAGE_VERSION

# Stage 1: Build Host Listener
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

ARG CARGO_PROFILE=release
ARG SCCACHE_BUCKET
ARG SCCACHE_REGION=eu-west-3
ARG SCCACHE_S3_PREFIX

# Preview-env only: compile this image as a different release, so a blue/green
# cutover has a second side to play without waiting for one to be published.
# Set both or neither - see the feature pair in fhevm-engine-common/Cargo.toml.
# Empty is the production path and leaves the image unchanged.
ARG BUILD_CONSENSUS_VERSION=
ARG BUILD_STACK_VERSION=
ENV SCCACHE_IDLE_TIMEOUT=0
ENV SCCACHE_BUCKET=${SCCACHE_BUCKET}
ENV SCCACHE_REGION=${SCCACHE_REGION}
ENV SCCACHE_S3_KEY_PREFIX=${SCCACHE_S3_PREFIX}
ENV AWS_REGION=${SCCACHE_REGION}

USER root

WORKDIR /app

COPY listener ./listener
COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY host-contracts/rust_bindings ./host-contracts/rust_bindings
COPY shared/ciphertext-attestation ./shared/ciphertext-attestation
COPY shared/block-manifest ./shared/block-manifest
COPY .git/HEAD ./coprocessor/fhevm-engine/BUILD_ID

WORKDIR /app/coprocessor/fhevm-engine

# Build host_listener binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then \
      export RUSTC_WRAPPER=sccache; \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
    cargo fetch && \
    version_override=""; \
    if [ -n "${BUILD_CONSENSUS_VERSION}" ] || [ -n "${BUILD_STACK_VERSION}" ]; then \
      : "${BUILD_CONSENSUS_VERSION:?must be set together with BUILD_STACK_VERSION}"; \
      : "${BUILD_STACK_VERSION:?must be set together with BUILD_CONSENSUS_VERSION}"; \
      version_override="--features fhevm-engine-common/consensus-version-override,fhevm-engine-common/stack-version-override"; \
    fi && \
    SQLX_OFFLINE=true BUILD_ID=$(cat BUILD_ID) cargo build ${version_override} --profile=${CARGO_PROFILE} -p host-listener && \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then sccache --show-stats; fi && \
    cp target/${CARGO_PROFILE}/host_listener /tmp/host_listener && \
    cp target/${CARGO_PROFILE}/host_listener_poller /tmp/host_listener_poller && \
    cp target/${CARGO_PROFILE}/host_listener_consumer /tmp/host_listener_consumer

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/host_listener /usr/local/bin/host_listener
COPY --from=builder --chown=fhevm:fhevm /tmp/host_listener_poller /usr/local/bin/host_listener_poller
COPY --from=builder --chown=fhevm:fhevm /tmp/host_listener_consumer /usr/local/bin/host_listener_consumer

USER fhevm:fhevm

# No ENTRYPOINT - consumers specify the full command (binary + args) in docker-compose

FROM prod AS dev
