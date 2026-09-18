import { describe, expect, test } from "bun:test";

import {
  backendSplit,
  containerFailure,
  waitForCoprocessorKeyMaterial,
  classifyStray,
  ensureOneMaterial,
  keyMaterialReadiness,
  parseSquashBackend,
  parseWorkerDatabase,
  strayWorkerPids,
  waitForRpc,
} from "./flow/readiness";

describe("waitForRpc", () => {
  test("retries until eth_chainId returns a JSON-RPC result", async () => {
    const originalFetch = globalThis.fetch;
    let calls = 0;
    globalThis.fetch = (async () => {
      calls += 1;
      return new Response(
        JSON.stringify(
          calls === 1
            ? { jsonrpc: "2.0", id: 1, error: { code: -32000, message: "not ready" } }
            : { jsonrpc: "2.0", id: 1, result: "0x3039" },
        ),
        {
          status: 200,
          headers: { "content-type": "application/json" },
        },
      );
    }) as unknown as typeof fetch;
    try {
      await waitForRpc("http://localhost:8545");
      expect(calls).toBe(2);
    } finally {
      globalThis.fetch = originalFetch;
    }
  });
});

describe("ensureOneMaterial", () => {
  test("accepts a published compressed keyset without waiting for the legacy key path", async () => {
    const originalFetch = globalThis.fetch;
    const requested: string[] = [];
    globalThis.fetch = (async (input: string | URL | Request) => {
      const url = String(input);
      requested.push(url);
      return new Response(null, { status: url.includes("CompressedXofKeySet") ? 200 : 404 });
    }) as unknown as typeof fetch;
    try {
      await ensureOneMaterial([
        "http://minio:9000/kms-public/PUB/CompressedXofKeySet/key-id",
        "http://minio:9000/kms-public/PUB/ServerKey/key-id",
      ]);
      expect(requested).toHaveLength(2);
      expect(requested[0]).toContain("CompressedXofKeySet");
    } finally {
      globalThis.fetch = originalFetch;
    }
  });
});

describe("strayWorkerPids", () => {
  test("ignores the container processes this stack owns", () => {
    expect(strayWorkerPids(new Set([11, 22]), [11, 22])).toEqual([]);
  });

  test("reports a worker that no container of this stack accounts for", () => {
    // The GPU units' case: containers up, plus a host-run binary on the same queue.
    expect(strayWorkerPids(new Set([11, 22]), [11, 22, 953222])).toEqual([953222]);
  });

  test("reports every stray, not just the first", () => {
    expect(strayWorkerPids(new Set([11]), [953222, 11, 953223])).toEqual([953222, 953223]);
  });
});

describe("parseSquashBackend", () => {
  test("reads gpu_enabled out of the worker's JSON startup line", () => {
    const line = '{"timestamp":"2026-09-02T08:29:31Z","level":"INFO","fields":{"gpu_enabled":true}}';
    expect(parseSquashBackend(line)).toBe("true");
  });

  test("reads the CPU case", () => {
    expect(parseSquashBackend('"gpu_enabled":false,"other":1')).toBe("false");
  });

  test("tolerates a bare key=value form so the guard does not go quiet", () => {
    expect(parseSquashBackend("gpu_enabled=TRUE")).toBe("true");
  });

  test("returns undefined when the line is absent", () => {
    expect(parseSquashBackend("starting sns worker")).toBeUndefined();
  });
});

describe("backendSplit", () => {
  test("a homogeneous fleet is one group", () => {
    expect([...backendSplit(["false", "false", "false"]).keys()]).toEqual(["false"]);
  });

  test("groups operators by backend so the split can be named", () => {
    const split = backendSplit(["true", "false", "false"]);
    expect(split.get("true")).toEqual([0]);
    expect(split.get("false")).toEqual([1, 2]);
  });

  test("an unreadable backend is not evidence of a split", () => {
    // Older image without the startup line: one group, so no failure.
    expect(backendSplit(["false", undefined, "false"]).size).toBe(1);
  });
});

/**
 * Readiness contracts (inventory case HAR-03-READINESS-CONTRACTS).
 *
 * These are the "cannot be evaluated" directions. The key-material wait used to
 * be `Number.parseInt(stdout) < 1` with `allowFailure: true`, so an
 * inaccessible database, a missing `keys` table and an empty result all produced
 * `NaN`, and `NaN < 1` is false -- the gate reported READY for a stack it had
 * not managed to look at. The point of this wait is that a different readiness
 * probe already overstated readiness once, so it failing open is worse than
 * most.
 */
describe("keyMaterialReadiness", () => {
  test("a failed query is never ready", () => {
    const state = keyMaterialReadiness(1, "", 'ERROR: relation "keys" does not exist');
    expect(state.ready).toBe(false);
    expect(state.reason).toContain("query failed");
  });

  test("empty output is never ready", () => {
    expect(keyMaterialReadiness(0, "").ready).toBe(false);
    expect(keyMaterialReadiness(0, "\n").ready).toBe(false);
  });

  test("non-numeric output is never ready", () => {
    const state = keyMaterialReadiness(0, "you are not permitted to log in");
    expect(state.ready).toBe(false);
    expect(state.reason).toContain("unreadable output");
  });

  test("zero key rows is not ready", () => {
    expect(keyMaterialReadiness(0, "0|0|0").ready).toBe(false);
  });

  test("a compressed keyset is ready", () => {
    const state = keyMaterialReadiness(0, "1|1|0");
    expect(state.ready).toBe(true);
    expect(state.compressed).toBe(1);
  });

  test("a legacy ServerKey row is ready on CPU and not on GPU", () => {
    // `sns-worker`'s keyset.rs falls back to the legacy `sns_pk` large object,
    // except under --features gpu where the legacy encoding is refused
    // outright. So the same row is usable material on one topology and not on
    // the other, and one column for every topology reports a legacy CLI stack
    // as unprovisioned.
    expect(keyMaterialReadiness(0, "1|0|1").ready).toBe(true);
    expect(keyMaterialReadiness(0, "1|0|1", "", { requireCompressed: true }).ready).toBe(false);
  });

  test("a key row carrying neither encoding is not ready", () => {
    const state = keyMaterialReadiness(0, "2|0|0");
    expect(state.ready).toBe(false);
    expect(state.reason).toContain("neither compressed_xof_keyset nor sns_pk");
  });
});

/**
 * Worker exclusivity has to be judged by the QUEUE, not by the executable name.
 *
 * A process called `tfhe_worker` pointed at an unrelated database is not a
 * conflict: the crate's own regression suite runs one against 127.0.0.1:1 on
 * purpose, and name-matching aborted a bring-up because of it. A second worker
 * on one of this stack's databases is a conflict whatever it is called.
 */
describe("worker queue attribution", () => {
  test("reads the database from a --database-url flag", () => {
    expect(
      parseWorkerDatabase("tfhe_worker --run-bg-worker --database-url=postgresql://p:p@db:5432/coprocessor_1", ""),
    ).toBe("coprocessor_1");
  });

  test("reads the database from the environment when the flag is absent", () => {
    expect(parseWorkerDatabase("tfhe_worker --run-bg-worker", "DATABASE_URL=postgres://p:p@db:5432/coprocessor\nPATH=/usr/bin")).toBe(
      "coprocessor",
    );
  });

  test("returns nothing when neither is present", () => {
    expect(parseWorkerDatabase("tfhe_worker --run-bg-worker", "PATH=/usr/bin")).toBeUndefined();
  });

  test("a stray on one of this stack's databases is a conflict", () => {
    expect(classifyStray(42, "coprocessor_2", new Set(["coprocessor", "coprocessor_1", "coprocessor_2"]))).toEqual({
      pid: 42,
      kind: "conflict",
      database: "coprocessor_2",
    });
  });

  test("a stray on an unrelated database is not a conflict", () => {
    expect(classifyStray(42, "unused", new Set(["coprocessor"]))).toEqual({
      pid: 42,
      kind: "unrelated",
      database: "unused",
    });
  });

  test("a stray whose target cannot be read fails closed rather than being assumed harmless", () => {
    expect(classifyStray(42, undefined, new Set(["coprocessor"]))).toEqual({ pid: 42, kind: "unknown" });
  });
});


describe("readiness restart evidence", () => {
  test("rejects a currently running crash-loop when RestartCount advances", () => {
    expect(containerFailure({RestartCount: 3, State: {Status: "running", ExitCode: 0}}, 2)).toBe(true);
    expect(containerFailure({RestartCount: 3, State: {Status: "running", ExitCode: 0}}, 3)).toBe(false);
    expect(containerFailure({RestartCount: 3, State: {Status: "restarting", ExitCode: 0}}, 3)).toBe(true);
  });
  test("support-floor key readiness does not contact a database with no keys table", async () => {
    // No scenario or Docker access exists in this fixture: touching either fails.
    await waitForCoprocessorKeyMaterial({versions: {env: {COPROCESSOR_DB_MIGRATION_VERSION: "v0.11.0"}}} as unknown as Parameters<typeof waitForCoprocessorKeyMaterial>[0]);
  });
});


test('queue readiness uses Docker metadata for root-owned workers without ignoring same-queue containers', async () => {
  const {mkdtempSync, writeFileSync, copyFileSync, existsSync, rmSync} = await import('node:fs');
  const {tmpdir} = await import('node:os');
  const path = await import('node:path');
  const dir = mkdtempSync(path.join(tmpdir(), 'queue-readiness-'));
  try {
    const fake = path.join(dir, 'fake.ts');
    writeFileSync(fake, `#!/usr/bin/env bun
const tool = process.argv[1].split('/').pop();
const args = process.argv.slice(2);
if (tool === 'docker') {
  if (args[0] === 'ps') console.log('owned\\nforeign');
  else if (args.includes('-f')) console.log(args.at(-1) === 'coprocessor-tfhe-worker' ? '111' : '0');
  else console.log(JSON.stringify([
    {Name:'/coprocessor-tfhe-worker',State:{Pid:111},Config:{Env:['DATABASE_URL=postgres://db/coprocessor']},NetworkSettings:{Networks:{local:{NetworkID:'local'}}}},
    {Name:'/rogue-tfhe-worker',State:{Pid:999},Config:{Env:process.env.UNKNOWN_TARGET ? [] : ['DATABASE_URL=postgres://db/coprocessor']},NetworkSettings:{Networks:{other:{NetworkID:process.env.FOREIGN_NETWORK}}}},
    {Name:'/other-db',State:{Pid:888},Config:{ExposedPorts:{'5432/tcp':{}}},NetworkSettings:{Networks:{other:{NetworkID:'other',Aliases:['db']}}}},
  ]));
} else if (tool === 'pgrep') {
  if (args.at(-1) === 'tfhe_worker') console.log('111\\n999');
  else process.exit(1);
} else {
  await Bun.write(process.env.PROC_PROBE_FILE, 'attempted');
  console.error('EACCES'); process.exit(1);
}
`, {mode:0o755});
    for (const name of ['docker','pgrep','cat','readlink']) copyFileSync(fake,path.join(dir,name));
    const readinessModule = new URL('./flow/readiness.ts', import.meta.url).pathname;
    const script = `import {assertOneWorkerPerQueue} from ${JSON.stringify(readinessModule)}; await assertOneWorkerPerQueue({scenario:{topology:{count:1,threshold:1}}});`;
    for (const [network, unknown, expected] of [['other','',0],['local','',1],['local','1',1]] as const) {
      const result = Bun.spawnSync([process.execPath,'-e',script],{env:{...process.env,PATH:`${dir}:${process.env.PATH}`,FOREIGN_NETWORK:network,UNKNOWN_TARGET:unknown,PROC_PROBE_FILE:path.join(dir,'proc-read')}});
      expect(result.exitCode, result.stderr.toString()).toBe(expected);
      expect(existsSync(path.join(dir,'proc-read'))).toBe(false);
    }
  } finally {rmSync(dir,{recursive:true,force:true});}
});
