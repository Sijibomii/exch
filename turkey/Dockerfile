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
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update && \
    apt-get install -y ninja-build && \
    rm -rf /var/lib/apt/lists/*
# Create a directory for the app
WORKDIR /app

# Copy your CMake project files into the container
COPY . /app

# # Clone and build amqpcpp
RUN git clone https://github.com/CopernicaMarketingSoftware/AMQP-CPP.git \
    && cd AMQP-CPP \
    && mkdir build && cd build \
    && cmake .. && cmake --build . --target install && make && make install

RUN cd /app

COPY scripts/build.sh /usr/local/bin/

RUN chmod +x /usr/local/bin/build.sh
RUN /usr/local/bin/build.sh

CMD ["./scripts/run_exchange.sh"]

