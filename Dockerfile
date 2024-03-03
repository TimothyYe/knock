# Use the Rust official image for the build stage
FROM rust:latest as builder

# Add target for musl
RUN rustup target add x86_64-unknown-linux-musl

COPY ./knockd /knockd

WORKDIR /knockd

# Build your application on the musl target.
# This creates a statically linked executable.
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine
RUN apk add --no-cache iptables

# Copy the binary from the builder stage
COPY --from=builder /knockd/target/x86_64-unknown-linux-musl/release/knockd /

# Command to run
CMD ["/knockd"]