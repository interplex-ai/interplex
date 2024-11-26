# Dockerfile
# Use an official lightweight image as a parent image
FROM alpine:latest

WORKDIR /app
# Install necessary runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the pre-built binary from your local filesystem to the container
COPY ./target/x86_64-unknown-linux-musl/release/interplex /app/interplex

# Set the entrypoint to the compiled binary
CMD ["/app/interplex"]
