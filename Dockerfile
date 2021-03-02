ARG BASE_IMAGE=ekidd/rust-musl-builder:1.48.0

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN rustup set profile minimal
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.13.2 AS app
# Set working directory
WORKDIR /app
# Copy in statically linked binary from builder stage
COPY --from=builder \
     /home/rust/src/target/x86_64-unknown-linux-musl/release/github-languages-percentage-webapi \
     /usr/local/bin

CMD ROCKET_PORT=$PORT github-languages-percentage-webapi
