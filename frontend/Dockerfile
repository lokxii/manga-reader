FROM rust:1.75-slim-buster

RUN apt-get --yes update && apt-get --yes install git pkg-config libssl-dev
RUN cargo install trunk --version 0.18.0
RUN rustup target add wasm32-unknown-unknown
