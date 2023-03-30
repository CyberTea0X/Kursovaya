FROM rust:latest

WORKDIR /usr/src/actix_server
COPY Cargo.toml ./

RUN cargo install --path .

CMD ["actix_server"]