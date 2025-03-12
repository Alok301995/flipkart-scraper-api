# First stage: Build the Rust binary
FROM docker.io/rust:1.73-slim-bullseye as builder

WORKDIR /usr/src/flipkart-scraper-api

# Install required dependencies
RUN apt update && apt install -y libssl-dev pkg-config

# Set necessary environment variables
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include

# Copy Cargo manifest and fetch dependencies separately for caching
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

# Copy the rest of the source code
COPY ./src ./src

# Build the project in release mode
RUN cargo build --release

# Second stage: Create a smaller production image
FROM docker.io/debian:bullseye-slim

WORKDIR /usr/local/bin/

# Install required runtime dependencies
RUN apt update && apt install -y ca-certificates

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/flipkart-scraper-api/target/release/flipkart-scraper-api .

# Expose the port (not strictly necessary for dynamic ports)
EXPOSE 3000

# Set Render’s dynamic port (default to 3000 if not set)
ENV PORT=3000

# Run the application with the dynamic port
CMD ["sh", "-c", "./flipkart-scraper-api --port ${PORT}"]
