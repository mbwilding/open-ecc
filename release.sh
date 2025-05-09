#!/usr/bin/env bash

set -e

./patch.sh

if git rev-parse "refs/tags/${TAG}" >/dev/null 2>&1; then
  echo "Error: Tag ${TAG} already exists." >&2
  exit 1
fi

git add .
git commit -m "Release ${TAG}"
git tag -a "${TAG}" -m "Release ${TAG}"
git push --follow-tags

echo "Created commit with tag ${TAG}"
