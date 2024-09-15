#!/bin/bash

set -e

if [ -z "$1" ]; then
    echo "Error: Missing release_version parameter. Please check what happened."
    exit 1
fi

release_version="$1"

# Determine the publish tag based on the RC environment variable
if [ -z "$RC" ]; then
    publish_tag="latest"
else
    publish_tag="rc"
fi

echo "Publishing with tag: $publish_tag"

# Updates the version to the release
npm version "$release_version" --git-tag-version false

# Generates the build
npm run build

# Gets the published versions in the registry
version_list=$(npm view @elribonazo/ridb versions)
published_versions=${version_list//[\[\]]/}

# Checks if it's been already published to npmjs
if [[ ${published_versions[@]} =~ "'$release_version'" ]]; then
    echo "$release_version is already published. Skipping publication."
else
    npm publish --access public --tag "$publish_tag"
fi
