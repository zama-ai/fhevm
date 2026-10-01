import { describe, expect, test } from "bun:test";
import { classifyStressDrift, stressConverged, validateStressFixture } from "./commands/manifest-healing-stress";
import { stressHandleIsCorrupted } from "./commands/manifest-stress-injection";

const handles = [1, 2, 3, 4].map(i => `0x${i.toString(16).padStart(64, "0")}`);
const inventory = { total: 2, healed: 2, unresolved: 0, verified: 1, inferred: 1, lastHealed: "2026-09-23" };
const rows = handles.map(handle => ({ handle: handle.slice(2), bytes: "healthy", length: 100 }));
describe("manifest healing stress convergence", () => {
  test("requires all work and all three ciphertext copies to agree", () => {
    expect(stressConverged(handles, [rows, rows, rows], [inventory, inventory, inventory])).toBe(true);
    expect(stressConverged(handles, [rows, rows, rows.slice(1)], [inventory, inventory, inventory])).toBe(false);
    const damaged = rows.map((r, i) => i === 0 ? { ...r, bytes: "damaged" } : r);
    expect(stressConverged(handles, [rows, rows, damaged], [inventory, inventory, inventory])).toBe(false);
    expect(stressConverged(handles, [rows, rows, rows], [inventory, inventory, { ...inventory, unresolved: 1 }])).toBe(false);
    expect(stressConverged([], [[], [], []], [inventory, inventory, inventory])).toBe(false);
  });
  test("classifies a corrupted handle and the results that reuse it", () => {
    const h = (n: number) => n.toString(16).padStart(64, "0");
    const [a, b, c, d] = [1, 2, 3, 4].map(h);
    const mixed = [10, 11, 12, 13].map(h);
    const heads = [20, 21, 22, 23].map(h);
    const next = [30, 31, 32, 33].map(h);
    const classified = classifyStressDrift([
      { inputs: [a!, b!, c!, d!], roots: [a!, b!, c!, d!], mixed, heads, independent: h(90) },
      { inputs: heads, roots: [a!, b!, c!, d!], mixed: next, heads: [40, 41, 42, 43].map(h), independent: h(91) },
    ], [mixed[0]!]);
    expect(classified.verifiedRoots).toEqual([mixed[0]]);
    expect(classified.descendants).toContain(heads[0]);
    expect(classified.descendants).toContain(next[0]);
    expect(classified.descendants).toContain(next[3]);
    expect(classified.clean).toContain(mixed[1]);
    expect(classified.clean).not.toContain(heads[0]);
  });
  test("selects a corrupted handle from its first byte", () => {
    const handle = (byte: number) => `0x${byte.toString(16).padStart(2, "0")}${"00".repeat(31)}`;
    expect(stressHandleIsCorrupted(handle(0))).toBe(true);
    expect(stressHandleIsCorrupted(handle(4))).toBe(true);
    expect(stressHandleIsCorrupted(handle(1))).toBe(false);
    expect(stressHandleIsCorrupted(handle(7))).toBe(false);
    expect(stressHandleIsCorrupted("0x01")).toBe(false);
  });
  test("rejects unsafe or incomplete fixture identities", () => {
    const f = { chainId: 12345, handles, heads: handles, roots: handles, rounds: 0 };
    expect(validateStressFixture(f, 12345)).toEqual(f);
    expect(() => validateStressFixture(f, 1)).toThrow();
    expect(() => validateStressFixture({ ...f, handles: ["bad", ...handles.slice(1)] }, 12345)).toThrow();
    expect(() => validateStressFixture({ ...f, heads: handles.slice(1) }, 12345)).toThrow();
  });
});
