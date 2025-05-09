#!/usr/bin/env bash

set -e

./patch.sh

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

if git rev-parse "refs/tags/${TAG}" >/dev/null 2>&1; then
  echo "Error: Tag ${TAG} already exists." >&2
  exit 1
fi

git add .
git commit -m "Release ${TAG}"
git tag -a "${TAG}" -m "Release ${TAG}"
git push --follow-tags

echo "Created commit with tag ${TAG}"
