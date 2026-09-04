import { describe, expect, test } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

// The change-detection filters in coprocessor-docker-build decide whether a
// service gets an image for a commit. A filter that misses one of the service's
// dependencies is silent in the worst way: the build is "skipped", the
// orchestration reads that as "no diff", and the stack runs a *stale* image for
// that service beside freshly built siblings.
//
// That is not hypothetical. PR-04 changed `fhevm-engine-common`, which every
// worker links, and no filter watched it -- so gw-listener, tx-sender and
// zkproof-worker had no image at that commit and an e2e run had to fall back to
// main's. It went unnoticed because the change happened to be GPU-gated, so the
// CPU behaviour of the stale images was identical. The next such change will not
// be so lucky.
//
// So this asserts the property directly: every internal path dependency a
// service actually builds against, transitively, is watched by its filter.
// Dev-dependencies and optional dependencies are excluded -- they do not reach
// the release binary the image ships.

const repoRoot = resolve(import.meta.dir, "../../..");
const workflow = join(repoRoot, ".github/workflows/coprocessor-docker-build.yml");

/** Crate directories a manifest depends on by path, excluding dev and optional. */
const pathDeps = (crateDir: string): string[] => {
  const manifest = join(crateDir, "Cargo.toml");
  if (!existsSync(manifest)) return [];
  const text = readFileSync(manifest, "utf8");
  const deps: string[] = [];
  let section = "";
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    const header = line.match(/^\[([^\]]+)\]/);
    if (header) {
      section = header[1];
      continue;
    }
    // Only sections that affect the shipped binary.
    if (!/^(dependencies|build-dependencies|target\..*\.dependencies)$/.test(section)) continue;
    const path = line.match(/path\s*=\s*"([^"]+)"/);
    if (!path) continue;
    if (/optional\s*=\s*true/.test(line)) continue;
    deps.push(resolve(crateDir, path[1]));
  }
  return deps;
};

/** Transitive closure of internal path dependencies, as repo-relative paths. */
const closure = (crateDir: string): string[] => {
  const seen = new Set<string>();
  const stack = [resolve(crateDir)];
  while (stack.length > 0) {
    const current = stack.pop()!;
    if (seen.has(current)) continue;
    seen.add(current);
    stack.push(...pathDeps(current));
  }
  seen.delete(resolve(crateDir));
  return [...seen].map((p) => relative(repoRoot, p)).sort();
};

/** The filter block for each service, read out of the workflow's literal YAML. */
const filters = (): Map<string, string[]> => {
  const lines = readFileSync(workflow, "utf8").split("\n");
  const out = new Map<string, string[]>();
  let current: string | null = null;
  for (let i = 0; i < lines.length; i += 1) {
    const name = lines[i].match(/^ {8}([a-z0-9-]+):$/);
    if (name) {
      current = name[1];
      out.set(current, []);
      continue;
    }
    const entry = lines[i].match(/^ {10}- (.+)$/);
    if (entry && current) {
      out.get(current)!.push(entry[1].trim());
      continue;
    }
    if (lines[i].trim() !== "" && !entry) current = null;
  }
  return out;
};

// Service filter name -> the crate directory it builds.
const SERVICE_CRATES: Record<string, string> = {
  "gw-listener": "coprocessor/fhevm-engine/gw-listener",
  "host-listener": "coprocessor/fhevm-engine/host-listener",
  "sns-worker": "coprocessor/fhevm-engine/sns-worker",
  "tfhe-worker": "coprocessor/fhevm-engine/tfhe-worker",
  "tx-sender": "coprocessor/fhevm-engine/transaction-sender",
  "zkproof-worker": "coprocessor/fhevm-engine/zkproof-worker",
  "consensus-detector": "coprocessor/fhevm-engine/consensus-detector",
  "upgrade-controller": "coprocessor/fhevm-engine/upgrade-controller",
};

describe("coprocessor-docker-build change detection", () => {
  const parsed = filters();

  test("every Rust service has a filter", () => {
    for (const service of Object.keys(SERVICE_CRATES)) {
      expect(parsed.has(service)).toBe(true);
    }
  });

  for (const [service, crate] of Object.entries(SERVICE_CRATES)) {
    test(`${service} watches every crate it links`, () => {
      const watched = parsed.get(service) ?? [];
      const missing = closure(join(repoRoot, crate)).filter(
        (dep) => !watched.some((w) => w === `${dep}/**` || w === dep),
      );
      // Named rather than counted: the failure has to say which dependency
      // would ship stale, or the next person has to rediscover this.
      expect(missing).toEqual([]);
    });

    test(`${service} watches its own sources and the workspace manifests`, () => {
      const watched = parsed.get(service) ?? [];
      expect(watched).toContain(`${crate}/**`);
      expect(watched).toContain("coprocessor/fhevm-engine/Cargo.*");
      expect(watched).toContain(".github/workflows/coprocessor-docker-build.yml");
    });
  }
});
