import { describe, expect, it } from "bun:test";
import {
  STEPS,
  STEP_IDS,
  filterSteps,
  stepsToCleanup,
  validateDag,
  getStepIndex,
  getStep,
  type StepId,
} from "../dag.js";

describe("DAG structure", () => {
  it("has exactly 13 steps", () => {
    expect(STEPS).toHaveLength(13);
  });

  it("has 13 unique step IDs", () => {
    const ids = new Set(STEP_IDS);
    expect(ids.size).toBe(13);
  });

  it("step IDs match the expected list from the bash script", () => {
    const expected: StepId[] = [
      "minio", "core", "kms-signer", "database", "host-node", "gateway-node",
      "coprocessor", "kms-connector", "gateway-mocked-payment", "gateway-sc",
      "host-sc", "relayer", "test-suite",
    ];
    expect(STEP_IDS).toEqual(expected);
  });

  it("kms-signer is the only step with no compose file", () => {
    const noCompose = STEPS.filter((s) => s.compose === null);
    expect(noCompose).toHaveLength(1);
    expect(noCompose[0].id).toBe("kms-signer");
  });

  it("every dependsOn reference is a valid step ID", () => {
    const ids = new Set(STEP_IDS);
    for (const step of STEPS) {
      for (const dep of step.dependsOn) {
        expect(ids.has(dep)).toBe(true);
      }
    }
  });

  it("no step depends on itself", () => {
    for (const step of STEPS) {
      expect(step.dependsOn).not.toContain(step.id);
    }
  });

  it("all dependsOn edges point backward (no forward references)", () => {
    for (let i = 0; i < STEPS.length; i++) {
      for (const dep of STEPS[i].dependsOn) {
        const depIdx = getStepIndex(dep as StepId);
        expect(depIdx).toBeLessThan(i);
      }
    }
  });

  it("validateDag does not throw on the real DAG", () => {
    expect(() => validateDag()).not.toThrow();
  });
});

describe("getStepIndex", () => {
  it("returns 0 for minio", () => {
    expect(getStepIndex("minio")).toBe(0);
  });

  it("returns 12 for test-suite", () => {
    expect(getStepIndex("test-suite")).toBe(12);
  });
});

describe("getStep", () => {
  it("returns the correct step by ID", () => {
    const step = getStep("coprocessor");
    expect(step).toBeDefined();
    expect(step!.label).toBe("Coprocessor Services");
    expect(step!.services.length).toBeGreaterThan(0);
  });

  it("returns undefined for unknown ID", () => {
    expect(getStep("nonexistent" as StepId)).toBeUndefined();
  });
});

describe("filterSteps", () => {
  it("returns all 13 steps with no flags", () => {
    const steps = filterSteps({});
    expect(steps).toHaveLength(13);
    expect(steps[0].id).toBe("minio");
    expect(steps[12].id).toBe("test-suite");
  });

  it("--only returns exactly one step", () => {
    for (const id of STEP_IDS) {
      const steps = filterSteps({ only: id });
      expect(steps).toHaveLength(1);
      expect(steps[0].id).toBe(id);
    }
  });

  it("--resume from minio returns all steps", () => {
    const steps = filterSteps({ resume: "minio" });
    expect(steps).toHaveLength(13);
  });

  it("--resume from test-suite returns only test-suite", () => {
    const steps = filterSteps({ resume: "test-suite" });
    expect(steps).toHaveLength(1);
    expect(steps[0].id).toBe("test-suite");
  });

  it("--resume from coprocessor returns coprocessor onward", () => {
    const steps = filterSteps({ resume: "coprocessor" });
    const ids = steps.map((s) => s.id);
    expect(ids[0]).toBe("coprocessor");
    expect(ids).toContain("kms-connector");
    expect(ids).toContain("test-suite");
    expect(ids).not.toContain("minio");
    expect(ids).not.toContain("database");
  });

  it("--resume returns a contiguous suffix of the full list", () => {
    for (const id of STEP_IDS) {
      const steps = filterSteps({ resume: id });
      const idx = getStepIndex(id);
      expect(steps).toHaveLength(13 - idx);
      for (let i = 0; i < steps.length; i++) {
        expect(steps[i].id).toBe(STEPS[idx + i].id);
      }
    }
  });
});

describe("stepsToCleanup", () => {
  it("returns steps in reverse order", () => {
    const cleanup = stepsToCleanup({});
    // Should be reversed and only include steps with compose files
    const withCompose = STEPS.filter((s) => s.compose !== null);
    expect(cleanup).toHaveLength(withCompose.length);
    expect(cleanup[0].id).toBe("test-suite");
    expect(cleanup[cleanup.length - 1].id).toBe("minio");
  });

  it("excludes steps without compose files (kms-signer)", () => {
    const cleanup = stepsToCleanup({});
    const ids = cleanup.map((s) => s.id);
    expect(ids).not.toContain("kms-signer");
  });

  it("--only returns at most one step (reversed)", () => {
    const cleanup = stepsToCleanup({ only: "coprocessor" });
    expect(cleanup).toHaveLength(1);
    expect(cleanup[0].id).toBe("coprocessor");
  });

  it("--only kms-signer returns empty (no compose file)", () => {
    const cleanup = stepsToCleanup({ only: "kms-signer" });
    expect(cleanup).toHaveLength(0);
  });

  it("--resume cleanup and execution sets cover all steps", () => {
    for (const id of STEP_IDS) {
      const execSteps = filterSteps({ resume: id });
      const cleanupSteps = stepsToCleanup({ resume: id });

      // Cleanup is a reversed subset of exec (only those with compose)
      const cleanupIds = new Set(cleanupSteps.map((s) => s.id));
      const execIds = new Set(execSteps.map((s) => s.id));

      for (const cid of cleanupIds) {
        expect(execIds.has(cid)).toBe(true);
      }
    }
  });
});

describe("property: disjoint partition for --resume", () => {
  it("skipped + executed = all steps for every resume point", () => {
    for (const id of STEP_IDS) {
      const idx = getStepIndex(id);
      const executed = filterSteps({ resume: id });
      const skipped = STEPS.slice(0, idx);

      expect(skipped.length + executed.length).toBe(13);

      const allIds = [...skipped.map((s) => s.id), ...executed.map((s) => s.id)];
      expect(new Set(allIds).size).toBe(13);
    }
  });
});
