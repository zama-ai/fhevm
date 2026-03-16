import { Command } from "@effect/cli";
import { status } from "./status";

export const statusCommand = Command.make("status", {}, () => status);
