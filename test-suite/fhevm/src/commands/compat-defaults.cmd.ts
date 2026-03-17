import { Command } from "@effect/cli";
import { compatDefaults } from "./compat-defaults";

export const compatDefaultsCommand = Command.make(
  "compat-defaults",
  {},
  () => compatDefaults,
).pipe(
  Command.withDescription("Print CI-facing compatibility defaults and anchor commits as JSON."),
);
