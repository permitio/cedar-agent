FROM rust:1.69-bullseye as build

WORKDIR /agent
ARG CARGO_FLAGS="--release"

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build ${CARGO_FLAGS}

FROM debian:bullseye-slim as agent

WORKDIR /agent

COPY --from=build /agent/target/release/cedar-agent /agent/cedar-agent

ENV ADDR=0.0.0.0

ENTRYPOINT ["/agent/cedar-agent"]

