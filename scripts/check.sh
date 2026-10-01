#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "[1/4] JavaScript syntax"
node --check ui/app.js

echo "[2/4] DOM / JSON / version checks"
node scripts/validate.mjs

echo "[3/4] Tauri JSON"
python3 - <<'PY'
import json
for path in ('src-tauri/tauri.conf.json','src-tauri/capabilities/default.json'):
    with open(path, encoding='utf-8') as f: json.load(f)
    print(path, 'OK')
PY

echo "[4/4] Rust"
if command -v cargo >/dev/null 2>&1; then
  if command -v cargo-tauri >/dev/null 2>&1 && [[ ! -f src-tauri/icons/icon.ico ]]; then
    (cd src-tauri && cargo tauri icon ../assets/gearpilot-icon.svg)
  fi
  (cd src-tauri && cargo check)
else
  echo "cargo is not installed in this environment; skipped cargo check"
fi

echo "GearPilot checks complete."
