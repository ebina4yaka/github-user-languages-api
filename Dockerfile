FROM ekidd/rust-musl-builder:latest AS builder

COPY src src
RUN sudo chown -R rust:rust /home/rust/src/
COPY Cargo.lock .
RUN sudo chown rust:rust /home/rust/src/Cargo.lock
COPY Cargo.toml .
RUN sudo chown rust:rust /home/rust/src/Cargo.toml
COPY Rocket.toml .
RUN sudo chown rust:rust /home/rust/src/Rocket.toml

RUN /bin/bash -c "rustup toolchain install nightly"
RUN /bin/bash -c "rustup target add x86_64-unknown-linux-musl --toolchain nightly"
# Build
RUN /bin/bash -c "cargo +nightly build --release --target x86_64-unknown-linux-musl"
# Test
RUN /bin/bash -c "cargo +nightly test --release"


FROM alpine:3.12.0 AS app
# Set working directory
WORKDIR /app
# Copy in statically linked binary from builder stage
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/github-languages-percentage-webapi /usr/local/bin
# Expose port for server
EXPOSE 8000
# Run entrypoint script
CMD github-languages-percentage-webapi
