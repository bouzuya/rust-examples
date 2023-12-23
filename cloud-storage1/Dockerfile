FROM rust:1.74.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN apt-get update && apt-get install -y \
  jq \
  mold \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*
RUN useradd --create-home bouzuya
USER bouzuya
RUN rustup component add \
  clippy \
  rustfmt \
  && cargo install \
  cargo-watch \
  && mkdir -p /home/bouzuya/kireta
WORKDIR /home/bouzuya/cloud-storage
