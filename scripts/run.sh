#!/bin/bash

source "$(dirname "$0")/log.sh"

# Download the docker image
log "Downloading docker image folder zip...\n"
wget https://assets.01-edu.org/filler/filler.zip
log "Docker image folder zip downloaded\n"

# Unzip the docker image
log "Unzipping docker image folder...\n"
unzip filler.zip
rm filler.zip
log "\nDocker image folder unzipped" "Zip folder deleted successfully."

# Building the filler project
log "\nBuilding filler binary..."
cargo build --target-dir solution
log "\n Binary built"

# Build the docker image
log "\nBuilding the docker image...\n"
docker build -t filler ./docker_image/
log "\nDocker image built"

# Run a docker container
log "\nRunning a docker container..."
docker run --rm -v "$(pwd)/solution":/filler/solution -it filler
log "Container exited"

# Cleaning up
log "\nRemoving docker image..."
docker rmi filler
log "Deleting folder..."
rm -rf docker_image/
log "Folder deleted successfully." "Deleting solution folder..."
rm -rf solution/
cargo clean
log "Solution folder deleted successfully." "Exiting..."