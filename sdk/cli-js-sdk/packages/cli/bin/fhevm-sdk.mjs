#!/usr/bin/env node
import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

if (process.argv[2] === "completion-server") {
  await import("./completion-server.mjs");
} else {
  const runtime = fileURLToPath(new URL("../dist/index.mjs", import.meta.url));
  if (!existsSync(runtime)) {
    console.error(
      'Missing dist/index.mjs. Run "pnpm run build" before using the linked CLI.',
    );
    process.exitCode = 1;
  } else {
    await import("../dist/index.mjs");
  }
}
