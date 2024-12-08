#!/bin/bash

source "$(dirname "$0")/utils.sh"

BRANCH="main"
USER="jefaye"
MAIL="clement.jean.l.faye@gmail.com"

# Checks if there is already a USER and MAIL in configuration...
# If no USER or MAIL if found, it will config it...
log "\nChecking credentials..."
if ! git config --get user.name || ! git config --get user.MAIL; then
    log "No Credentials found!!!" "Configuring credentials..."
    git config user.name "$USER"
    git config user.MAIL "$MAIL"
fi
log "Credentials have been configured successfully!"

# Checks if we are in the desired branch before adding files...
# Exits the program is we're in a different branch to prevent confusion...
log "\nChecking branch..."
git branch
if [ "$(git rev-parse --abbrev-ref HEAD)" != "$BRANCH" ]; then
    log "[WARNING] Not on branch $BRANCH..." "Please restart..."
    exit 1
fi
log "You're on branch $BRANCH!"

# Adds the files given as arguments...
# If there is no arguments, it adds all files...
log "\nAdding files..."
if [ $# -eq 0 ]; then
    log "No Files specified..." "Adding all changes..."
    git add .
else
    log "Adding Specified files..."
    for file in "$@"; do
        git add "$file"
        log "Added: $file"
    done
fi
log "Files added successfully!"

# Prompts the user for a commit message...
log "\nTime to push changes..."
git status
read -r -p "Enter Commit Message: " commit_message
git commit -a -m "$commit_message"

# Pushes the changes...
git push origin "$BRANCH"
git push mirror "$BRANCH"
log "Well done!"
