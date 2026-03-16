import { Command } from "@effect/cli";
import { Option } from "effect";
import { groupArg } from "../options";
import { upgrade } from "./upgrade";

export const upgradeCommand = Command.make(
  "upgrade",
  { group: groupArg },
  ({ group }) => upgrade(Option.getOrUndefined(group)),
);
