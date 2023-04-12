FROM rust:latest

WORKDIR /usr/src/actix_server
COPY . .
RUN cargo install --path .
RUN cargo clean

CMD ["actix_server"]