# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.83.0
ARG APP_NAME=filler

################################################################################
# Create a stage for building the application.

FROM docker:20.10.7-dind

# Install Rust and Cargo from the official Rust installation script
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ARG APP_NAME
WORKDIR /${APP_NAME}

COPY . .

# What the container should run when it is started.
ENTRYPOINT [ "bin/bash" ]
