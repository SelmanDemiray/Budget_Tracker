FROM rust:1.83 AS builder

WORKDIR /app

# Set SQLx to offline mode to avoid needing database during build
ENV SQLX_OFFLINE=true

# Copy Cargo.toml and sqlx-data.json first
COPY Cargo.toml ./
COPY sqlx-data.json ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/budget_tracker*

# Copy the actual source code
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/budget_tracker .
COPY --from=builder /app/migrations ./migrations
COPY static ./static

# Create non-root user for security
RUN useradd -r -s /bin/false appuser && \
    chown -R appuser:appuser /app
USER appuser

EXPOSE 3000

CMD ["./budget_tracker"]
