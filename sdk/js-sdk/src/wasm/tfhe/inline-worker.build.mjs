#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { transformSync } from "esbuild";

const __dirname = dirname(fileURLToPath(import.meta.url));
const input = resolve(__dirname, "tfhe-worker.v1.5.3.mjs");
const output = resolve(__dirname, "tfhe-worker.v1.5.3.inline.js");

const code = readFileSync(input, "utf-8");

const { code: minified } = transformSync(code, {
  minify: true,
  loader: "js",
});

const base64 = Buffer.from(minified).toString("base64");
const sha256 = createHash("sha256").update(base64).digest("hex");

writeFileSync(
  output,
  `// Auto-generated — do not edit. Run: node inline-worker.build.mjs
// SHA-256: ${sha256}
export const workerBase64 = ${JSON.stringify(base64)};
`,
);

const savings = ((1 - minified.length / code.length) * 100).toFixed(1);
console.log(`Original:  ${code.length} bytes`);
console.log(`Minified:  ${minified.length} bytes (${savings}% smaller)`);
console.log(`Base64:    ${base64.length} chars`);
console.log(`SHA-256:   ${sha256}`);
console.log(`Written:   ${output}`);
