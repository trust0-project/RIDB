#!/bin/bash

set -e

release_version="$1"
release_channel="$2"

if [ -z "$release_version" ]; then
    echo "Error: Missing release_version parameter."
    exit 1
fi

# Determine the publish tag based on the release channel
if [ "$release_channel" == "rc" ]; then
    publish_tag="rc"
else
    publish_tag="latest"
fi

echo "Publishing version: $release_version with tag: $publish_tag"

# Update the version in package.json without creating a git tag
npm version "$release_version"

# Generate the build
npm run build

# Check if the version is already published
if npm view "@elribonazo/ridb@$release_version" > /dev/null 2>&1; then
    echo "Version $release_version is already published. Skipping publication."
else
    npm publish --access public --tag "$publish_tag"
fi
