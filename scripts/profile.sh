#!/usr/bin/env bash

# check if samply is installed
if ! command -v samply &> /dev/null
then
    echo "samply could not be found, please install it from https://github.com/mstange/samply"
    return
fi

# variables
SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_PATH")"

BINARY_PATH="$PROJECT_ROOT/target/release-profiling/luma"

PROFILE_NAME="profile_$(date +%s)"
PROFILE_DIR="$PROJECT_ROOT/scripts/profiles/$PROFILE_NAME"

# make profile directory
mkdir -p $PROFILE_DIR 2> /dev/null || true

# should build?
if [ "$1" == "--build" ]; then
    shift # remove the first argument

    echo "Building the project in release-profiling profile..."
    cargo build --profile release-profiling

    cp "$BINARY_PATH" "$PROFILE_DIR/"
fi

# run samply with the binary
samply record \
    --rate 100000 \
    --output "$PROFILE_DIR/profile.json.gz" \
    "$BINARY_PATH" "$@"

