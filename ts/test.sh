#!/usr/bin/env bash
set -e  # Exit on any error
# Initialize variables
ENVIRONMENT=""

# Parse command-line options
while getopts e: option
do
    case "${option}" in
        e) ENVIRONMENT=${OPTARG};;
    esac
done

# Check required options
if [ -z "$ENVIRONMENT" ]; then
    echo "Usage: $0 -e [node|browser]"
    exit 1
fi

# Define test environment options
NODE_ENV="--environment node --run"
BROWSER_ENV="--environment jsdom --browser --browser.name=chrome --run"

# Change to parent directory, add error handling
cd .. || { echo "Failed to change directory"; exit 1; }

RUSTFLAGS="-Awarnings" cargo test

# Check which environment to test based on the options provided
if [ "$ENVIRONMENT" = "node" ]; then
    # Execute wasm-pack for node, handle possible failures
    #wasm-pack --log-level error test --node -- --features node   || { echo "wasm-pack test failed"; exit 1; }
    echo "Testing ESM Version on Node"
    cd ts || { echo "Failed to change to 'ts' directory"; exit 1; }
    VITE_CJS_IGNORE_WARNING=true npx vitest --config "vitest.config.ts" $NODE_ENV tests/node.test.ts || { echo "Vitest tests failed"; exit 1; }
elif [ "$ENVIRONMENT" = "browser" ]; then
    # Check if chromedriver is installed
    if ! which chromedriver > /dev/null; then
        echo "Error: chromedriver is not installed. Please install chromedriver to continue."
        exit 1
    fi

    # Execute wasm-pack for browser, handle possible failures
    #wasm-pack --log-level error test --headless --chrome -- --features browser   || { echo "wasm-pack test failed"; exit 1; }
    cd ts || { echo "Failed to change to 'ts' directory"; exit 1; }
    echo "Testing ESM Version in Browser"
    VITE_CJS_IGNORE_WARNING=true npx vitest --config "vitest.config.ts" $BROWSER_ENV tests/browser.test.ts || { echo "Vitest tests failed"; exit 1; }
else
    echo "Error: Unknown environment specified. Please use '-e node' or '-e browser'."
    exit 1
fi
