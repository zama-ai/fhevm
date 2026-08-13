import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { migrateActiveStack } from "../v0.15-to-v0.15.1-gpu-key-migration/run";

export default async function runMigration(ctx: RolloutRunContext) {
  await migrateActiveStack(ctx, "standalone");
}
