import { Command } from "@effect/cli";
import { scopeArg } from "../options";
import { unpause } from "./unpause";

export const unpauseCommand = Command.make(
  "unpause",
  { scope: scopeArg },
  ({ scope }) => unpause(scope),
).pipe(
  Command.withDescription("Unpause host or gateway contracts in the running stack."),
);
