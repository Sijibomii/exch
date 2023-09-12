FROM rust:latest AS build

# Set the working directory in the container
WORKDIR /usr/src/iyan

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./


# Copy the rest of the application code to the container
COPY config ./config
COPY core ./core
COPY iyan ./iyan
COPY server ./server
COPY types ./types
COPY rabbitmq ./rabbitmq
COPY logger ./logger

# Build and cache dependencies
RUN cargo build --release

# Build the application
RUN cargo install --path .

# Use a smaller image for the final runtime
FROM debian:buster-slim

# Set the working directory in the container
WORKDIR /usr/src/iyan

# Copy the binary from the build stage to the runtime container
COPY --from=build /usr/local/cargo/bin/iyan .

# Run the application when the container starts
CMD ["./iyan"]