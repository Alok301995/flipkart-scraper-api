# First stage: Build the Rust binary
FROM docker.io/rust:1.73-slim-bullseye as builder

WORKDIR /usr/src/flipkart-scraper-api

# Install required dependencies
RUN apt update && apt install -y libssl-dev pkg-config

# Copy Cargo manifest first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Ensure src exists before fetching dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Fetch dependencies only (without compiling the project yet)
RUN cargo fetch --locked

# Now copy the actual source code
COPY ./src ./src

# Build the Rust project in release mode
RUN cargo build --release

# Second stage: Create a smaller production image
FROM docker.io/debian:bullseye-slim

WORKDIR /usr/local/bin/

# Install required runtime dependencies
RUN apt update && apt install -y ca-certificates

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/flipkart-scraper-api/target/release/flipkart-scraper-api .

# Expose the dynamic port (Render assigns it)
EXPOSE 3000

# Set dynamic port for Render
ENV PORT=3000

# Run the application
CMD ["sh", "-c", "./flipkart-scraper-api --port ${PORT}"]
