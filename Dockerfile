FROM ekidd/rust-musl-builder:latest AS builder

COPY src src
RUN sudo chown -R rust:rust /home/rust/src/
COPY Cargo.lock .
RUN sudo chown rust:rust /home/rust/src/Cargo.lock
COPY Cargo.toml .
RUN sudo chown rust:rust /home/rust/src/Cargo.toml
COPY Rocket.toml .
RUN sudo chown rust:rust /home/rust/src/Rocket.toml
COPY rust-toolchain .
RUN sudo chown rust:rust /home/rust/src/Rocket.toml

RUN /bin/bash -c "rustup target add x86_64-unknown-linux-musl"
# Build
RUN /bin/bash -c "cargo build --release --target x86_64-unknown-linux-musl"
# Test
RUN /bin/bash -c "cargo test --release"


FROM alpine:3.12.0 AS app
# Set working directory
WORKDIR /app
# Copy in statically linked binary from builder stage
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/github-languages-percentage-webapi /usr/local/bin
# Expose port for server
# Run entrypoint script
CMD ROCKET_PORT=$PORT github-languages-percentage-webapi
