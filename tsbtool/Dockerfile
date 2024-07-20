#### Stage 1: Build stage
FROM --platform=linux/amd64 rust:1.79.0-slim AS build

# Create app directory
WORKDIR /app

# Copy app manifests
COPY . .

# Install app dependencies
RUN cargo update

# Compile app
RUN cargo build

#### Stage 2: Runtime stage
FROM alpine:3.16.9 AS release

# Copy binary from previous stage
COPY --from=build /app/target/debug/tsbtool_bin /

# Define the command to run the app
CMD [ "/tsbtool_bin" ]
