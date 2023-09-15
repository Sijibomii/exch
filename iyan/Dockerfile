FROM rust:latest AS build

# Set the working directory in the container
WORKDIR /usr/src/iyan

RUN apt-get update && apt install -y openssl

RUN apt install -y postgresql

COPY . .

RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install --path .

EXPOSE 4001

CMD ["iyan"]

# Use a smaller image for the final runtime
# FROM debian:bookworm

# RUN apt-get update && apt install -y openssl

# RUN apt install -y postgresql


# # Set the working directory in the container
# WORKDIR /usr/src/iyan

# COPY --from=build /usr/local/cargo/bin/iyan .

# CMD ["./iyan"]