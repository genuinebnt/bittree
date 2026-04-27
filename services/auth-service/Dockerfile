FROM lukemathwalker/cargo-chef:latest-rust-1.91 AS chef
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends lld clang libssl-dev pkg-config ca-certificates \
  && update-ca-certificates \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt \
    SSL_CERT_DIR=/etc/ssl/certs \
    CARGO_HTTP_CAINFO=/etc/ssl/certs/ca-certificates.crt

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin auth-service
COPY . .
RUN cargo build --release --bin auth-service

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/auth-service auth-service
COPY services/auth-service/config config
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./auth-service"]
