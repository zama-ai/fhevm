#!/usr/bin/env bun
import { waitForCoprocessorKeyMaterial } from "../src/flow/readiness";
import { loadState } from "../src/state/state";

try {
  const state = await loadState();
  if (!state) throw new Error("GPU key readiness requires an active stack");
  await waitForCoprocessorKeyMaterial(state, 150, { requireCompressed: true });
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
}
