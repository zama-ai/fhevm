import { Command } from "@effect/cli";
import { Option } from "effect";
import { scopeArg } from "../options";
import { pause } from "./pause";

export const pauseCommand = Command.make(
  "pause",
  { scope: scopeArg },
  ({ scope }) => pause(Option.getOrUndefined(scope)),
);
