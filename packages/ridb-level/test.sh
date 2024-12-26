#!/usr/bin/env bash
set -e  # Exit on any error
# Initialize variables
ENVIRONMENT=""
# Define test environment options
NODE_ENV="--environment node --run"

VITE_CJS_IGNORE_WARNING=true npx vitest --config "vitest.config.ts" $NODE_ENV tests/node.test.ts