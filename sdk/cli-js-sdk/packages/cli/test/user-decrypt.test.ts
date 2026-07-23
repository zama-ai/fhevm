import assert from "node:assert/strict";
import { test } from "node:test";

import { Command } from "@commander-js/extra-typings";

import { registerUserDecryptCommands } from "../src/cli/commands/user-decrypt";

/** Finds a direct child command by name. */
const child = (parent: Command, name: string): Command => {
  const found = parent.commands.find((command) => command.name() === name);
  assert.ok(found, `expected subcommand "${name}"`);
  return found as unknown as Command;
};

test("stored subcommand receives --artifact now that no parent shadows it", async () => {
  const program = new Command();
  registerUserDecryptCommands(program);

  const userDecrypt = child(program, "user-decrypt");
  const stored = child(userDecrypt, "stored");

  // Replace the real action (which does dynamic imports and network calls) with
  // a capture so parsing exercises the real registration without side effects.
  let captured: Record<string, unknown> | undefined;
  stored.action(() => {
    captured = stored.opts();
  });

  await program.parseAsync(
    ["user-decrypt", "stored", "--artifact", "out.json"],
    { from: "user" },
  );

  assert.equal(captured?.artifact, "out.json");
});

test("user-decrypt parent is a pure dispatcher over direct/stored/fresh", () => {
  const program = new Command();
  registerUserDecryptCommands(program);

  const userDecrypt = child(program, "user-decrypt");

  // The parent declares no options of its own, so none can shadow subcommand
  // flags such as --artifact.
  assert.deepEqual(
    userDecrypt.options.map((option) => option.long),
    [],
  );
  assert.deepEqual(
    userDecrypt.commands.map((command) => command.name()).sort(),
    ["direct", "fresh", "stored"],
  );
});
