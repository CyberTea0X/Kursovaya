FROM debian

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install git curl g++ build-essential

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

WORKDIR /usr/src/app

RUN git clone https://github.com/unegare/actix_server.git

RUN ["/bin/bash", "-c", "source $HOME/.cargo/env; cd ./actix_server/; cargo build --release; mkdir uploaded"]

EXPOSE 8080

ENTRYPOINT ["/bin/bash", "-c", "echo 'Hello there!'; source $HOME/.cargo/env; cd ./actix_server/; cargo run --release"]