// EXEMPLAR — thin CLI subcommand; no orchestration logic of its own.
/**
 * `fhevm runbook` subcommands.
 *
 * run     — load a TypeScript runbook file and execute it with the Stack.
 * receipt — print the receipt of the most recent runbook run.
 *
 * The runbook file must export a default function `(s: Stack) => Promise<void>`.
 * This module owns only file loading and arg dispatch; all domain logic lives
 * in the runbook itself or the Stack implementation.
 */

import path from "node:path";
import { pathToFileURL } from "node:url";

import type { Stack } from "../lib/stack";

export type RunbookRunArgs = {
  /** Path to the .ts runbook file. */
  name: string;
};

export type Runbook = (stack: Stack) => Promise<void>;

/**
 * Resolve and import a runbook file.
 * The runbook must export a default function or a named `run` export.
 */
export const loadRunbook = async (filePath: string): Promise<Runbook> => {
  const resolved = path.resolve(filePath);
  // Cache-bust so repeated invocations in one process pick up file changes.
  const url = `${pathToFileURL(resolved).href}?t=${Date.now()}`;
  const mod = await import(url);
  const fn = mod.default ?? mod.run;
  if (typeof fn !== "function") {
    throw new Error(
      `Runbook "${filePath}" must export a default function or a named run(stack) function.`,
    );
  }
  return fn as Runbook;
};

/** `fhevm runbook run <name>` — execute the runbook at the given path. */
export const runRunbook = async (stack: Stack, args: RunbookRunArgs): Promise<void> => {
  const runbook = await loadRunbook(args.name);
  await runbook(stack);
};

/**
 * `fhevm runbook receipt` — print the markdown receipt of the most recent run.
 *
 * The receipt file path is resolved from the Stack's state directory; this
 * subcommand simply calls stack.state() to locate it and writes to stdout.
 * The actual receipt format is owned by the Stack implementation.
 */
export const runReceipt = async (stack: Stack): Promise<void> => {
  const state = await stack.state();
  // The receipt lives alongside the stack state; print a summary to stdout.
  // Real implementation: read the receipt JSON/MD from the stateDir and render.
  console.log(JSON.stringify(state, null, 2));
};
