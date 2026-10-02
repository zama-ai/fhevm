import { expect, test } from "bun:test";
import { mkdtemp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";

import { archiveRolloutSolidity, resolveRolloutSolidityRef } from "./commands/rollout-solidity";
import { run } from "./utils/process";

test("rollout fixtures use the resolved baseline even after its tag and checkout move", async () => {
  const root = await mkdtemp(path.join(tmpdir(), "rollout-solidity-test-"));
  try {
    const git = (...args: string[]) => run(["git", ...args], { cwd: root });
    await git("init", "-q");
    await git("config", "user.name", "Fixture Test");
    await git("config", "user.email", "fixture@example.invalid");
    await mkdir(path.join(root, "library-solidity/lib"), { recursive: true });
    const source = path.join(root, "library-solidity/lib/FHE.sol");
    await writeFile(source, "baseline ABI\n");
    await writeFile(path.join(root, "outside-library"), "must not be packaged\n");
    await git("add", ".");
    await git("commit", "-qm", "baseline");
    await git("tag", "baseline");
    const revision = await resolveRolloutSolidityRef("baseline", root);
    await writeFile(source, "new executor ABI\n");
    await git("commit", "-qam", "new ABI");
    await git("tag", "-f", "baseline");
    const snapshot = await archiveRolloutSolidity(revision, root, root);
    const names = (await run(["tar", "-tf", snapshot.archive])).stdout;
    expect(names).not.toContain("outside-library");
    const extracted = path.join(root, "extracted");
    await mkdir(extracted);
    await run(["tar", "-xf", snapshot.archive, "-C", extracted]);
    expect(await readFile(path.join(extracted, "library-solidity/lib/FHE.sol"), "utf8")).toBe("baseline ABI\n");
    expect(await readFile(source, "utf8")).toBe("new executor ABI\n");
    expect(snapshot.revision).toBe(revision);
    expect(snapshot.sha256).toMatch(/^[0-9a-f]{64}$/);
    await expect(archiveRolloutSolidity("baseline", root, root)).rejects.toThrow("resolved commit SHA");
    await expect(resolveRolloutSolidityRef("missing-release", root)).rejects.toThrow();
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});
