import { Command, Options } from "@effect/cli";

import { workflowE2eInputs } from "./workflow-e2e-inputs";

export const workflowE2eInputsCommand = Command.make(
  "workflow-e2e-inputs",
  {
    previousCommit: Options.text("previous-commit").pipe(
      Options.withDescription("Base commit SHA used when a component build did not produce a PR image."),
    ),
    newCommit: Options.text("new-commit").pipe(
      Options.withDescription("Head commit SHA used when a component build produced a PR image."),
    ),
    needsFile: Options.text("needs-file").pipe(
      Options.withDescription("Path to the JSON-encoded GitHub workflow needs object."),
    ),
  },
  workflowE2eInputs,
).pipe(
  Command.withDescription("Resolve repo-owned image version overrides for the reusable e2e workflow."),
);
