FROM ubuntu:latest
WORKDIR /workspace
COPY connectors/*.yaml connectors/
COPY dataflow.yaml .
COPY run.sh .
ENV PATH "$PATH:/root/.fluvio/bin"
ENV PATH "$PATH:/root/.fvm/bin"

ARG FLUVIO_VERSION=

RUN apt-get update && \
    apt-get install -y \
    curl unzip cargo 
RUN apt-get install -y rustc
RUN apt-get install -y rustup

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    # Add Rust to the PATH for the current session
    export PATH="$HOME/.cargo/bin:$PATH" && \
    # Install a specific version of Rust
    rustup install 1.80.0 && \
    rustup default 1.80.0
RUN rustup target add wasm32-wasip2
    
RUN curl -fsS "https://hub.infinyon.cloud/install/install.sh?ctx=sdf-demo" | FLUVIO_VERSION=${FLUVIO_VERSION} bash &&
    echo 'source "${HOME}/.fvm/env"' >> ~/.bashrc
RUN cd /workspace/connectors/ && \
    cdk hub download infinyon/http-source@0.4.3

CMD ["bash","run.sh"]

    



