import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

import { Command } from "@commander-js/extra-typings";

import { registerVerifyUserDecryptCommand } from "../src/cli/commands/verify-user-decrypt";

test("verify-user-decrypt is a top-level command with --artifact/--job-id/--url", () => {
  const program = new Command();
  registerVerifyUserDecryptCommand(program);

  const command = program.commands.find(
    (candidate) => candidate.name() === "verify-user-decrypt",
  );
  assert.ok(command, "expected top-level verify-user-decrypt command");

  assert.deepEqual(
    command.options.map((option) => option.long).sort(),
    ["--artifact", "--job-id", "--url"],
  );
  const artifactOption = command.options.find(
    (option) => option.long === "--artifact",
  );
  assert.equal(artifactOption?.mandatory, true);
});

test("the removed relayer-result command module no longer exists", () => {
  const modulePath = fileURLToPath(
    new URL("../src/cli/commands/relayer-result.ts", import.meta.url),
  );
  assert.equal(existsSync(modulePath), false);
});
