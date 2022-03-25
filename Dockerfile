FROM rust:1.59 as builder
WORKDIR /app
COPY . /app
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/echo-hyper /usr/local/bin
ENTRYPOINT ["echo-hyper"]