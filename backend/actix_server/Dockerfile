FROM rust:1.68.1 as builder

RUN USER=root

RUN mkdir actix_server
WORKDIR /actix_server
ADD . ./
RUN cargo clean && \
    cargo build -vv --release

FROM debian:buster-slim

ARG APP=/usr/src/app

ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# Copy the compiled binaries into the new container.
COPY --from=builder /actix_server/target/release/actix_server ${APP}/actix_server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./trampoline"]