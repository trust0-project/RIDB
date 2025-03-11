#! /bin/bash
set -e  # Exit on any error
npx npm-check-updates -u @trust0/ridb -t newest
npx npm-check-updates -u @trust0/ridb-core -t newest
npm i