#!/usr/bin/env bash
#
# Replace vendor/oxidt-ui with the theme from a tagged oxidt-ui release.
#
# The tag below is the pinned version; bump it to upgrade. CI runs this and
# fails if the committed copy differs, so the vendored files cannot drift.

set -euo pipefail

TAG=v0.1.2
DEST=vendor/oxidt-ui

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
curl -fsSL "https://github.com/oxidt/oxidt-ui/archive/refs/tags/$TAG.tar.gz" \
    | tar -xz -C "$tmp" --strip-components=1

rm -rf "$DEST/theme"
cp -R "$tmp/theme" "$DEST/theme"
cp "$tmp/LICENSE" "$tmp/THIRD_PARTY_NOTICES.md" "$DEST/"
echo "$TAG" > "$DEST/VERSION"
