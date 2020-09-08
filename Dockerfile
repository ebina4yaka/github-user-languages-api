FROM rustlang/rust:nightly-alpine

RUN mkdir /app
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
COPY ./src /app/src
WORKDIR /app
RUN apk add --no-cache g++ libressl-dev
RUN cargo build --release
RUN cargo test --release

CMD cargo run --release
