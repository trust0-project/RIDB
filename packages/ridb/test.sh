#!/usr/bin/env bash
set -e  # Exit on any error
# Initialize variables
ENVIRONMENT=""
# Define test environment options
NODE_ENV="--environment node --run"
BROWSER_ENV="--environment jsdom --browser --browser.name=chrome --run"

# Parse command-line options
while getopts e: option
do
    case "${option}" in
        e) ENVIRONMENT=${OPTARG};;
    esac
done

# Check required options
if [ -z "$ENVIRONMENT" ]; then
    echo "Testing ESM Version on both Node and Browser environments"
    
    echo "Testing ESM Version on Node"
    VITE_CJS_IGNORE_WARNING=true npx vitest --config "vitest.config.ts" $NODE_ENV tests/node.test.ts || { echo "Vitest tests failed"; exit 1; }
    
    # Check if chromedriver is installed
    if ! which chromedriver > /dev/null; then
        echo "Error: chromedriver is not installed. Please install chromedriver to continue."
        exit 1
    fi
    echo "Testing ESM Version in Browser"
    VITE_CJS_IGNORE_WARNING=true npx vitest --config "vitest.config.ts" $BROWSER_ENV tests/browser.test.ts || { echo "Vitest tests failed"; exit 1; }
    exit 0
fi


# Check which environment to test based on the options provided
if [ "$ENVIRONMENT" = "node" ]; then
    echo "Testing ESM Version on Node"
    VITE_CJS_IGNORE_WARNING=true npx vitest --config "vitest.config.ts" $NODE_ENV tests/node.test.ts || { echo "Vitest tests failed"; exit 1; }
elif [ "$ENVIRONMENT" = "browser" ]; then
    # Check if chromedriver is installed
    if ! which chromedriver > /dev/null; then
        echo "Error: chromedriver is not installed. Please install chromedriver to continue."
        exit 1
    fi
    echo "Testing ESM Version in Browser"
    VITE_CJS_IGNORE_WARNING=true npx vitest --config "vitest.config.ts" $BROWSER_ENV tests/browser.test.ts || { echo "Vitest tests failed"; exit 1; }
else
    echo "Error: Unknown environment specified. Please use '-e node' or '-e browser'."
    exit 1
fi
