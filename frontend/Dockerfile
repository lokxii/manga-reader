FROM rust:1.75-slim-bookworm

RUN apt-get --yes update && apt-get --yes install curl
RUN rustup target add wasm32-unknown-unknown
COPY install-from-binstall-release.sh .
RUN bash install-from-binstall-release.sh
RUN cargo binstall wasm-bindgen-cli@0.2.90 --no-confirm
RUN cargo binstall trunk@0.18.1 --no-confirm
