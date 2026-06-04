#!/usr/bin/env node
// Global bin entry. Importing tsx here resolves it against the project's
// node_modules from any working directory and registers its loader before
// the TypeScript entry point is imported.
import "tsx";

await import("../index.ts");
