import { describe, expect, it } from "vitest";

import { ceilingWarnings, PROTOCOL_LIMITS } from "../src/scenario/limits";
import { scenarioSchema } from "../src/scenario/schema";

const scenario = (value: unknown) => scenarioSchema.parse(value);

describe("PROTOCOL_LIMITS", () => {
  it("pins the authoritative ceilings", () => {
    expect(PROTOCOL_LIMITS).toEqual({ inputProofRps: 20, decryptRps: 10 });
  });
});

describe("ceilingWarnings", () => {
  it("returns nothing for gentle constant scenarios", () => {
    expect(
      ceilingWarnings(
        scenario({
          name: "gentle",
          flows: [{ flow: "input-proof", weight: 1 }],
          shape: { kind: "constant", rps: 5, durationSec: 60 },
        }),
      ),
    ).toEqual([]);
  });

  it("warns when input-proof peak exceeds its ceiling", () => {
    const warnings = ceilingWarnings(
      scenario({
        name: "hot-ip",
        flows: [{ flow: "input-proof", weight: 1 }],
        shape: { kind: "constant", rps: 25, durationSec: 60 },
      }),
    );
    expect(warnings).toHaveLength(1);
    expect(warnings[0]).toContain("input-proof peak ~25.0 rps");
    expect(warnings[0]).toContain("20 rps");
  });

  it("sums the three decrypt flows against the combined ceiling", () => {
    const warnings = ceilingWarnings(
      scenario({
        name: "hot-decrypt",
        flows: [
          { flow: "public-decrypt", weight: 1, handlesPerRequest: 2 },
          { flow: "user-decrypt", weight: 1 },
          { flow: "delegated-user-decrypt", weight: 1 },
        ],
        shape: { kind: "constant", rps: 15, durationSec: 60 },
      }),
    );
    expect(warnings).toHaveLength(1);
    expect(warnings[0]).toContain("combined decrypt peak ~15.0 rps");
    expect(warnings[0]).toContain("10 rps");
  });

  it("applies the weight share when computing per-flow peaks", () => {
    // 30 rps total, input-proof share 1/3 -> 10 rps (under 20, no warning);
    // decrypt share 2/3 -> 20 rps (over 10, warns).
    const warnings = ceilingWarnings(
      scenario({
        name: "weighted",
        flows: [
          { flow: "input-proof", weight: 1 },
          { flow: "user-decrypt", weight: 2 },
        ],
        shape: { kind: "constant", rps: 30, durationSec: 60 },
      }),
    );
    expect(warnings).toHaveLength(1);
    expect(warnings[0]).toContain("combined decrypt peak ~20.0 rps");
  });

  it("uses the max segment endpoint for segmented shapes", () => {
    const warnings = ceilingWarnings(
      scenario({
        name: "ramp",
        flows: [{ flow: "input-proof", weight: 1 }],
        shape: {
          kind: "segments",
          segments: [
            { fromRps: 4, toRps: 4, durationSec: 30 },
            { fromRps: 24, toRps: 24, durationSec: 30 },
          ],
        },
      }),
    );
    expect(warnings).toHaveLength(1);
    expect(warnings[0]).toContain("input-proof peak ~24.0 rps");
  });

  it("uses maxRps for burst shapes and stays silent without it", () => {
    expect(
      ceilingWarnings(
        scenario({
          name: "burst-hot",
          flows: [{ flow: "input-proof", weight: 1 }],
          shape: { kind: "burst", count: 100, maxRps: 25 },
        }),
      ),
    ).toHaveLength(1);
    expect(
      ceilingWarnings(
        scenario({
          name: "burst-uncapped",
          flows: [{ flow: "input-proof", weight: 1 }],
          shape: { kind: "burst", count: 100 },
        }),
      ),
    ).toEqual([]);
  });

  it("never warns for closed shapes because the rate is an output", () => {
    expect(
      ceilingWarnings(
        scenario({
          name: "closed",
          flows: [{ flow: "user-decrypt", weight: 1 }],
          shape: { kind: "closed", vus: 100, durationSec: 60 },
        }),
      ),
    ).toEqual([]);
  });
});
