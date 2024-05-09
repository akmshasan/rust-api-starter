FROM rust:latest

WORKDIR /app/

COPY src /app/
COPY migrations /app/
COPY .env /app/
COPY Cargo.lock /app/
COPY Cargo.toml /app/
COPY diesel.toml /app/

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

CMD ["cargo", "watch", "--why", "-x", "build"]