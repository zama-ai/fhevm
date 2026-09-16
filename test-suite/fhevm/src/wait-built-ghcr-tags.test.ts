import { describe, expect, test } from "bun:test";

const {
  selectBaselineGhcrImages,
} = require("../../../ci/preview-env/scripts/wait-built-ghcr-tags.cjs") as {
  selectBaselineGhcrImages: (options: {
    lockEnv?: Record<string, string>;
    skipped?: Array<{ envKey: string; repo: string }>;
  }) => Array<{ repo: string; tag: string; envKey: string }>;
};

describe("orchestrated GHCR lock verification", () => {
  test("successful head builds plus a missing baseline still select the lock tag", () => {
    const baseline = selectBaselineGhcrImages({
      lockEnv: {
        COPROCESSOR_TFHE_WORKER_VERSION: "head222",
        GATEWAY_VERSION: "ancest0",
      },
      skipped: [
        { envKey: "GATEWAY_VERSION", repo: "fhevm/gateway-contracts" },
      ],
    });
    expect(baseline).toEqual([
      { repo: "fhevm/gateway-contracts", tag: "ancest0", envKey: "GATEWAY_VERSION" },
    ]);
  });

  test("omits optional unpublished baseline keys rather than inventing a tag", () => {
    expect(
      selectBaselineGhcrImages({
        lockEnv: { HOST_VERSION: "base111" },
        skipped: [
          { envKey: "COPROCESSOR_CONSENSUS_DETECTOR_VERSION", repo: "fhevm/coprocessor/consensus-detector" },
        ],
      }),
    ).toEqual([]);
  });
});
