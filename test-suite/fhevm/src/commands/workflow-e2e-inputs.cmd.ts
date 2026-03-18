import { Command, Options } from "@effect/cli";

import { workflowE2eInputs } from "./workflow-e2e-inputs";

export const workflowE2eInputsCommand = Command.make(
  "workflow-e2e-inputs",
  {
    commit: Options.text("commit").pipe(
      Options.withDescription("Merge-candidate commit SHA whose built repo-owned images must be exercised."),
    ),
    needsFile: Options.text("needs-file").pipe(
      Options.withDescription("Path to the JSON-encoded GitHub workflow needs object."),
    ),
  },
  workflowE2eInputs,
).pipe(
  Command.withDescription("Resolve repo-owned image version overrides for the reusable e2e workflow and fail if a required build did not succeed."),
);
