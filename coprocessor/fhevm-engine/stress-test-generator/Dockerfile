# RUST_IMAGE_VERSION is injected by CI from rust-toolchain.toml (see coprocessor-docker-build.yml)
ARG RUST_IMAGE_VERSION

# Stage 0: Build Stress-Tool
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

ARG CARGO_PROFILE=release

USER root

WORKDIR /app

COPY listener ./listener
COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY host-contracts/rust_bindings ./host-contracts/rust_bindings
COPY shared/ciphertext-attestation ./shared/ciphertext-attestation
# Solana path-dependencies of the fhevm-engine workspace (host-listener and
# tfhe-worker manifests reference them; `cargo build -p <crate>` loads every
# member manifest). Same copy set as the other fhevm-engine Dockerfiles.
COPY solana/Cargo.toml ./solana/Cargo.toml
COPY solana/crates ./solana/crates
COPY solana/programs ./solana/programs
COPY solana/runtime-tests ./solana/runtime-tests

WORKDIR /app/coprocessor/fhevm-engine

# Build stress_generator binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    cargo fetch && \
    SQLX_OFFLINE=true cargo build --profile=${CARGO_PROFILE} -p stress-test-generator && \
    cp target/${CARGO_PROFILE}/stress_generator /tmp/stress_generator

# Stage 2: Runtime image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/stress_generator /usr/local/bin/stress_generator
USER fhevm:fhevm

CMD ["/usr/local/bin/stress_generator"]

FROM prod AS dev
