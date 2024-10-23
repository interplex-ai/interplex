# Dockerfile
# Use an official lightweight image as a parent image
FROM alpine:latest

# Install necessary runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the pre-built binary from your local filesystem to the container
COPY ./target/release/interplex /usr/local/bin/interplex

# Set the entrypoint to the compiled binary
ENTRYPOINT ["/usr/local/bin/interplex"]
