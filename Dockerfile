FROM rust:latest

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src
COPY templates ./templates

ENV DB_NAME=db.sqlite
ENV ADDRESS=0.0.0.0
ENV PORT=3002


VOLUME ["/app/db"]
RUN mkdir /app/db
RUN cargo build --release


CMD ["./target/release/Rust-Test-Task-1"]
