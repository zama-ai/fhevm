import { describe, expect, test } from "bun:test";
import { stressConverged, validateStressFixture } from "./commands/manifest-healing-stress";

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
  test("rejects unsafe or incomplete fixture identities", () => {
    const f = { chainId: 12345, handles, heads: handles, roots: handles, rounds: 0 };
    expect(validateStressFixture(f, 12345)).toEqual(f);
    expect(() => validateStressFixture(f, 1)).toThrow();
    expect(() => validateStressFixture({ ...f, handles: ["bad", ...handles.slice(1)] }, 12345)).toThrow();
    expect(() => validateStressFixture({ ...f, heads: handles.slice(1) }, 12345)).toThrow();
  });
});
