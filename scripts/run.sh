#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status
source "$(dirname $0)/utils.sh"

# Download the docker image
if [ ! -f "filler.zip" ]; then
    log "Downloading filler.zip..."
    wget https://assets.01-edu.org/filler/filler.zip || error "Failed to download file."
fi

# Unzip the docker image
if [ ! -d "docker_image" ]; then
    log "Unzipping file..."
    unzip filler.zip || error "Failed to unzip file."
fi

# Build the docker image
if [ ! "$(docker images -q filler)" ]; then
    log "Building image..."
    docker build -t filler ./docker_image/ || error "Failed to build image."
fi

# Building the filler project
log "\nBuilding binary..."
cargo build --target-dir solution || error "Failed to build binary."

if [ "$(uname -s)" == "Darwin" ] && [ "$(uname -m)" != "arm64" ]; then
    log "Copying src/ into solution/"
    cp src/ ./solution
else
    # Run a docker container
    log "Running Container..."
    docker run --rm -v "$(pwd)/solution":/filler/solution -it filler || error "Failed to run container."
fi

# Cleaning up
read -n 1 -p "Would you like to clean? [Y/n]: " response

if [[ "${response,,}" == "y" ]]; then
    log "Cleaning..."
    source "$(dirname $0)/clean.sh"
fi

log "Exiting..."
