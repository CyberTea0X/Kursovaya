FROM rust:latest

WORKDIR /usr/src/actix_server
COPY . .

RUN cargo install --path .

CMD ["actix_server"]