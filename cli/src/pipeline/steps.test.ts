import { describe, expect, test } from "bun:test";

import {
  BOOT_STEPS,
  getParallelGroup,
  getServicesForStep,
  getStepByName,
  getStepByNumber,
  getStepsTeardownOrder,
  resolveStepRef,
} from "./steps";

describe("boot steps", () => {
  test("defines all 13 steps in order", () => {
    expect(BOOT_STEPS).toHaveLength(13);
    expect(BOOT_STEPS.map((step) => step.number)).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
  });

  test("resolves step by name and number", () => {
    expect(getStepByName("minio")?.number).toBe(1);
    expect(getStepByNumber(7)?.name).toBe("gateway-mocked-payment");
    expect(resolveStepRef("8").name).toBe("gateway-contracts");
    expect(resolveStepRef("gateway-contracts").number).toBe(8);
  });

  test("throws for invalid step references", () => {
    expect(() => resolveStepRef("unknown-step")).toThrow("unknown step reference");
    expect(() => resolveStepRef("")).toThrow("step reference is empty");
  });

  test("finds parallel groups", () => {
    const step5 = BOOT_STEPS.findIndex((step) => step.number === 5);
    const step10 = BOOT_STEPS.findIndex((step) => step.number === 10);

    expect(getParallelGroup(BOOT_STEPS, step5).map((step) => step.number)).toEqual([5, 6]);
    // Steps 10 and 11 are sequential (no parallelGroup) since kms-connector
    // must complete FHE key discovery before coprocessor can start.
    expect(getParallelGroup(BOOT_STEPS, step10).map((step) => step.number)).toEqual([10]);
  });

  test("returns teardown order from target step", () => {
    expect(getStepsTeardownOrder(7).map((step) => step.number)).toEqual([13, 12, 11, 10, 9, 8, 7]);
  });

  test("all step services resolve", () => {
    for (const step of BOOT_STEPS) {
      expect(() => getServicesForStep(step)).not.toThrow();
    }
  });
});
