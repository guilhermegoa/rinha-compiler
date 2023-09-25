FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

# CMD ["./target/release/rinha-interpreter", "./examples/print.json"]
# CMD ["./target/release/rinha-interpreter", "./examples/sum.json"]
# CMD ["./target/release/rinha-interpreter", "./examples/tuple.json"]
# CMD ["./target/release/rinha-interpreter", "./examples/combination.json"]
CMD ["./target/release/rinha-interpreter", "./examples/fib.json"]