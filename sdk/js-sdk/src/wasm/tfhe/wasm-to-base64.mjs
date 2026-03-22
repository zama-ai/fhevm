#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const dir = dirname(fileURLToPath(import.meta.url));
const wasmPath = join(dir, "tfhe_bg.v1.5.3.wasm");
const outPath = join(dir, "tfhe_bg.v1.5.3.wasm.base64.js");

const wasmBytes = readFileSync(wasmPath);
const base64 = wasmBytes.toString("base64");
const sha256 = createHash("sha256").update(base64).digest("hex");

writeFileSync(
  outPath,
  `// Auto-generated — do not edit. Run: node wasm-to-base64.mjs
// SHA-256: ${sha256}
export const tfheWasmBase64 = "${base64}";
`,
);

const sizeMB = (wasmBytes.length / 1024 / 1024).toFixed(2);
const outSizeMB = (base64.length / 1024 / 1024).toFixed(2);
console.log(`SHA-256:   ${sha256}`);
console.log(`Converted ${sizeMB}MB wasm -> ${outSizeMB}MB base64 JS`);
