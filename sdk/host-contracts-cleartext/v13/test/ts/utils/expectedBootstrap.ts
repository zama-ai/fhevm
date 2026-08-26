// The bootstrap values a default `deploy()` must produce — the vitest suite's single copy.
//
// Before this module existed the same triple
//
//   hcuLimit: { hcuCapPerBlock: 281474976710655n, maxHCUDepthPerTx: 5000000n, maxHCUPerTx: 20000000n }
//
// appeared verbatim in four test files here and three more in the previous generation. Seven copies of one
// decision, each needing to be found and hand-edited when a value moved, and each of which would have gone
// on passing by itself if only some of them were.
//
// ## Why these are literals rather than an import
//
// The obvious version of this file imports `CLEARTEXT_*_HCU_*` from `internal/cleartext-config.ts`. That
// does not compile, and the reason it does not is a guarantee worth more than the import:
// `test/ts/tsconfig.json` sets `rootDir: "."`, so nothing under `test/ts/` can reach the source tree
// (TS6059). That boundary is what stops a test in this suite from importing our SOURCE when the suite's
// whole purpose is to exercise the PUBLISHED package — `tarball-consumer.test.ts` would otherwise be able
// to pass while the published artifact was broken. Relaxing `rootDir` to save one copy would trade a real
// guarantee for a small convenience.
//
// The published package cannot supply them either: `pkg/ts/index.ts` does not export the cleartext config.
// Whether it should is an open decision — plans/CLEARTEXT_CONFIG_SOURCE_OF_TRUTH_PLAN.md item 3.
//
// So this file is a FACE of `sdk/cleartext-config.json` in exactly the sense RULES.md rule 23 means, like
// `internal/cleartext-config.ts` and `create2-deploy/script/FhevmCleartextConfig.sol`: a hand-written copy
// that is safe because it is *checked*. `test/cleartext-config-mirror.test.ts` compares the three values
// below against the source of truth, so this is one verified copy in place of seven unverified ones.
//
// Keep the constant NAMES in the comments beside each value: that is what makes the check able to find
// them, and what makes a `grep CLEARTEXT_MAX_HCU_PER_TX` reach this file.

/**
 * What `HCULimit` holds after a default deploy.
 *
 * A function rather than a shared object: every caller compares it against a freshly read value, and a
 * shared mutable literal is the kind of thing one test can quietly change for the next. The cost is an
 * allocation, in a suite that starts an anvil node.
 */
export function expectedHcuLimit(): {
  hcuCapPerBlock: bigint;
  maxHCUDepthPerTx: bigint;
  maxHCUPerTx: bigint;
} {
  return {
    hcuCapPerBlock: 281474976710655n, // CLEARTEXT_HCU_CAP_PER_BLOCK
    maxHCUDepthPerTx: 5000000n, // CLEARTEXT_MAX_HCU_DEPTH_PER_TX
    maxHCUPerTx: 20000000n, // CLEARTEXT_MAX_HCU_PER_TX
  };
}
