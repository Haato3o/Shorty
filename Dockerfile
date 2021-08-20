FROM rust:latest as builder

WORKDIR /app

# Install required system packages for building.
RUN apt-get update \
    && apt-get install musl-tools -y \
    && rustup target add x86_64-unknown-linux-musl

COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api .

ENTRYPOINT [ "./api" ]