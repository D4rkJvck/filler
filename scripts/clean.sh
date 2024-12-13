#!/bin/bash

source "$(dirname $0)/utils.sh"

# Remove filler.zip
if [ -e "filler.zip" ]; then
    log "Removing filler.zip"
    rm filler.zip
    log "filler.zip [REMOVED]\n"
else
    log "filler.zip already removed"
fi

# Remove docker_image/
if [ -e "docker_image" ]; then
    log "Removing docker_image"
    rm -rf docker_image/
    log "docker_image/ [REMOVED]\n"
else
    log "docker_image/ already removed"
fi

# Clean build artifacts
log "Cleaning build artifacts"
cargo clean -v --target-dir solution
cargo clean -v --target target
