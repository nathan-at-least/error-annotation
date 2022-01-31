#!/bin/bash

set -efuxo pipefail

cd "$(dirname "$(readlink -f "$0")")/.."

# Don't proceed if there are uncommitted changes:
[ "$(git status --porcelain | wc -l)" -eq 0 ]

# Don't proceed if the current branch is not main:
[ "$(git rev-parse --abbrev-ref HEAD)" = 'main' ]

# Ensure we're on the latest upstream main revision:
git pull

CURRENT="$(cargo metadata --format-version 1 --no-deps | jq '.packages[0].version')"
NEXT="$(echo "$CURRENT" | sed 's/\.[0-9]*$//').$(eval "$(echo "$CURRENT" | sed 's/^[0-9]*\.[0-9]*\.//')" + 1)"

git checkout -b "v$NEXT"
sed -i "s/version = $CURRENT/version = $NEXT/" ./Cargo.toml
git commit -pm "Update version for release v$NEXT"
