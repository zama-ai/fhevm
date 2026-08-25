// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// DRAFT — see README.md. Small, dependency-free helpers for deploy-testnet.ts.
//
// Runs on plain `node` (>= 22.6), which strips types at load. That constrains the syntax to the
// "erasable" subset: no `enum`, no `namespace`, no parameter properties, and relative imports must
// carry their `.ts` extension. Union types stand in for enums throughout.

import { spawnSync } from "node:child_process";
import { appendFileSync, existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { relative, resolve, sep } from "node:path";
import { createInterface } from "node:readline/promises";

////////////////////////////////////////////////////////////////////////////////

export type CaptureResult = {
  readonly ok: boolean;
  readonly stdout: string;
  readonly stderr: string;
  readonly code: number;
};

////////////////////////////////////////////////////////////////////////////////

/** Print an error block to stderr and exit non-zero. Never returns. */
export function fail(...lines: readonly string[]): never {
  for (const line of lines) console.error(line);
  process.exit(1);
}

////////////////////////////////////////////////////////////////////////////////

export function say(...lines: readonly string[]): void {
  for (const line of lines) console.log(line);
}

////////////////////////////////////////////////////////////////////////////////

/** Only the first line is labelled; the rest are indented to align under it. */
export function warn(...lines: readonly string[]): void {
  lines.forEach((line, i) => console.log(i === 0 ? `  WARNING: ${line}` : `           ${line}`));
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Run a command with its output streaming straight through, and return its exit code.
 *
 * Does NOT throw on failure: several callers need the code so they can record a journal entry
 * before giving up, which is the whole point of keeping the audit trail useful for failed runs.
 */
export function run(cmd: string, args: readonly string[], env?: NodeJS.ProcessEnv): number {
  const r = spawnSync(cmd, args as string[], {
    stdio: "inherit",
    env: env ? { ...process.env, ...env } : process.env,
  });
  return r.status ?? 1;
}

////////////////////////////////////////////////////////////////////////////////

/** Run a command and capture its output. Never throws; inspect `.ok`. */
export function capture(cmd: string, args: readonly string[]): CaptureResult {
  const r = spawnSync(cmd, args as string[], { encoding: "utf8" });
  return {
    ok: r.status === 0,
    stdout: (r.stdout ?? "").trim(),
    stderr: (r.stderr ?? "").trim(),
    code: r.status ?? 1,
  };
}

////////////////////////////////////////////////////////////////////////////////

/** Capture, or die with the command line and stderr. For calls whose failure is not recoverable. */
export function captureOrFail(cmd: string, args: readonly string[]): string {
  const r = capture(cmd, args);
  if (!r.ok) {
    fail(`Error: \`${cmd} ${args.join(" ")}\` failed (exit ${r.code}).`, r.stderr ? `       ${r.stderr}` : "");
  }
  return r.stdout;
}

////////////////////////////////////////////////////////////////////////////////

export function requireTool(name: string): void {
  if (!capture("command", ["-v", name]).ok && !capture("which", [name]).ok) {
    fail(`Error: ${name} not on PATH.`);
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Parse a hex quantity as forge writes it ("0x5f3a91") into a number.
 *
 * Block numbers and gas values are well inside Number's exact-integer range; anything that is not
 * is a bug worth surfacing rather than silently rounding, hence the explicit guard.
 */
export function hexToNumber(hex: string | null | undefined): number | null {
  if (hex === null || hex === undefined) return null;
  const n = BigInt(hex);
  if (n > BigInt(Number.MAX_SAFE_INTEGER)) fail(`Error: value out of safe integer range: ${hex}`);
  return Number(n);
}

////////////////////////////////////////////////////////////////////////////////

export function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms));
}

////////////////////////////////////////////////////////////////////////////////

/** Case-insensitive address comparison. Neither side is assumed to be checksummed. */
export function sameAddress(a: string, b: string): boolean {
  return a.toLowerCase() === b.toLowerCase();
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Is `child` the same as, or underneath, `parent`?
 *
 * Compares resolved paths rather than strings, so `..` segments and a trailing slash cannot smuggle
 * a path out of the directory it is being checked against.
 */
export function isInside(child: string, parent: string): boolean {
  const rel = relative(resolve(parent), resolve(child));
  return rel === "" || (!rel.startsWith("..") && !rel.startsWith(sep) && !/^[A-Za-z]:/.test(rel));
}

////////////////////////////////////////////////////////////////////////////////

export function readJson<T>(path: string): T | null {
  if (!existsSync(path)) return null;
  return JSON.parse(readFileSync(path, "utf8")) as T;
}

////////////////////////////////////////////////////////////////////////////////

/** Append one JSON object per line. Creates the file if absent; never rewrites what is there. */
export function appendJsonl(path: string, rows: readonly unknown[]): void {
  if (rows.length === 0) return;
  appendFileSync(path, rows.map((r) => JSON.stringify(r)).join("\n") + "\n");
}

////////////////////////////////////////////////////////////////////////////////

export function readJsonl<T>(path: string): T[] {
  if (!existsSync(path)) return [];
  return readFileSync(path, "utf8")
    .split("\n")
    .filter((l) => l.trim() !== "")
    .map((l) => JSON.parse(l) as T);
}

////////////////////////////////////////////////////////////////////////////////

export function ensureDir(path: string): void {
  mkdirSync(path, { recursive: true });
}

////////////////////////////////////////////////////////////////////////////////

export function removeIfPresent(...paths: readonly string[]): void {
  for (const p of paths) rmSync(p, { recursive: true, force: true });
}

////////////////////////////////////////////////////////////////////////////////

/** Left-padded fixed-width column, truncated rather than allowed to overflow and break alignment. */
export function pad(value: string, width: number): string {
  const s = value.length > width ? value.slice(0, width) : value;
  return s.padEnd(width);
}

////////////////////////////////////////////////////////////////////////////////

export async function confirm(question: string): Promise<boolean> {
  const rl = createInterface({ input: process.stdin, output: process.stdout });
  try {
    const answer = await rl.question(question);
    return answer.trim().toLowerCase() === "y";
  } finally {
    rl.close();
  }
}
