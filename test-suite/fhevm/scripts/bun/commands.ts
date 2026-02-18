import { createAdminHandlers } from "./command-admin";
import { createCleanHandlers } from "./command-clean";
import { createTestHandlers } from "./command-test";
import { createTraceHandlers } from "./command-trace";
import type { CommandDeps } from "./command-contracts";

export function createCommandHandlers(deps: CommandDeps) {
  return {
    ...createTraceHandlers(deps),
    ...createTestHandlers(deps),
    ...createAdminHandlers(deps),
    ...createCleanHandlers(deps),
  };
}
