FROM rust:1-slim-bookworm

ARG FLUVIO_VERSION=

RUN rustup target add wasm32-wasip2
RUN apt-get update
RUN apt-get install -y curl unzip
RUN apt-get install -y tini openssl curl
RUN curl -fsS "https://hub.infinyon.cloud/install/install.sh?ctx=sdf-demo" | FLUVIO_VERSION=${FLUVIO_VERSION} bash

ENV PATH "$PATH:/root/.fluvio/bin"
ENV PATH "$PATH:/root/.fvm/bin"

RUN sdf setup || true
