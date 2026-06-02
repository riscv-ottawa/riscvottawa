ARG RUST_VERSION=1.95
ARG CARGO_LEPTOS_VERSION=0.3.6

# ---------- Builder ----------
# Cache mounts below need BuildKit (DOCKER_BUILDKIT=1) or podman, which use
# them by default. They keep the cargo registry and the compiled target/
# directory between builds so only changed code is recompiled.
FROM rust:${RUST_VERSION}-bookworm AS builder

# cargo-leptos lives in its own layer, rebuilt only when the version arg changes.
ARG CARGO_LEPTOS_VERSION
RUN rustup target add wasm32-unknown-unknown \
 && cargo install --locked cargo-leptos --version ${CARGO_LEPTOS_VERSION}

WORKDIR /app
COPY . .
# target/ is a cache mount, so its contents don't survive into the image layer;
# copy the artifacts we need out to /out within the same step.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo leptos build --release \
 && mkdir -p /out \
 && cp target/release/riscvottawa /out/riscvottawa \
 && cp -r target/site /out/site

# ---------- Runtime ----------
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl \
 && rm -rf /var/lib/apt/lists/* \
 && groupadd --system app \
 && useradd --system --gid app --home-dir /app --shell /usr/sbin/nologin app

WORKDIR /app
COPY --from=builder --chown=app:app /out/riscvottawa /app/riscvottawa
COPY --from=builder --chown=app:app /out/site /app/site
COPY --chown=app:app content /app/content

USER app

ENV LEPTOS_OUTPUT_NAME=riscvottawa \
    LEPTOS_SITE_ROOT=/app/site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -fsS http://localhost:3000/ >/dev/null || exit 1
CMD ["/app/riscvottawa"]
