# Use an official Ubuntu as the base image
FROM ubuntu:20.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Update and install required packages 
RUN apt-get update && apt-get install -y \
    g++ \
    cmake \
    make \
    libpthread-stubs0-dev \
    git \ 
    openssl \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update && \
    apt-get install -y ninja-build && \
    rm -rf /var/lib/apt/lists/*
    
# Create a directory for the app
WORKDIR /app

# Copy your CMake project files into the container
COPY . .

RUN cd /app

# # cd amqpcpp and build amqpcpp
RUN cd amqpcpp \
    && mkdir build && cd build \
    && cmake .. && cmake --build . --target install  && make && make install

RUN cd /app 

WORKDIR /app/build

RUN cmake -DCMAKE_BUILD_TYPE=Release .. && \
    cmake --build . --parallel 8

ENTRYPOINT [ "/app/build/exchange_main" ]

# FROM alpine:3.17.0 AS build

# RUN apk update && \
#     apk add --no-cache \
#     build-base \
#     cmake \
#     git \
#     openssl \
#     openssl-dev \
#     boost1.80-dev=1.80.0-r3

# WORKDIR /turkey

# COPY . .

# RUN cd common/rabbitmq-c-0.13.0 \
#     && mkdir build && cd build \
#     && cmake .. && cmake --build . --target install  && make && make install

# WORKDIR /turkey

# RUN cd amqpcpp \
#     && mkdir build && cd build \
#     && cmake .. && cmake --build . --target install  && make && make install

# WORKDIR /turkey/build

# RUN cmake -DCMAKE_BUILD_TYPE=Release .. && \
#     cmake --build . --parallel 8

# ########################################################################################################################
# # turkey
# ########################################################################################################################

# FROM alpine:3.17.0

# RUN apk update && \
#     apk add --no-cache \
#     libstdc++ \
#     boost1.80-program_options=1.80.0-r3

# RUN addgroup -S turkey && adduser -S turkey -G turkey
# USER turkey

# COPY --chown=turkey:turkey --from=build \
#     /turkey/build \
#     /turkey

# ENTRYPOINT [ "./turkey/exchange_main" ]