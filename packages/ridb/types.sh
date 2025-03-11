#! /bin/bash
set -e  # Exit on any error
npx dts-bundle-generator src/index.ts  --no-check -o ./build/index.d.ts
npx dts-bundle-generator src/worker.ts  --no-check -o ./build/worker.d.ts
npx dts-bundle-generator src/testing/index.ts  --no-check -o ./build/testing/index.d.ts