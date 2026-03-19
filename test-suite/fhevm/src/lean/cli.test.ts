import path from "node:path";
import { describe, expect, test } from "bun:test";

const CLI_DIR = path.resolve(import.meta.dir, "..", "..");

const shellEscape = (value: string) => `'${value.replaceAll("'", `'\\''`)}'`;

const execCli = async (args: string[]) => {
  const proc = Bun.spawn(["zsh", "-lc", `bun run src/cli.ts ${args.map(shellEscape).join(" ")}`], {
    cwd: CLI_DIR,
    stdout: "pipe",
    stderr: "pipe",
    env: process.env,
  });
  const [stdout, stderr, code] = await Promise.all([
    new Response(proc.stdout).text(),
    new Response(proc.stderr).text(),
    proc.exited,
  ]);
  return { stdout, stderr, code };
};

describe("cli", () => {
  test("lists bundled scenarios", async () => {
    const result = await execCli(["scenario", "list"]);
    expect(result.code).toBe(0);
    expect(result.stdout).toContain("two-of-two");
  });

  test("rejects unsupported targets", async () => {
    const result = await execCli(["up", "--target", "bogus"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("Unsupported target bogus");
  });

  test("requires --sha for sha target", async () => {
    const result = await execCli(["up", "--target", "sha"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("--target sha requires --sha");
  });

  test("rejects scenario with coprocessor override", async () => {
    const result = await execCli([
      "up",
      "--scenario",
      "two-of-two",
      "--override",
      "coprocessor",
      "--dry-run",
    ]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("--scenario cannot be combined with --override coprocessor");
  });

  test("validates pause scope", async () => {
    const result = await execCli(["pause", "nope"]);
    expect(result.code).toBe(1);
    expect(result.stderr).toContain("pause expects `host` or `gateway`");
  });
});
