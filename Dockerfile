FROM rust:1.90-bookworm as build

WORKDIR /agent
ARG CARGO_FLAGS="--release"

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# To improve performance and prevent the entire registry from being downloaded
# see https://blog.rust-lang.org/inside-rust/2023/01/30/cargo-sparse-protocol.html
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo build ${CARGO_FLAGS}

FROM debian:bookworm-slim as agent

# Update package lists and install security updates
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /agent

COPY --from=build /agent/target/release/cedar-agent /agent/cedar-agent

ENV ADDR=0.0.0.0

ENTRYPOINT ["/agent/cedar-agent"]

