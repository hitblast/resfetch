# builder stage
FROM rust:1.76 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# executable stage
FROM scratch
USER 1000
COPY --from=builder ./target/release/resfetch .
CMD ["./resfetch"]