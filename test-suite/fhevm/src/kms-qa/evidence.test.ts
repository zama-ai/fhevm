import { describe, expect, test } from "bun:test";

import {
  EvidenceRecorder,
  formatDuration,
  formatEvidenceLine,
  formatEvidenceTable,
  formatFields,
  txEvidenceFields,
  type EvidenceEntry,
} from "./evidence";
import type { Receipt } from "../kms-onchain";

const entry = (overrides: Partial<EvidenceEntry> = {}): EvidenceEntry => ({
  seq: 1,
  caseId: "epoch-rotation",
  kind: "tx",
  label: "defineNewEpochForCurrentKmsContext",
  startedAtMs: 0,
  durationMs: 1200,
  ok: true,
  fields: {},
  ...overrides,
});

describe("kms-qa evidence formatDuration", () => {
  test("renders sub-second durations in milliseconds", () => {
    expect(formatDuration(0)).toBe("0ms");
    expect(formatDuration(999)).toBe("999ms");
  });

  test("renders one-second-and-over in seconds with one decimal", () => {
    expect(formatDuration(1000)).toBe("1.0s");
    expect(formatDuration(94_300)).toBe("94.3s");
  });
});

describe("kms-qa evidence formatFields", () => {
  test("preserves insertion order", () => {
    expect(formatFields({ txHash: "0x9f", block: "1421" })).toBe("txHash=0x9f block=1421");
  });

  test("renders an empty map as an empty string", () => {
    expect(formatFields({})).toBe("");
  });
});

describe("kms-qa evidence formatEvidenceLine", () => {
  test("includes prefix, case, kind, verdict, timing and fields", () => {
    const line = formatEvidenceLine(entry({ fields: { txHash: "0x9f" } }));
    expect(line).toBe("[kms-context-qa][epoch-rotation][tx] defineNewEpochForCurrentKmsContext ok 1.2s txHash=0x9f");
  });

  test("marks a failed step so a failing run is greppable", () => {
    expect(formatEvidenceLine(entry({ ok: false }))).toContain("FAILED");
  });

  test("omits timing for notes, which have no duration", () => {
    const line = formatEvidenceLine(entry({ kind: "note", durationMs: 0, label: "handoff" }));
    expect(line).toBe("[kms-context-qa][epoch-rotation][note] handoff ok");
  });
});

describe("kms-qa evidence formatEvidenceTable", () => {
  test("reports the empty case explicitly rather than rendering nothing", () => {
    expect(formatEvidenceTable([])).toBe("(no evidence recorded)");
  });

  test("right-aligns sequence numbers against the widest one", () => {
    const rendered = formatEvidenceTable([entry({ seq: 9 }), entry({ seq: 10 })]);
    const [first, second] = rendered.split("\n");
    expect(first).toContain("   9  ok");
    expect(second).toContain("  10  ok");
  });

  test("is independent of the wall clock — durations come from the entries", () => {
    expect(formatEvidenceTable([entry({ durationMs: 2500 })])).toContain("2.5s");
  });
});

describe("kms-qa evidence txEvidenceFields", () => {
  const receipt = (extra: Record<string, unknown> = {}): Receipt =>
    ({ status: "0x1", logs: [], ...extra }) as unknown as Receipt;

  test("extracts the fields cast send returns beyond the narrowed Receipt type", () => {
    const fields = txEvidenceFields(
      receipt({ transactionHash: "0xabc", blockNumber: "0x58d", gasUsed: "0x11d88" }),
    );
    expect(fields.txHash).toBe("0xabc");
    expect(fields.block).toBe("1421");
    expect(fields.gasUsed).toBe("73096");
  });

  test("accepts decimal as well as hex numerics", () => {
    expect(txEvidenceFields(receipt({ blockNumber: 1421, gasUsed: "73096" }))).toMatchObject({
      block: "1421",
      gasUsed: "73096",
    });
  });

  test("omits missing or unparseable fields instead of throwing — evidence must never fail a run", () => {
    const fields = txEvidenceFields(receipt({ blockNumber: "not-a-number" }));
    expect(fields.block).toBeUndefined();
    expect(fields.txHash).toBeUndefined();
    expect(fields.status).toBe("0x1");
  });

  test("always reports the log count, so a missing event is visible in the trail", () => {
    expect(txEvidenceFields(receipt({ logs: [{ address: "0x0", topics: [], data: "0x" }] })).logCount).toBe("1");
  });
});

describe("kms-qa EvidenceRecorder", () => {
  test("numbers entries monotonically across cases", async () => {
    const recorder = new EvidenceRecorder();
    const first = recorder.forCase("a");
    const second = recorder.forCase("b");
    await first.step("call", "one", {}, async () => undefined);
    await second.step("call", "two", {}, async () => undefined);
    expect(recorder.entries.map((item) => item.seq)).toEqual([1, 2]);
    expect(recorder.entries.map((item) => item.caseId)).toEqual(["a", "b"]);
  });

  test("records a failed step and rethrows the original error", async () => {
    const recorder = new EvidenceRecorder();
    const evidence = recorder.forCase("a");
    const boom = new Error("boom");
    await expect(
      evidence.step("tx", "failing", {}, async () => {
        throw boom;
      }),
    ).rejects.toThrow("boom");
    expect(recorder.entries).toHaveLength(1);
    expect(recorder.entries[0]!.ok).toBe(false);
  });

  test("returns the task result unchanged", async () => {
    const recorder = new EvidenceRecorder();
    const result = await recorder.forCase("a").step("call", "value", {}, async () => 42n);
    expect(result).toBe(42n);
  });

  test("scopes entries per case", async () => {
    const recorder = new EvidenceRecorder();
    await recorder.forCase("a").step("call", "one", {}, async () => undefined);
    recorder.forCase("b").note("note", "two");
    expect(recorder.entriesForCase("a")).toHaveLength(1);
    expect(recorder.entriesForCase("b")).toHaveLength(1);
  });
});
