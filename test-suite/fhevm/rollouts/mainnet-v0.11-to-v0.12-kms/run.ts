import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { coprocessorSenderSchemaBridge } from "./coprocessor-schema-bridge";
import { from, scenario, to, versionSources } from "./versions";

export const relayerSdkV042TestKeyError = "Cannot find user decryption pivot";

export const run = async (ctx: RolloutRunContext) => {
  const baselineLock = await ctx.writeVersionLock("00-kms-core-baseline", { versions: from, sources: versionSources });
  const targetLock = await ctx.writeVersionLock("01-kms-core-target", { versions: to, sources: versionSources });

  await ctx.up({ lockFile: baselineLock, overrides: [{ group: "test-suite" }], scenario });
  await ctx.runCoprocessorSql(
    "bridge v0.11 coprocessor rows to the v0.12 transaction sender",
    coprocessorSenderSchemaBridge,
  );
  // Threshold test parameters exercise every real node and response, but the
  // pinned legacy client cannot reconstruct them. Plaintext fidelity belongs
  // to the centralized realistic-key path; this path checks response compatibility.
  await ctx.test("input-proof", { parallel: false });
  await ctx.checkUserDecryptionResponses("old KMS nodes return v0 responses", {
    versionsByNode: ["v0", "v0", "v0", "v0"],
    expectedClientError: relayerSdkV042TestKeyError,
  });

  await ctx.upgradeKmsNodes([1, 2], { lockFile: targetLock });
  await ctx.test("input-proof", { parallel: false });
  await ctx.checkUserDecryptionResponses("mixed KMS nodes return both response versions", {
    versionsByNode: ["v1", "v1", "v0", "v0"],
    expectedClientError: relayerSdkV042TestKeyError,
  });

  await ctx.upgradeKmsNodes([3, 4], { lockFile: targetLock });
  await ctx.test("input-proof", { parallel: false });
  await ctx.checkUserDecryptionResponses("new KMS nodes return v1 responses", {
    versionsByNode: ["v1", "v1", "v1", "v1"],
    expectedClientError: relayerSdkV042TestKeyError,
  });
};

export default run;
