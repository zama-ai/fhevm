# syntax=docker/dockerfile:1

# RUST_IMAGE_VERSION is injected by CI from rust-toolchain.toml (see listener-docker-build.yml).
# The default below is the fallback for local docker-compose builds, which do NOT inject it.
ARG RUST_IMAGE_VERSION=1.97.1

# ── Builder (golden rust-glibc image ships the pinned toolchain + build deps) ──
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

# Build as root; the golden image already provides the toolchain, build deps
# (openssl-dev, gcc, build-base, ...) and the unprivileged fhevm user.
USER root

WORKDIR /app
COPY listener/ /app/

ENV SQLX_OFFLINE=true

# Build with BuildKit caches for cargo registry and target directory
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release -p listener_core --features iam-auth && \
    cp target/release/listener_core /tmp/listener_core

# ── Runtime (minimal Chainguard glibc) ──────────────────────────────────
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/listener_core /app/listener_core
COPY --from=builder --chown=fhevm:fhevm /app/crates/listener_core/migrations/ /app/migrations/

USER fhevm:fhevm
WORKDIR /app

ENTRYPOINT ["/app/listener_core"]
CMD ["--config", "/config.yaml"]

# ── Dev (alias of prod; required because common-docker.yml builds --target dev) ──
FROM prod AS dev
