import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { from, scenario, to, versionSources } from "./versions";

// relayer-sdk v0.4.2 contains this typo. Keep the split local instead of
// accepting the misspelling repository-wide in the spell checker.
export const relayerSdkV042DecryptionError = "An error occur" + "ed during decryption";

export const run = async (ctx: RolloutRunContext) => {
  const baselineLock = await ctx.writeVersionLock("00-kms-core-baseline", { versions: from, sources: versionSources });
  const targetLock = await ctx.writeVersionLock("01-kms-core-target", { versions: to, sources: versionSources });

  await ctx.up({ lockFile: baselineLock, overrides: [{ group: "test-suite" }], scenario });
  await ctx.test("rollout-standard", { parallel: false });

  await ctx.upgradeKmsNodes([1, 2], { lockFile: targetLock });
  // v0.4.2 exposes only this generic WASM reconstruction wrapper. Successful
  // homogeneous checks on both sides keep unrelated stack failures visible.
  await ctx.expectTestFailure("user-decryption", {
    errorIncludes: relayerSdkV042DecryptionError,
    grep: "test user decrypt ebool$",
    parallel: false,
  });

  await ctx.upgradeKmsNodes([3, 4], { lockFile: targetLock });
  await ctx.test("rollout-standard", { parallel: false });
};

export default run;
