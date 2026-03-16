import { Command } from "@effect/cli";
import { down } from "./down";

export const downCommand = Command.make("down", {}, () => down);
