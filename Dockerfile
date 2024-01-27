FROM rust:buster

RUN mkdir -p /src/app

WORKDIR /src/app

COPY Cargo.toml Cargo.lock .

COPY src src

RUN cargo build -r
