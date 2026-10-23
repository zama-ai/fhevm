# Stage 1: Build PRF service
# RUST_IMAGE_VERSION is injected by CI from rust-toolchain.toml (see coprocessor-docker-build.yml)
ARG RUST_IMAGE_VERSION
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
COPY shared/block-manifest ./shared/block-manifest
COPY shared/fhevm-sig-auth ./shared/fhevm-sig-auth
COPY .git/HEAD ./coprocessor/fhevm-engine/BUILD_ID

WORKDIR /app/coprocessor/fhevm-engine

# Build prf-service, and the spike's stand-in pad-worker and connector (prf-standins)
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    cargo fetch && \
    SQLX_OFFLINE=true BUILD_ID=$(cat BUILD_ID) cargo build --profile=${CARGO_PROFILE} -p prf-service -p prf-standins && \
    cp target/${CARGO_PROFILE}/prf-service target/${CARGO_PROFILE}/standin-pad-worker \
       target/${CARGO_PROFILE}/standin-connector /tmp/

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/prf-service /tmp/standin-pad-worker /tmp/standin-connector /usr/local/bin/

USER fhevm:fhevm

CMD ["/usr/local/bin/prf-service"]

FROM prod AS dev
