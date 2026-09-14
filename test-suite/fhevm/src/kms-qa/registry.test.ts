import { describe, expect, test } from "bun:test";

import { QA_CASES, checkCaseRequirements, selectCases, type KmsTopology, type QaCase } from "./registry";

/** A minimal case stub; only the fields the pure selection logic reads are populated. */
const stubCase = (id: string): QaCase => ({
  id,
  title: `title for ${id}`,
  proves: `claim for ${id}`,
  requirements: {},
  mutatesLifecycle: false,
  run: async () => undefined,
});

const CASES = [stubCase("alpha"), stubCase("beta"), stubCase("gamma")];

/** The five-party-swap-threshold-kms topology: 4-node committee, t=1, one spare. */
const fiveParty: KmsTopology = {
  mode: "threshold",
  parties: 5,
  threshold: 1,
  committeeSize: 4,
  fheParams: "Test",
};

describe("kms-qa registry selectCases", () => {
  test("runs everything when the selector is unset, empty or 'all'", () => {
    for (const spec of [undefined, "", "   ", "all", "ALL"]) {
      expect(selectCases(CASES, spec).map((item) => item.id)).toEqual(["alpha", "beta", "gamma"]);
    }
  });

  test("selects the named subset", () => {
    expect(selectCases(CASES, "beta").map((item) => item.id)).toEqual(["beta"]);
  });

  test("tolerates whitespace and trailing separators", () => {
    expect(selectCases(CASES, " alpha , gamma ,").map((item) => item.id)).toEqual(["alpha", "gamma"]);
  });

  test("always returns registry order, never the order typed — these cases mutate shared state", () => {
    expect(selectCases(CASES, "gamma,alpha").map((item) => item.id)).toEqual(["alpha", "gamma"]);
  });

  test("deduplicates repeated ids", () => {
    expect(selectCases(CASES, "beta,beta").map((item) => item.id)).toEqual(["beta"]);
  });

  test("rejects an unknown id and lists what is available", () => {
    expect(() => selectCases(CASES, "alpha,nope")).toThrow(/unknown case id\(s\) nope/);
    expect(() => selectCases(CASES, "nope")).toThrow(/alpha \(title for alpha\)/);
  });
});

describe("kms-qa registry checkCaseRequirements", () => {
  test("accepts a topology that satisfies every requirement", () => {
    expect(
      checkCaseRequirements({ mode: "threshold", minParties: 5, minSpares: 1, minCommitteeSize: 4 }, fiveParty),
    ).toBeUndefined();
  });

  test("accepts empty requirements", () => {
    expect(checkCaseRequirements({}, fiveParty)).toBeUndefined();
  });

  test("names the mode mismatch", () => {
    expect(checkCaseRequirements({ mode: "threshold" }, { ...fiveParty, mode: "centralized" })).toMatch(
      /requires a threshold-mode KMS.*centralized/,
    );
  });

  test("reports the observed party count", () => {
    expect(checkCaseRequirements({ minParties: 5 }, { ...fiveParty, parties: 4 })).toMatch(
      /at least 5 KMS core\(s\).*has 4/,
    );
  });

  test("derives spare count from parties minus committeeSize and shows both", () => {
    expect(checkCaseRequirements({ minSpares: 1 }, { ...fiveParty, parties: 4 })).toMatch(
      /at least 1 spare core\(s\).*has 0.*parties=4, committeeSize=4/,
    );
  });

  test("reports the observed committee size", () => {
    expect(checkCaseRequirements({ minCommitteeSize: 4 }, { ...fiveParty, committeeSize: 3 })).toMatch(
      /committee of at least 4.*has 3/,
    );
  });
});

describe("kms-qa registry QA_CASES", () => {
  test("case ids are unique, since they are the selector keys", () => {
    expect(new Set(QA_CASES.map((item) => item.id)).size).toBe(QA_CASES.length);
  });

  test("every case documents what it proves, which the summary prints verbatim", () => {
    for (const item of QA_CASES) {
      expect(item.title.length).toBeGreaterThan(0);
      expect(item.proves.length).toBeGreaterThan(0);
    }
  });

  test("registers the epoch-rotation case", () => {
    expect(QA_CASES.map((item) => item.id)).toContain("epoch-rotation");
  });

  test("the epoch-rotation case requires a threshold KMS and declares itself disruptive", () => {
    const rotation = QA_CASES.find((item) => item.id === "epoch-rotation")!;
    expect(rotation.requirements.mode).toBe("threshold");
    expect(rotation.mutatesLifecycle).toBe(true);
  });
});
