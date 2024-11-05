FROM rust:1.82.0-bullseye AS build

RUN apt-get update
RUN apt-get install -y protobuf-compiler

# we assume we're in the root directory
ADD . /app/

RUN cd /app/fhevm-engine/coprocessor && SQLX_OFFLINE=true cargo build --release

FROM debian:bullseye

COPY --from=build /app/fhevm-engine/target/release/coprocessor /usr/local/bin

ENTRYPOINT ["/usr/local/bin/coprocessor"]