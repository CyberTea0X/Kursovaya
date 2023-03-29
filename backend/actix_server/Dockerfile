FROM rust:1.68.2 AS build
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV TARGET_ENV x86_64-unknown-linux-musl
WORKDIR /app
COPY . .
RUN apt-get update \
    && apt-get -y install \
        ca-certificates \
        cmake \
        musl-tools \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add ${TARGET_ENV} \
    && USER=root cargo new --bin main
RUN cargo build --target ${TARGET_ENV} --release && \
    strip target/release/actix_server && \
    rm -rf src/ tests/ target/release/.fingerprint/actix_server-*

FROM debian:buster-slim
WORKDIR /app

COPY --from=build /app/target/release/actix_server .
CMD ["./actix_server"]