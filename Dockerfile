FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

ENV JSON_PATH=""

CMD ["./target/release/rinha-interpreter", "$JSON_PATH"]