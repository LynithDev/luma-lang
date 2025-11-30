#!/usr/bin/env bash

# check if hyperfine is installed
if ! command -v hyperfine &> /dev/null
then
    echo "hyperfine could not be found, please install it from https://github.com/sharkdp/hyperfine"
    return
fi

# variables
SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_PATH")"
BENCHMARKS_DIR="$PROJECT_ROOT/scripts/benchmarks"

LATEST_BINARY_PATH="$PROJECT_ROOT/target/release-speed/luma"
CURR_BENCHMARK_NAME="benchmark_$(date +%s)"
CURR_BENCHMARK_DIR="$BENCHMARKS_DIR/$CURR_BENCHMARK_NAME"

# collect all previous benchmark binaries
PREV_BENCHMARK_BINARIES=($(ls -td $BENCHMARKS_DIR/benchmark_*/luma 2> /dev/null))

# make benchmark directory
mkdir -p $CURR_BENCHMARK_DIR 2> /dev/null || true

# should skip build?
if [ "$1" == "--build" ]; then
    shift # remove the first argument

    echo "Building the project in release-speed profile..."
    cargo build --profile release-speed

    cp "$LATEST_BINARY_PATH" "$CURR_BENCHMARK_DIR/"
fi

# prepare command list
ARGS=("$@")

CMD_LIST=("$LATEST_BINARY_PATH ${ARGS[@]}")  # current binary with args
for bin in "${PREV_BENCHMARK_BINARIES[@]}"; do
    CMD_LIST+=("$bin ${ARGS[@]}")
done

# run hyperfine with the binary
hyperfine --warmup 3 \
    --min-runs 10 \
    --export-json "$CURR_BENCHMARK_DIR/benchmark.json" \
    "${CMD_LIST[@]}"

