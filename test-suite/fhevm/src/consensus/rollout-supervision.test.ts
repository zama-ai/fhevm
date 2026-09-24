import { expect, test } from "bun:test";
import { mkdtemp, mkdir, readFile, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";

for (const fails of [false, true]) {
  test(`rollout supervisor ${fails ? "cancels failed" : "completes successful"} workloads`, async () => {
    const root = await mkdtemp(path.join(tmpdir(), "rollout-supervision-"));
    try {
      const scripts = path.join(root, "test-suite/fhevm/scripts");
      await mkdir(scripts, { recursive: true });
      await writeFile(path.join(scripts, "interrupt-key-application.sh"), `
set -euo pipefail
printf 'ROLLOUT_HOLD_READY\\n'
if IFS= read -r command; then
  [[ "$command" == release ]]
  printf completed > "$OBSERVATION"
else
  printf cancelled > "$OBSERVATION"
fi
`);
      const source = await readFile(path.join(import.meta.dir, "rollout-supervision.ts"), "utf8");
      const modulePath = path.join(root, "supervisor.ts");
      await writeFile(modulePath, source.replace('import { REPO_ROOT } from "../layout";', `const REPO_ROOT = ${JSON.stringify(root)};`));
      const { withRolloutSupervisor } = await import(pathToFileURL(modulePath).href);
      const observation = path.join(root, "observation");
      const workloadError = new Error("workload failed before fault");
      const result = withRolloutSupervisor(root, "interrupt-key-application.sh", [], { OBSERVATION: observation }, async () => {
        if (fails) throw workloadError;
        return 42;
      });
      if (fails) await expect(result).rejects.toBe(workloadError);
      else expect(await result).toBe(42);
      expect(await readFile(observation, "utf8")).toBe(fails ? "cancelled" : "completed");
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });
}
