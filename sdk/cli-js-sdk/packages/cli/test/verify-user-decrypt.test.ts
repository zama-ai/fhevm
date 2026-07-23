import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { Command } from "@commander-js/extra-typings";
import { expect, test } from "vitest";

import { registerVerifyUserDecryptCommand } from "../src/cli/commands/verify-user-decrypt";

test("verify-user-decrypt is a top-level command with --artifact/--job-id/--url", () => {
  const program = new Command();
  registerVerifyUserDecryptCommand(program);

  const command = program.commands.find(
    (candidate) => candidate.name() === "verify-user-decrypt",
  );
  if (command === undefined) {
    throw new Error("expected top-level verify-user-decrypt command");
  }

  expect(command.options.map((option) => option.long).sort()).toEqual([
    "--artifact",
    "--job-id",
    "--url",
  ]);
  const artifactOption = command.options.find(
    (option) => option.long === "--artifact",
  );
  expect(artifactOption?.mandatory).toBe(true);
});

test("the removed relayer-result command module no longer exists", () => {
  const modulePath = fileURLToPath(
    new URL("../src/cli/commands/relayer-result.ts", import.meta.url),
  );
  expect(existsSync(modulePath)).toBe(false);
});
