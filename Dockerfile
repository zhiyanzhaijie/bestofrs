FROM rust:1.90.0-bookworm AS chef

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef --no-confirm
RUN cargo binstall dioxus-cli -y --force --version 0.7.4
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN dx bundle --package ui --release --fullstack --force-sequential

FROM debian:bookworm-slim AS runtime
WORKDIR /app

ARG PG_MAJOR=16
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates curl gnupg \
    && install -d /usr/share/keyrings \
    && curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc \
    | gpg --dearmor -o /usr/share/keyrings/pgdg.gpg \
    && echo "deb [signed-by=/usr/share/keyrings/pgdg.gpg] http://apt.postgresql.org/pub/repos/apt bookworm-pgdg main" \
    > /etc/apt/sources.list.d/pgdg.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
    postgresql-client-${PG_MAJOR} \
    && apt-get purge -y --auto-remove curl gnupg \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/dx/ui/release/web/server ./server
COPY --from=builder /app/target/dx/ui/release/web/public ./public

ENTRYPOINT ["./server"]
