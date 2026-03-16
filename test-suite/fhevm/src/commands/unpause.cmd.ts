import { Command } from "@effect/cli";
import { Option } from "effect";
import { scopeArg } from "../options";
import { unpause } from "./unpause";

export const unpauseCommand = Command.make(
  "unpause",
  { scope: scopeArg },
  ({ scope }) => unpause(Option.getOrUndefined(scope)),
);
