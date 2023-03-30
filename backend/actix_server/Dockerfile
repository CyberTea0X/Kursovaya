FROM rust:latest

WORKDIR /usr/src/actix_server
COPY Cargo.toml src ./
RUN cargo init --bin
RUN cargo install --path .

CMD ["actix_server"]