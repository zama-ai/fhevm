# RUST_IMAGE_VERSION is injected by CI from rust-toolchain.toml (see coprocessor-docker-build.yml)
ARG RUST_IMAGE_VERSION

# Stage 1: Build the coprocessor proxy
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

# Build the coprocessor-proxy binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
# No sccache: with RUSTC_WRAPPER=sccache, cc also runs the vendored OpenSSL's assembler probes,
# `cc ... -o /dev/null`, through sccache. Up to v0.15.0, sccache writes a cached output to a
# temporary file next to the target and renames it over the target: as root, it replaces /dev/null,
# and probes running meanwhile fail and fall back to non-AVX code. OpenSSL then mixes AVX and
# non-AVX AES-GCM code and writes TLS records with wrong tags. Same bug as a non-root user:
# https://github.com/mozilla/sccache/issues/2288. sccache v0.16.0 skips outputs to /dev/null; the
# rust-glibc golden image pins v0.15.0.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    cargo fetch && \
    SQLX_OFFLINE=true BUILD_ID=$(cat BUILD_ID) cargo build --profile=${CARGO_PROFILE} -p coprocessor-proxy && \
    cp target/${CARGO_PROFILE}/coprocessor-proxy /tmp/coprocessor-proxy

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/coprocessor-proxy /usr/local/bin/coprocessor-proxy

USER fhevm:fhevm

ENTRYPOINT ["/usr/local/bin/coprocessor-proxy"]

FROM prod AS dev
