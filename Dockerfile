# build
FROM rust:buster as builder

RUN mkdir -p /src/app

WORKDIR /src/app

COPY Cargo.toml Cargo.lock .

COPY src src

RUN cargo build --release

# package
FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder /src/app/target/release/palbot /usr/local/bin/palbot

ENTRYPOINT ["/usr/local/bin/palbot"]
