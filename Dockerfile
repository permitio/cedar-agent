FROM rust:1.77-bullseye as build

WORKDIR /agent
ARG CARGO_FLAGS="--release"

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# To improve performance and prevent the entire registry from being downloaded
# see https://blog.rust-lang.org/inside-rust/2023/01/30/cargo-sparse-protocol.html
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo build ${CARGO_FLAGS}

FROM debian:bullseye-slim as agent

WORKDIR /agent

COPY --from=build /agent/target/release/cedar-agent /agent/cedar-agent

ENV ADDR=0.0.0.0

ENTRYPOINT ["/agent/cedar-agent"]

