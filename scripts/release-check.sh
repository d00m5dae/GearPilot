#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
node scripts/validate.mjs
python3 - <<'PY'
import json, pathlib
root = pathlib.Path('.')
conf = json.loads((root/'src-tauri/tauri.conf.json').read_text())
assert conf['bundle']['active'] is True
assert conf['bundle']['targets'] == 'all'
assert (root/'.github/workflows/release.yml').exists()
assert (root/'installer/linux/99-gearpilot-hidraw.rules').exists()
print('Release packaging files: OK')
PY
