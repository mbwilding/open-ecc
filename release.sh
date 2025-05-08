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

sed -i '' -E "s/^(version[[:space:]]*=[[:space:]]*\")[0-9]+\.[0-9]+\.[0-9]+(\")/\1$VERSION\2/" Cargo.toml
if [ $? -ne 0 ]; then
    echo "Failed to update Cargo.toml"
    exit 1
fi
echo "Cargo.toml version updated to $VERSION"
cargo generate-lockfile

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
