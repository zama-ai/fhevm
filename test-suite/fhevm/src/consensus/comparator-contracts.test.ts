import { expect, test } from "bun:test";
import { REQUIRED_COMPARATOR_CONTRACTS, verifyComparatorContracts } from "./comparator-contracts";
const report = () => ({ stats: { failures: 0 }, passes: REQUIRED_COMPARATOR_CONTRACTS.map(fullTitle => ({ fullTitle })) });
test("requires every named comparator and publication-safety contract", () => {
  expect(verifyComparatorContracts(report())).toBe(REQUIRED_COMPARATOR_CONTRACTS.length);
  for (const missing of REQUIRED_COMPARATOR_CONTRACTS) {
    const value = report();
    value.passes = value.passes.filter(test => test.fullTitle !== missing);
    expect(() => verifyComparatorContracts(value)).toThrow("did not pass exactly once");
  }
});
test("unrelated tests cannot substitute for a removed comparator suite", () => {
  const passes = Array.from({ length: 100 }, (_, i) => ({ fullTitle: `unrelated ${i}` }));
  expect(() => verifyComparatorContracts({ stats: { failures: 0 }, passes })).toThrow();
});
