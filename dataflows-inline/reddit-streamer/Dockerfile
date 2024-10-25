FROM rust:latest
RUN mkdir -p fluvio/run
COPY Makefile fluvio/run
COPY generator fluvio/run
COPY dataflow* fluvio/run
COPY cleancon fluvio/run
COPY dockerscript.sh fluvio/run
WORKDIR fluvio/run


RUN apt-get update
RUN apt-get install -y curl unzip make
#need to install cargo for fvm
RUN curl -fsS https://hub.infinyon.cloud/install/install.sh?ctx=dc | bash

ENV PATH "$PATH:/root/.fluvio/bin"
ENV PATH "$PATH:/root/.fvm/bin"

#curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#source $HOME/.cargo/env
#rustup update
#rustup target add wasm32-wasip1

RUN fvm install sdf-beta2 && \
    rustup target add wasm32-wasip1
#cargo is 500mb
EXPOSE 8000

RUN chmod +x dockerscript.sh
CMD ["./dockerscript.sh"]
