# Use the official Rust image as the base
FROM rust:1.69 as builder

# Set the working directory
WORKDIR /app

# Copy the necessary files
COPY Cargo.toml .
COPY src src

# Build the application
RUN cargo build --release

# Create the final image
FROM debian:buster-slim

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl1.1 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api /app/api

# Expose the port
EXPOSE 8000

# Set the environment variables for Rocket
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Start the application
CMD ["/app/api"]
