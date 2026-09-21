// `bun run demo:smoke`: the deposit-arc acceptance gate. `bun test` exits 0 when every matched test
// is skipped, so the arc proves it ran by writing the layout's smoke marker; this script deletes the
// marker, runs the suite with the demo lane enabled, and requires the marker back.
import fs from "node:fs/promises";

import { solanaDemoSmokeMarkerPath } from "../src/layout";

await fs.rm(solanaDemoSmokeMarkerPath, { force: true });
const suite = Bun.spawn(["bun", "e2e/run.ts", "e2e/scenarios/deposit-arc.scenario.test.ts"], {
  cwd: import.meta.dir + "/..",
  env: { ...process.env, RUN_DEMO_SCENARIOS: "1" },
  stdio: ["inherit", "inherit", "inherit"],
});
const exitCode = await suite.exited;
if (exitCode !== 0) process.exit(exitCode);
try {
  await fs.access(solanaDemoSmokeMarkerPath);
} catch {
  console.error(`demo:smoke: the deposit arc did not run (no marker at ${solanaDemoSmokeMarkerPath})`);
  process.exit(1);
}
