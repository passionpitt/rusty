# Rusty ^(-_-)^

FROM rust:1.80 AS builder

WORKDIR /usr/src/rusty

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --bin rusty || true

COPY src ./src

RUN cargo build --release --bin rusty

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update && \
    apt-get install -y wget && \
    wget http://security.debian.org/debian-security/pool/updates/main/o/openssl/libssl1.1_1.1.1n-0+deb11u5_amd64.deb && \
    dpkg -i libssl1.1_1.1.1n-0+deb11u5_amd64.deb && \
    rm libssl1.1_1.1.1n-0+deb11u5_amd64.deb && \
    apt-get install -f -y

RUN ls -l /usr/lib/x86_64-linux-gnu/ | grep libssl

COPY --from=builder /usr/src/rusty/target/release/rusty /usr/local/bin/rusty

RUN ldconfig

ENTRYPOINT ["rusty"]
