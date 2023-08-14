FROM rust:latest
WORKDIR /
COPY . .
RUN cargo run --release