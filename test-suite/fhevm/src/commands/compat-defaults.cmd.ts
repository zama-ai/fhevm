import { Command } from "@effect/cli";
import { compatDefaults } from "./compat-defaults";

export const compatDefaultsCommand = Command.make(
  "compat-defaults",
  {},
  () => compatDefaults,
);
