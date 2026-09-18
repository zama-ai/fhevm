import { describe, expect, test } from "bun:test";
import { assertDriftMatrix, assertManifestFault, DRIFT_CASES, validateHealingFixture, type Finding } from "./commands/manifest-healing";

const handle = (i: number) => `0x${i.toString(16).padStart(64, "0")}`;
const fixture = {
  chainId: 12345, rootBlock: 42, rootBlockHash: handle(99),
  roots: Array.from({ length: 10 }, (_, i) => handle(i + 1)),
  children: Array.from({ length: 10 }, (_, i) => handle(i + 11)), joined: handle(21), tail: handle(22),
};
const digests = fixture.roots.map((_, i) => handle(i + 100));
const rows = (healed: boolean): Finding[] => [
  ...DRIFT_CASES.map((entry, i) => ({
    handle: fixture.roots[i]!.slice(2), detection_kind: "verified", reason: entry.reason,
    is_contained: entry.reason === "ct64_mismatch", can_be_healed: entry.healable && !healed,
    healed_at: healed && entry.healable ? "2026-09-21T12:00:00Z" : null, target: entry.healable ? digests[i]! : null,
  })),
  ...[fixture.children[0]!, fixture.children[1]!, fixture.joined, fixture.tail].map(h => ({
    handle: h.slice(2), detection_kind: "inferred", reason: "ct64_mismatch", is_contained: true,
    can_be_healed: false, healed_at: healed ? "2026-09-21T12:00:00Z" : null, target: healed ? handle(101) : null,
  })),
];

describe("manifest healing matrix", () => {
  test("covers all nine reasons with two ct64 roots", () => {
    expect(new Set(DRIFT_CASES.map(c => c.reason)).size).toBe(9);
    expect(DRIFT_CASES.filter(c => c.reason === "ct64_mismatch").length).toBe(2);
    expect(validateHealingFixture(fixture)).toEqual(fixture);
    expect(() => validateHealingFixture({ ...fixture, roots: [] })).toThrow();
    expect(() => validateHealingFixture({ ...fixture, tail: fixture.joined })).toThrow();
    expect(() => validateHealingFixture({ ...fixture, joined: "0x'" })).toThrow();
  });

  test("direct and inferred findings must match before and after healing", () => {
    expect(() => assertDriftMatrix(fixture, rows(false), digests, false)).not.toThrow();
    expect(() => assertDriftMatrix(fixture, rows(true), digests, true)).not.toThrow();
    expect(() => assertDriftMatrix(fixture, rows(true), digests, false)).toThrow();
    expect(() => assertDriftMatrix(fixture, rows(false), digests, true)).toThrow();
  });

  test("detects false containment, wrong pins, missing inference, and accidental non-healable repair", () => {
    for (const mutate of [
      (r: Finding[]) => { r[5]!.is_contained = true; },
      (r: Finding[]) => { r[0]!.target = handle(999); },
      (r: Finding[]) => { r[10]!.detection_kind = "verified"; },
      (r: Finding[]) => { r[7]!.healed_at = "unexpected"; },
      (r: Finding[]) => { r[6]!.can_be_healed = true; },
      (r: Finding[]) => { r.pop(); },
      (r: Finding[]) => { r.push({ ...r[10]!, handle: fixture.children[5]!.slice(2) }); },
    ]) {
      const observed = rows(false);
      mutate(observed);
      expect(() => assertDriftMatrix(fixture, observed, digests, false)).toThrow();
    }
  });

  test("signed faults must change the intended descriptor field or omit it", () => {
    const healthy = { handle: handle(1), status: "computed", ct64_digest: handle(100), ct128_digest: handle(101), keyset_id: "0x2" };
    const flipped = `0x01${healthy.ct64_digest.slice(4)}`;
    expect(() => assertManifestFault(healthy, { ...healthy, ct64_digest: flipped }, "ct64_digest_bit_flip")).not.toThrow();
    expect(() => assertManifestFault(healthy, healthy, "ct64_digest_bit_flip")).toThrow();
    expect(() => assertManifestFault(healthy, undefined, "missing_here")).not.toThrow();
    expect(() => assertManifestFault(healthy, healthy, "missing_here")).toThrow();
    expect(() => assertManifestFault(healthy, { handle: handle(1), status: "error" }, "error_here")).not.toThrow();
    expect(() => assertManifestFault(healthy, { ...healthy, status: "error" }, "error_here")).toThrow();
    expect(() => assertManifestFault(healthy, { ...healthy, keyset_id: "0x3" }, "keyset_id_bit_flip")).not.toThrow();
    expect(() => assertManifestFault(healthy, { ...healthy, keyset_id: "0x4" }, "keyset_id_bit_flip")).toThrow();
  });
});
