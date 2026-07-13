import type { CommandUnknownOpts } from "@commander-js/extra-typings";

import { registerScenarioRunCommand } from "./scenarios";

/** Backward-compatible root alias for the canonical `scenario run` action. */
export const registerRunCommand = (program: CommandUnknownOpts): void => {
  registerScenarioRunCommand(program, "Alias for `scenario run`");
};
