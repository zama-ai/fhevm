import { Command, Options } from "@effect/cli";
import { Option } from "effect";

import { workflowUpArgs } from "./workflow-up-args";

export const workflowUpArgsCommand = Command.make(
  "workflow-up-args",
  {
    imageMode: Options.text("image-mode").pipe(
      Options.withDescription("Workflow image mode: registry or workspace."),
      Options.withDefault("registry"),
    ),
    override: Options.text("override").pipe(
      Options.withDescription("Comma-separated workflow override string for workspace mode."),
      Options.optional,
    ),
    scenarioOut: Options.text("scenario-out").pipe(
      Options.withDescription("Path to write a temporary synthesized scenario when workspace coprocessor overrides are used."),
      Options.optional,
    ),
  },
  ({ imageMode, override, scenarioOut }) =>
    workflowUpArgs({
      imageMode,
      override: Option.getOrUndefined(override),
      scenarioOut: Option.getOrUndefined(scenarioOut),
    }),
).pipe(
  Command.withDescription("Resolve CI-facing fhevm-cli up args from workflow image-mode and override inputs."),
);
