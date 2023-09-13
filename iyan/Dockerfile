FROM rust:latest AS build

# Set the working directory in the container
WORKDIR /usr/src/iyan

RUN apt-get update && apt install -y openssl

RUN apt install -y postgresql

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

COPY config ./config
COPY core ./core
COPY iyan ./iyan
COPY server ./server
COPY types ./types
COPY rabbitmq ./rabbitmq
COPY logger ./logger

RUN cargo build --release

RUN cargo install --path .

# Use a smaller image for the final runtime
FROM debian:bookworm

# Set the working directory in the container
WORKDIR /usr/src/iyan

COPY --from=build /usr/local/cargo/bin/iyan .

CMD ["./iyan"]