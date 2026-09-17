#!/usr/bin/env bun
import { existsSync } from "node:fs";
import { assertOneWorkerPerQueue } from "../src/flow/queue-ownership";
import { STATE_FILE } from "../src/layout";
import type { State } from "../src/types";
import { run } from "../src/utils/process";

try {
  const args = process.argv.slice(2);
  const allowMissing = args.includes("--allow-missing");
  const cpuOnly = args.includes("--cpu-only");
  const noBlueGreen = args.includes("--no-blue-green");
  const positional = args.filter((arg) => !["--allow-missing", "--cpu-only", "--no-blue-green"].includes(arg));
  if (positional.length > 1) throw new Error("usage: queue-ownership.ts [operator-count] [--allow-missing] [--cpu-only] [--no-blue-green]");
  const state = existsSync(STATE_FILE) ? await Bun.file(STATE_FILE).json() as State : undefined;
  if (noBlueGreen && state?.scenario.kind === "blue-green") throw new Error("GPU handover does not support blue/green writer ownership");
  let count = Number(positional[0] || state?.scenario.topology?.count || 0);
  if (!count) {
    const listed = await run(["docker", "ps", "-a", "--format", "{{.Names}}"]);
    const indexes = listed.stdout.split("\n").flatMap((name) => {
      const match = /^coprocessor(\d*)-tfhe-worker$/.exec(name);
      return match ? [Number(match[1] || 0)] : [];
    }).sort((a, b) => a - b);
    if (!indexes.length || indexes.some((index, position) => index !== position)) throw new Error("Cannot identify a complete operator fleet");
    count = indexes.length;
  }
  if (!Number.isSafeInteger(count) || count < 1) throw new Error("Expected a positive operator count");
  if (state && state.scenario.topology.count !== count) throw new Error("Requested operator count disagrees with the active scenario");
  const scenario = state?.scenario ?? {kind: "coprocessor-consensus", topology: {count, threshold: count}} as State["scenario"];
  await assertOneWorkerPerQueue({scenario}, {allowGpu: !cpuOnly, requireEveryRole: !allowMissing});
  console.log(`validity: ${count} operator queues have no rogue workers${allowMissing ? " (stopped roles permitted)" : "; all expected TFHE, SNS and ZK owners are running"}`);
} catch (error) {
  console.error(`validity: FAIL ${error instanceof Error ? error.message : String(error)}`);
  process.exitCode = 1;
}
