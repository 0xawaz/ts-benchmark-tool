#### Stage 1: Build stage
FROM --platform=linux/amd64 rust:1.79.0-slim AS build

# Install musl target
RUN rustup target add x86_64-unknown-linux-musl

# Create app directory
WORKDIR /app

# Copy source code
COPY src/ ./src/

# Install app dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --target x86_64-unknown-linux-musl --release

# Compile app
RUN cargo build --target x86_64-unknown-linux-musl --release

#### Stage 2: Runtime stage
FROM alpine:3.16.9 AS release

# Install required libraries
RUN apk add --no-cache libgcc

# Create app directory
WORKDIR /app

# Copy binary from previous stage
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/tsbtool /app

# Define app Entrypoint
ENTRYPOINT [ "/app/tsbtool" ]