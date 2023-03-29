FROM rust:latest AS build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /app
COPY . .
RUN cargo build --release && \
    strip target/release/actix_server && \
    rm -rf src/ tests/ target/release/.fingerprint/actix_server-*

FROM gcr.io/distroless/cc-debian10
WORKDIR /app

COPY --from=build /app/target/release/actix_server .
CMD ["./actix_server"]