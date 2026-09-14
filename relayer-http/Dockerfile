# syntax=docker/dockerfile:1
# Build from the repository root:
#   docker build -f relayer-http/Dockerfile --build-arg RUST_IMAGE_VERSION=1.97.1 -t relayer-http .
# RUST_IMAGE_VERSION has no default on purpose: the toolchain version has one source of truth,
# relayer-http/rust-toolchain.toml (the CI template reads it).
ARG RUST_IMAGE_VERSION

FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS build
USER root
WORKDIR /app

# The crate depends on kms-connector/crates/api by path, and that crate's workspace references its own
# siblings: copy them at the same relative places as in the repository so every `../` path resolves.
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY host-contracts/rust_bindings ./host-contracts/rust_bindings
COPY shared ./shared
COPY kms-connector ./kms-connector
COPY relayer-http ./relayer-http

WORKDIR /app/relayer-http
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/relayer-http/target,sharing=locked \
    cargo build --locked --release -p relayer-http && \
    cp target/release/relayer-http /tmp/relayer-http

FROM cgr.dev/zama.ai/glibc-dynamic:16.2.0 AS prod
COPY --from=build /etc/group /etc/group
COPY --from=build /etc/passwd /etc/passwd
COPY --from=build --chown=fhevm:fhevm /tmp/relayer-http /app/bin/relayer-http
USER fhevm:fhevm
WORKDIR /app

# The configuration is mounted at run time; the API keys come from the env vars it names.
ENTRYPOINT ["/app/bin/relayer-http"]
CMD ["/app/config/config.yaml"]

FROM prod AS dev
