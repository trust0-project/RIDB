#!/usr/bin/env bash
set -e  # Exit on any error

# Function to test node environment
test_node() {
    wasm-pack --log-level error test --node -- --features node || { echo "wasm-pack test for node failed"; exit 1; }
}

# Function to test browser environment
test_browser() {
    # Check if chromedriver is installed
    if ! which chromedriver > /dev/null; then
        echo "Error: chromedriver is not installed. Please install chromedriver to continue."
        exit 1
    fi
    wasm-pack --log-level info test --headless --firefox -- --lib --features browser || { echo "wasm-pack test for browser failed"; exit 1; }
}

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
    echo "No environment specified. Testing both node and browser environments."
    cargo test
    test_node
    test_browser
    exit 0
fi

# Check which environment to test based on the options provided
case "$ENVIRONMENT" in
    node)
        cargo test
        test_node
        ;;
    browser)
        cargo test
        test_browser
        ;;
    *)
        echo "Error: Unknown environment specified. Please use '-e node' or '-e browser'."
        exit 1
        ;;
esac
