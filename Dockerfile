FROM rust:latest as rust-builder
WORKDIR /build
COPY . /build
RUN cargo build --release -p rust-victoria-benchmark --target-dir ./target

FROM debian:stable-slim
COPY --from=rust-builder /build/target/release/rust-victoria-benchmark /usr/local/bin/
RUN apt install openssl
CMD ["rust-victoria-benchmark"]