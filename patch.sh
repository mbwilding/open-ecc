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

# Workspace Cargo.toml
if [ "$(uname)" = "Darwin" ]; then
    sed -i '' -E "s/^(version[[:space:]]*=[[:space:]]*\")[0-9]+\.[0-9]+\.[0-9]+(\")/\1$VERSION\2/" Cargo.toml
else
    sed -i -r "s/^(version[[:space:]]*=[[:space:]]*\")[0-9]+\.[0-9]+\.[0-9]+(\")/\1$VERSION\2/" Cargo.toml
fi

# Crates Cargo.toml
for file in ./crates/*/Cargo.toml; do
    if [ "$(uname)" = "Darwin" ]; then
        sed -i '' -E '/path/ s/(version[[:space:]]*=[[:space:]]*")[0-9]+\.[0-9]+\.[0-9]+(")/\1'"$VERSION"'\2/' "$file"
    else
        sed -i -r '/path/ s/(version[[:space:]]*=[[:space:]]*")[0-9]+\.[0-9]+\.[0-9]+(")/\1'"$VERSION"'\2/' "$file"
    fi
done

cargo generate-lockfile
