import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { from, scenario, to, versionSources } from "./versions";

const testRollout = (ctx: RolloutRunContext) => ctx.test("rollout-standard", { parallel: false });
const testRequiredNode = (ctx: RolloutRunContext) =>
  ctx.test("user-decryption", { grep: "test user decrypt ebool$", parallel: false });

export const run = async (ctx: RolloutRunContext) => {
  const baselineLock = await ctx.resolveVersionLock("00-kms-core-baseline", {
    versions: from,
    sources: versionSources,
  });
  const targetLock = await ctx.resolveVersionLock("01-kms-core-target", {
    versions: to,
    sources: versionSources,
  });

  // Build the e2e image from the working tree: this runbook exercises current test
  // code against released KMS cores, so the published test-suite image for either
  // core tag would be the wrong harness.
  await ctx.up({ lockFile: baselineLock, overrides: [{ group: "test-suite" }], scenario });
  await testRollout(ctx);

  const state = await ctx.readState();
  const nodeIds = Array.from({ length: state.scenario.kms.committeeSize }, (_, index) => index + 1);
  for (const nodeId of nodeIds) {
    await ctx.upgradeKmsNodes([nodeId], { lockFile: targetLock });
    await ctx.withRequiredKmsNode(nodeId, () => testRequiredNode(ctx));
    await testRollout(ctx);
  }
};

export default run;
