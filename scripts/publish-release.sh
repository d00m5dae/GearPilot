#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
  echo "Usage: ./scripts/publish-release.sh 0.5.0"
  exit 2
fi
VERSION="${VERSION#v}"

CONFIG_VERSION="$(python3 - <<'PY'
import json
print(json.load(open('src-tauri/tauri.conf.json'))['version'])
PY
)"

if [[ "$VERSION" != "$CONFIG_VERSION" ]]; then
  echo "Version mismatch: requested $VERSION but tauri.conf.json is $CONFIG_VERSION"
  exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
  echo "Working tree is not clean. Commit your changes before publishing."
  exit 1
fi

./scripts/release-check.sh

TAG="v$VERSION"
if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "Tag $TAG already exists."
  exit 1
fi

echo "Creating $TAG and pushing it. GitHub Actions will build the installers."
git tag -a "$TAG" -m "GearPilot $TAG"
git push origin "$TAG"
