#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status
source "$(dirname $0)/utils.sh"

if [ ! "$(docker images -q filler)" ]; then
    # Download the docker image
    log "Downloading file..."
    wget https://assets.01-edu.org/filler/filler.zip || error "Failed to download file."

    # Unzip the docker image
    log "Unzipping file..."
    unzip filler.zip || error "Failed to unzip file."

    # Build the docker image
    log "Building image..."
    docker build -t filler ./docker_image/ || error "Failed to build image."
fi

# Building the filler project
# log "Building binary..."
# cargo build --target-dir solution || error "Failed to build binary."

# Run a docker container
log "Running Container..."
docker run --rm -v "$(pwd)/solution":/filler/solution -it filler || error "Failed to run container."

# # Cleaning up
# log "Cleaning..."
# source "$(dirname $0)/clean.sh"
# log "Exiting..."
