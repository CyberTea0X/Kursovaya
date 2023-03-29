FROM rust:1.68.2-slim-buster AS build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /app
COPY . .
RUN cargo build --release && \
    strip target/release/actix_server && \
    rm -rf src/ tests/ target/release/.fingerprint/actix_server-*

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    apt-get -y install libpq-dev && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=build /app/target/release/actix_server .
CMD ["./actix_server"]