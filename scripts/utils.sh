#!/bin/bash

# Prints its first argument...
# Adds a 1 second cooldown...
log() {
    for arg in "$@"; do
        echo -e "$arg"
        sleep .5
    done
}

# Handle errors by logging them fisrt...
# then exit the script with a non-zero exit code
error() {
    log "ERROR: $1" "Exiting..."
    exit 1
}

# Removes files or directories
# through their name given as argument.
# if they exist or sends a message
# let us know that it has already been removed.
remove() {
    for arg in "$@"; do
        if [ -d "$arg" ]; then
            log "Removing directory $arg..."
            rm -rf "$arg" || continue
            log "$arg [REMOVED]\n"
        elif [ -f "$arg" ]; then
            log "Removing file $arg..."
            rm "$arg" || continue
            log "$arg [REMOVED]\n"
        else
            log "$arg already removed...\n"
        fi
    done
}

# Adds files, then commits with a autogenerated message, then pushes...
# All in a defined time interval...
# Not used, needs further investigations...
# Requires commit_and_push() to be desactivated...
auto_push() {
    i=1
    while true; do
        if [[ -n $(git status -s) ]]; then
            add_files "$@"
            log "Status:"
            git status
            git commit -a -m "Auto Commit - $i"
            git push origin master
            i=$((i + 1))
        fi
        sleep 1200
    done
}
