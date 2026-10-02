import { createHash } from "node:crypto";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";

import { REPO_ROOT, TEST_SUITE_CONTAINER } from "../layout";
import { run, runWithHeartbeat } from "../utils/process";

/** Resolve once before boot: a movable tag must not change the fixture library mid-rollout. */
export async function resolveRolloutSolidityRef(ref: string, repository = REPO_ROOT): Promise<string> {
  const revision = (await run(["git", "rev-parse", "--verify", "--end-of-options", `${ref}^{commit}`],
    { cwd: repository })).stdout.trim();
  if (!/^[0-9a-f]{40}$/.test(revision)) throw new Error("Invalid rollout Solidity revision");
  await run(["git", "cat-file", "-e", `${revision}:library-solidity/lib/FHE.sol`], { cwd: repository });
  return revision;
}

export async function archiveRolloutSolidity(revision: string, directory: string, repository = REPO_ROOT) {
  if (!/^[0-9a-f]{40}$/.test(revision)) throw new Error("Rollout Solidity requires a resolved commit SHA");
  const archive = path.join(directory, "solidity.tar");
  const tree = (await run(["git", "rev-parse", `${revision}:library-solidity/lib`], { cwd: repository })).stdout.trim();
  await run(["git", "archive", "--format=tar", `--output=${archive}`, revision, "library-solidity/lib"], { cwd: repository });
  const sha256 = createHash("sha256").update(await Bun.file(archive).bytes()).digest("hex");
  return { archive, revision, tree, sha256 };
}

/** Only the test container changes; production contracts and the checkout remain untouched. */
export async function prepareRolloutSolidity(revision: string) {
  const directory = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-solidity-"));
  try {
    const snapshot = await archiveRolloutSolidity(revision, directory);
    // docker cp creates a root-owned file: use the application-owned directory,
    // where the unprivileged test user can unlink it (unlike sticky /tmp).
    const remote = "/app/library-solidity/.rollout-solidity.tar";
    await run(["docker", "cp", snapshot.archive, `${TEST_SUITE_CONTAINER}:${remote}`]);
    await run(["docker", "exec", TEST_SUITE_CONTAINER, "sh", "-ec",
      'rm -rf /app/library-solidity/lib; tar -xf "$1" -C /app; rm "$1"', "sh", remote]);
    await runWithHeartbeat(["docker", "exec", "-w", "/app/test-suite/e2e", TEST_SUITE_CONTAINER,
      "npx", "hardhat", "compile"], "compile rollout fixtures against baseline Solidity");
    return { revision: snapshot.revision, tree: snapshot.tree, archiveSha256: snapshot.sha256 };
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}
