import { Command } from "@effect/cli";
import { groupArg } from "../options";
import { upgrade } from "./upgrade";

export const upgradeCommand = Command.make(
  "upgrade",
  { group: groupArg },
  ({ group }) => upgrade(group),
).pipe(
  Command.withDescription("Rebuild and restart an active local override in-place."),
);
