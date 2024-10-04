FROM rust:1.81.0 AS builder

RUN apt-get update && apt-get install -y musl-tools

WORKDIR /usr/src/app
COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target=x86_64-unknown-linux-musl --release  --verbose

FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/sample_api /usr/local/bin/sample_api

CMD ["sample_api"]

EXPOSE 3000
