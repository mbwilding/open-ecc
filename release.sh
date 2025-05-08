#!/usr/bin/env bash

set -e

if [ ! -f Changes.md ]; then
    echo "Changes.md not found"
    exit 1
fi

VERSION=$(grep "^##" Changes.md | head -n 1 | sed -E 's/^##[[:space:]]+([0-9]+\.[0-9]+\.[0-9]+).*/\1/')
if [ -z "$VERSION" ]; then
    echo "Version not found in Changes.md"
    exit 1
fi

TAG="v${VERSION}"

git add .
git commit -m "Release ${TAG}"
git tag "${TAG}"
git push

echo "Created commit with tag ${TAG}"
