import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { from, scenario, to, versionSources } from "./versions";

const testRollout = (ctx: RolloutRunContext) => ctx.test("rollout-standard", { parallel: false });
const testRequiredNode = (ctx: RolloutRunContext) =>
  ctx.test("user-decryption", { grep: "test user decrypt ebool$", parallel: false });

type KmsCoreVersions = Record<string, string> & { CORE_VERSION: string };

export type ProgressiveKmsCoreRunbook = {
  from: KmsCoreVersions;
  overrides?: Parameters<RolloutRunContext["up"]>[0]["overrides"];
  scenario: string;
  to: KmsCoreVersions;
  versionSources: string[];
};

export const runProgressiveKmsCoreRollout = async (
  ctx: RolloutRunContext,
  config: ProgressiveKmsCoreRunbook,
) => {
  const baselineLock = await ctx.writeVersionLock("00-kms-core-baseline", {
    versions: config.from,
    sources: config.versionSources,
  });
  const targetLock = await ctx.writeVersionLock("01-kms-core-target", {
    versions: config.to,
    sources: config.versionSources,
  });

  await ctx.up({ lockFile: baselineLock, overrides: config.overrides, scenario: config.scenario });
  await testRollout(ctx);

  const state = await ctx.readState();
  const nodeIds = Array.from({ length: state.scenario.kms.committeeSize }, (_, index) => index + 1);
  for (const nodeId of nodeIds) {
    await ctx.upgradeKmsNodes([nodeId], { lockFile: targetLock });
    await ctx.withRequiredKmsNode(nodeId, () => testRequiredNode(ctx));
    await testRollout(ctx);
  }
};

export const run = (ctx: RolloutRunContext) =>
  runProgressiveKmsCoreRollout(ctx, {
    from,
    scenario,
    to,
    versionSources,
  });

export default run;
