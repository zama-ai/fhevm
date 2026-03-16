import { Command } from "@effect/cli";
import { Effect } from "effect";
import { PreflightError } from "../errors";

export const doctorCommand = Command.make("doctor", {}, () =>
  Effect.fail(
    new PreflightError({
      message: "`doctor` was removed; use `fhevm-cli up --dry-run ...`",
    }),
  ),
);
