# Leptos + Axum project template — multi-stage build
# Stage 1: build server binary + site (WASM, assets)
FROM rust:bookworm AS builder

# Use nightly (required by Leptos) and wasm32 target
RUN rustup default nightly && rustup target add wasm32-unknown-unknown

# Install cargo-leptos and sass (for SCSS)
RUN cargo install cargo-leptos --locked
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && npm install -g sass \
    && apt-get purge -y curl \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy workspace and dependency manifests first for better layer caching
COPY Cargo.toml Cargo.lock* rust-toolchain.toml ./
COPY app/Cargo.toml ./app/
COPY frontend/Cargo.toml ./frontend/
COPY server/Cargo.toml ./server/

# Copy source (needed for cargo leptos build)
COPY app ./app
COPY frontend ./frontend
COPY server ./server
COPY style ./style
COPY public ./public

# Release build: server binary + static site (WASM, CSS, assets)
ENV LEPTOS_ENV=PRODUCTION
RUN cargo leptos build --release

# Stage 2: minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Server binary
COPY --from=builder /build/target/release/server /app/server
# Static site (WASM, pkg, CSS, public assets)
COPY --from=builder /build/target/site /app/target/site

# Listen on all interfaces in container
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000

CMD ["/app/server"]
