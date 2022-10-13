# Use the Rust official image from Docker
FROM rust:1.64

# Copy the files to the Docker image
WORKDIR /app
COPY . .

# Build program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/resfetch"]