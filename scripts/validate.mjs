import fs from 'node:fs';
const root = new URL('../', import.meta.url);
const html = fs.readFileSync(new URL('ui/index.html', root), 'utf8');
const js = fs.readFileSync(new URL('ui/app.js', root), 'utf8');
const tauri = JSON.parse(fs.readFileSync(new URL('src-tauri/tauri.conf.json', root), 'utf8'));
const cargo = fs.readFileSync(new URL('src-tauri/Cargo.toml', root), 'utf8');

const ids = new Set([...html.matchAll(/\bid="([^"]+)"/g)].map(m => m[1]));
const duplicateIds = [...html.matchAll(/\bid="([^"]+)"/g)].map(m=>m[1]).filter((id,i,a)=>a.indexOf(id)!==i);
if (duplicateIds.length) throw new Error(`Duplicate HTML ids: ${[...new Set(duplicateIds)].join(', ')}`);

const jsIds = [...js.matchAll(/(?<!\$)\$\('([^']+)'\)/g)].map(m=>m[1]);
const dynamic = new Set(['add-app-link','new-app-link']);
const missing = [...new Set(jsIds.filter(id => !ids.has(id) && !dynamic.has(id)))];
if (missing.length) throw new Error(`JS references missing HTML ids: ${missing.join(', ')}`);

const cargoVersion = cargo.match(/^version = "([^"]+)"/m)?.[1];
if (cargoVersion !== tauri.version) throw new Error(`Version mismatch: Cargo ${cargoVersion}, Tauri ${tauri.version}`);
if (tauri.version !== '0.5.0') throw new Error(`Unexpected v0.5 package version: ${tauri.version}`);

console.log(`DOM references: OK (${ids.size} unique ids)`);
console.log(`Version consistency: OK (${tauri.version})`);
console.log('JSON parse: OK');
