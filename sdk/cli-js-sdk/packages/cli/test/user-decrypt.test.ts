import { Command } from "@commander-js/extra-typings";
import { expect, test } from "vitest";

import { registerUserDecryptCommands } from "../src/cli/commands/user-decrypt";

/** Finds a direct child command by name. */
const child = (parent: Command, name: string): Command => {
  const found = parent.commands.find((command) => command.name() === name);
  if (found === undefined) {
    throw new Error(`expected subcommand "${name}"`);
  }
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

  expect(captured?.artifact).toBe("out.json");
});

test("user-decrypt parent is a pure dispatcher over direct/stored/fresh", () => {
  const program = new Command();
  registerUserDecryptCommands(program);

  const userDecrypt = child(program, "user-decrypt");

  // The parent declares no options of its own, so none can shadow subcommand
  // flags such as --artifact.
  expect(userDecrypt.options.map((option) => option.long)).toEqual([]);
  expect(userDecrypt.commands.map((command) => command.name()).sort()).toEqual([
    "direct",
    "fresh",
    "stored",
  ]);
});
