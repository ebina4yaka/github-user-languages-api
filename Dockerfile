FROM ekidd/rust-musl-builder:latest AS builder

# Add our source code.
ADD --chown=rust:rust . ./
# Build our application.
RUN sudo cargo build --release

FROM alpine:3.12.0 AS app
# Set working directory
WORKDIR /app
# Copy in statically linked binary from builder stage
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/github-languages-percentage-webapi /usr/local/bin
# Expose port for server
# Run entrypoint script
CMD ROCKET_PORT=$PORT github-languages-percentage-webapi
