import { Command } from "@effect/cli";
import { scopeArg } from "../options";
import { pause } from "./pause";

export const pauseCommand = Command.make(
  "pause",
  { scope: scopeArg },
  ({ scope }) => pause(scope),
).pipe(
  Command.withDescription("Pause host or gateway contracts in the running stack."),
);
