# The rust:<version>-slim tag pulls
# an image that contains just the minimum
# packages needed to run Rust. This saves plenty of space,
# but fails in environments that require deployments beyond just your rust image
# The rust:<version>-slim-bullseye tag pulls an image built upon Debian 11 branch,
# which is the current stable distro
# The rust:<version>slim-buster tag also pulls an
# image built upon the Debian 10 branch, which is even slightly
# smaller than its bullseye successor

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
    extra-runtime-dependencies \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add ${TARGET_ENV} \
    && USER=root cargo new --bin main
RUN cargo build --release

FROM alpine:latest
WORKDIR /app

COPY --from=build /app/target/release/actix_server .
CMD ["actix_server"]