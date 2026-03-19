import { Command } from "@effect/cli";

import { listScenarios } from "./scenario";

const scenarioListCommand = Command.make(
  "list",
  {},
  () => listScenarios(),
).pipe(
  Command.withDescription("List bundled coprocessor scenarios."),
);

export const scenarioCommand = Command.make("scenario").pipe(
  Command.withDescription("Inspect bundled coprocessor scenarios."),
  Command.withSubcommands([scenarioListCommand]),
);
