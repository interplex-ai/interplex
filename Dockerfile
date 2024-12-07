# Use an official Rust image for building
FROM rust:alpine as builder
ARG BUF_TOKEN
# Install necessary dependencies for building Rust projects
RUN apk add --no-cache musl-dev build-base openssl-dev ca-certificates git curl

# Set the working directory
WORKDIR /app

# Copy the entire project into the container
COPY . .

# Log in to the buf registry (you can pass credentials at build time)
#
RUN cargo login --registry buf "Bearer $BUF_TOKEN"
#
## Build the project in release mode
RUN cargo build --release
#
# Use a lightweight image for running the application
FROM alpine:latest

# Install necessary runtime dependencies
RUN apk add --no-cache ca-certificates

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/interplex /app/interplex

# Set the entrypoint to the compiled binary
CMD ["/app/interplex"]
