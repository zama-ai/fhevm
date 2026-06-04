#!/usr/bin/env node
if (process.argv[2] === "completion-server") {
  await import("./completion-server.mjs");
} else {
// Global bin entry. Importing tsx here resolves it against the project's
// node_modules from any working directory and registers its loader before
// the TypeScript entry point is imported.
  await import("tsx");

  await import("../index.ts");
}
