#!/bin/bash

# Prints its first argument...
# Adds a 1 second cooldown...
log() {
    for arg in "$@"
    do
        echo -e "$arg"
        sleep .5
    done
}
