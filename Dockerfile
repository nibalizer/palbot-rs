# build
FROM rust:bookworm as builder

RUN mkdir -p /src/app

WORKDIR /src/app

COPY Cargo.toml Cargo.lock .

COPY src src

RUN cargo build --release

# package
FROM debian:bookworm

RUN apt-get update && apt-get install -y libssl3

COPY --from=builder /src/app/target/release/palbot /bin/palbot

CMD ["/bin/palbot"]
