import { Command, Options } from "@effect/cli";

import { workflowE2eInputs } from "./workflow-e2e-inputs";

export const workflowE2eInputsCommand = Command.make(
  "workflow-e2e-inputs",
  {
    commit: Options.text("commit").pipe(
      Options.withDescription("PR head commit SHA used when a repo-owned image build succeeded."),
    ),
    previousCommit: Options.text("previous-commit").pipe(
      Options.withDescription("Base commit SHA used when a repo-owned image build was skipped or failed."),
    ),
    needsFile: Options.text("needs-file").pipe(
      Options.withDescription("Path to the JSON-encoded GitHub workflow needs object."),
    ),
  },
  workflowE2eInputs,
).pipe(
  Command.withDescription("Resolve repo-owned image version overrides for the reusable e2e workflow from PR-head-or-base build results."),
);
