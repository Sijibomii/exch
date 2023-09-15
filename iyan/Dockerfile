FROM rust:latest AS build

# Set the working directory in the container
WORKDIR /usr/src/iyan

RUN apt-get update && apt install -y openssl

RUN apt install -y postgresql

# Copy the Cargo.toml and Cargo.lock files to the container
# COPY Cargo.toml Cargo.lock ./

# COPY config ./config
# COPY core ./core
# COPY iyan ./iyan
# COPY server ./server
# COPY types ./types
# COPY rabbitmq ./rabbitmq
# COPY logger ./logger

COPY . .

RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install --path .

EXPOSE 4001

CMD ["diesel migration run  && iyan"]

# Use a smaller image for the final runtime
# FROM debian:bookworm

# RUN apt-get update && apt install -y openssl

# RUN apt install -y postgresql


# # Set the working directory in the container
# WORKDIR /usr/src/iyan

# COPY --from=build /usr/local/cargo/bin/iyan .

# CMD ["./iyan"]