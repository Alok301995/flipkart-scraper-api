# FROM rust:1.75-slim as builder

# WORKDIR /app
# COPY . .

# RUN cargo build --release

# FROM debian:bullseye-slim

# WORKDIR /app

# # Install SSL certificates for HTTPS requests
# RUN apt-get update && \
#     apt-get install -y ca-certificates && \
#     rm -rf /var/lib/apt/lists/*

# COPY --from=builder /app/target/release/flipkart-scraper-api .

# # Expose the port your application will run on
# EXPOSE 10000

# # Command to run the application
# CMD ["./flipkart-scraper-api"]