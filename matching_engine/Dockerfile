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
    && rm -rf /var/lib/apt/lists/*

# Create a directory for the app
WORKDIR /app

# Copy your CMake project files into the container
COPY . /app

# Build the project
RUN cmake .
RUN make

# Command to run your application
CMD ["./exchange_main"]
