import { Command } from "@commander-js/extra-typings";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

vi.mock("../src/shared/logger", () => ({
  logger: { error: vi.fn(), info: vi.fn(), success: vi.fn(), warn: vi.fn() },
}));

import { registerBaselineCommands } from "../src/cli/commands/baseline";
import { logger } from "../src/shared/logger";

let dir: string;

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-baseline-list-"));
  vi.mocked(logger.info).mockReset();
  vi.mocked(logger.error).mockReset();
  vi.mocked(logger.success).mockReset();
  vi.mocked(logger.warn).mockReset();
});

afterEach(async () => {
  await rm(dir, { recursive: true, force: true });
});

const program = (): Command => {
  const command = new Command();
  registerBaselineCommands(command as unknown as Parameters<typeof registerBaselineCommands>[0]);
  return command;
};

describe("baseline list", () => {
  it("reports no baselines when the directory does not exist", async () => {
    await program().parseAsync([
      "node", "load-test", "baseline", "list",
      "--baselines-dir", join(dir, "missing"),
    ]);
    expect(vi.mocked(logger.info).mock.calls.flat()).toEqual([
      expect.stringContaining("No baselines found"),
    ]);
  });

  it("lists every stored baseline with its network, label, and updated time", async () => {
    const baselinesDir = join(dir, "baselines");
    await mkdir(join(baselinesDir, "testnet"), { recursive: true });
    await mkdir(join(baselinesDir, "mainnet"), { recursive: true });
    await writeFile(join(baselinesDir, "testnet", "open-steady.json"), "{}");
    await writeFile(join(baselinesDir, "testnet", "smoke.json"), "{}");
    await writeFile(join(baselinesDir, "mainnet", "open-steady.json"), "{}");

    await program().parseAsync([
      "node", "load-test", "baseline", "list", "--baselines-dir", baselinesDir,
    ]);

    const lines = vi.mocked(logger.info).mock.calls.flat();
    expect(lines).toHaveLength(3);
    expect(lines.some((line) => line.startsWith("mainnet/open-steady — updated"))).toBe(true);
    expect(lines.some((line) => line.startsWith("testnet/open-steady — updated"))).toBe(true);
    expect(lines.some((line) => line.startsWith("testnet/smoke — updated"))).toBe(true);
  });
});
