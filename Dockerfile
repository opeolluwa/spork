FROM rust:1.62.0 as builder
RUN cargo new --bin spork
WORKDIR /app

# FROM rust:1 as builder
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml
# RUN cargo build --release

# RUN rm src/*.rs
COPY . .
RUN cargo install --path .
# RUN cargo build --release



RUN rm ./target/release/deps/spork*
# RUN cargo build --release

FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/spork /app/spork
COPY --from=builder /app/views /app/views
COPY --from=builder /app/*.toml /app/

EXPOSE 8080
ENTRYPOINT ["/app/spork"]