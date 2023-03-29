# FROM alpine:latest
# WORKDIR /app

# COPY --from=build /usr/local/cargo/bin/api-service /usr/local/bin/api-service
# CMD ["actix_server"]


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
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates

COPY --from=build /app/target/release/actix_server .
CMD ["actix_server"]