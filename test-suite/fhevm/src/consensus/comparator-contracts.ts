/** Named executed contracts, not a floor satisfiable by unrelated test files. */
export const REQUIRED_COMPARATOR_CONTRACTS = [
  "consensus comparator accepts a fleet that agrees on everything",
  "consensus comparator detects a raw byte mismatch",
  "consensus comparator detects a compute digest mismatch",
  "consensus comparator detects an SNS digest mismatch",
  "consensus comparator treats a missing SNS digest as missing evidence rather than as agreement",
  "consensus comparator treats a missing compute digest as missing evidence",
  "consensus comparator detects a wrong key identity",
  "consensus comparator detects a different producing operation",
  "consensus comparator detects a provenance mismatch",
  "consensus comparator detects a type or version mismatch",
  "consensus comparator names a squash-backend split distinctly from a bad squash",
  "consensus comparator accepts an aliased handle whose producing transactions arrive in a different order",
  "consensus comparator compares fewer fields only when the caller says so, and still compares bytes",
  "consensus comparator refuses to call a single operator agreement",
  "consensus comparator evidence folding folds several producing transactions into one normalized provenance set",
  "consensus comparator evidence folding rejects one operator holding two different values for one handle",
  "consensus comparator evidence folding asserts storage-row uniqueness separately from the computation join",
  "consensus comparator evidence folding rejects an operator whose own rows disagree on the digest",
  "consensus comparator evidence folding rejects an operation mismatch on any alias producer regardless of row order",
  "consensus comparator evidence folding reports no rows as missing evidence, so a wait can keep waiting",
  "canary publication safety waits for the selected operator to publish, releasing the lock between polls",
  "canary publication safety refuses to poison an unpublished victim even when other operators could have quorum",
  "canary publication safety rolls back and releases the lock on a failed poison write",
  "canary publication safety restores with a fresh connection when the COMMIT result is uncertain",
  "canary publication safety journals a detector original under its row lock before poisoning the pending submission",
  "canary publication safety refuses a detector mutation when the sender already published",
  "raw-byte canary recovery requires cross-operator rejection while the local digest stays valid",
  "raw-byte canary recovery does not accept a local digest failure or a disabled comparator",
  "raw-byte canary recovery restores both values after a lost mutation commit reply",
  "raw-byte canary recovery replays the durable raw-byte journal after interrupted comparison",
  "raw-byte canary recovery refuses an unpublished operator without changing either value"
] as const;

export function verifyComparatorContracts(value: unknown): number {
  const report = value as { stats?: { failures?: number }; passes?: { fullTitle?: string }[] };
  if (!report || report.stats?.failures !== 0 || !Array.isArray(report.passes)) throw new Error("invalid or failing Mocha report");
  const titles = report.passes.map(test => {
    if (!test || typeof test.fullTitle !== "string") throw new Error("invalid Mocha pass entry");
    return test.fullTitle;
  });
  for (const title of REQUIRED_COMPARATOR_CONTRACTS) {
    if (titles.filter(actual => actual === title).length !== 1) throw new Error(`required comparator contract did not pass exactly once: ${title}`);
  }
  return titles.length;
}
