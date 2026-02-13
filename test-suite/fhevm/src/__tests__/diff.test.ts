/**
 * Migration test — remove when the original bash scripts are retired.
 *
 * Diff test: structural comparison between the bash deploy script and the
 * TypeScript CLI's dry-run trace.
 *
 * Uses a pinned commit SHA to extract the original deploy script from git
 * history, so the test keeps working after the rewrite is merged.
 *
 * This test does NOT require Docker. It:
 *   1. Extracts the bash deploy-fhevm-stack.sh from git (pinned SHA).
 *   2. Runs the TypeScript CLI with `--dry-run` and captures the executor trace.
 *   3. Compares the two structurally.
 */

import { describe, expect, it, beforeAll } from "bun:test";
import { execSync } from "child_process";
import { STEPS } from "../dag.js";
import { setDryRun, getRecordedCommands, clearRecordedCommands, type DryRunEntry } from "../executor.js";
import { composeUp, composeDown, composeDownAll, waitForService } from "../docker.js";
import { composeFile, localEnvFile, PROJECT_NAME, FHEVM_ROOT } from "../paths.js";

// ---------------------------------------------------------------------------
// Golden trace: extract expected commands from the bash script
// ---------------------------------------------------------------------------

interface GoldenStep {
  id: string;
  compose: string | null;
  /** Expected docker compose up argv fragments (env file, compose file) */
  envFile: string | null;
  composeFile: string | null;
  waitTargets: { container: string; expect: "running" | "complete" }[];
  supportsBuild: boolean;
}

/**
 * Parse the bash deploy script to extract step definitions.
 * We read the actual file and extract:
 *   - run_compose calls with their component name and wait targets
 *   - The ordering of steps
 */
function parseBashScript(): GoldenStep[] {
  const content = execSync(
    "git show a4a9aa47d204cd2c6b6320883b7f3c4121957cb6:test-suite/fhevm/scripts/deploy-fhevm-stack.sh",
    { cwd: FHEVM_ROOT, encoding: "utf-8" },
  );
  const lines = content.split("\n");

  const steps: GoldenStep[] = [];

  // Extract step sequence from the script:
  // Pattern: `run_compose "component" "description" "container:state" ...`
  // Or:      `${RUN_COMPOSE} "component" "description" "container:state" ...`
  //
  // We also look for the DEPLOYMENT_STEPS array to get the step IDs.

  // First, extract the DEPLOYMENT_STEPS array
  const stepIds: string[] = [];
  let inStepsArray = false;
  for (const line of lines) {
    if (line.includes("DEPLOYMENT_STEPS=(")) {
      inStepsArray = true;
      continue;
    }
    if (inStepsArray) {
      if (line.includes(")")) break;
      const match = line.match(/"([^"]+)"/);
      if (match) stepIds.push(match[1]);
    }
  }

  // Now extract run_compose calls with their wait targets
  // We process the script section-by-section looking for step comments
  const runComposeRegex = /(?:run_compose|run_compose_with_build|\$\{RUN_COMPOSE\})\s+"([^"]+)"\s+"([^"]+)"\s*((?:\\\n|.)*?)$/;

  let currentStepId = "";
  let i = 0;
  while (i < lines.length) {
    const line = lines[i].trim();

    // Match step comment: `# Step N: <id>`
    const stepComment = line.match(/^# Step \d+: (.+)$/);
    if (stepComment) {
      currentStepId = stepComment[1].trim();
    }

    // Match run_compose call (may span multiple lines with backslash continuation)
    if (line.match(/(?:run_compose|RUN_COMPOSE)/)) {
      // Collect the full command including continuations
      let fullLine = lines[i];
      while (fullLine.trimEnd().endsWith("\\") && i + 1 < lines.length) {
        i++;
        fullLine += "\n" + lines[i];
      }

      // Extract component and wait targets
      const compMatch = fullLine.match(/(?:run_compose|run_compose_with_build|\$\{RUN_COMPOSE\})\s+"([^"]+)"\s+"([^"]+)"/);
      if (compMatch) {
        const component = compMatch[1];

        // Extract wait targets: "container:state" patterns
        const waitTargets: { container: string; expect: "running" | "complete" }[] = [];
        const targetRegex = /"([^"]+):(running|complete)"/g;
        let tMatch;
        // Skip the first two quoted strings (component and description)
        let quotedCount = 0;
        const targetStr = fullLine;
        const allQuoted = [...targetStr.matchAll(/"([^"]+)"/g)];
        for (const q of allQuoted.slice(2)) {
          const parts = q[1].split(":");
          if (parts.length === 2 && (parts[1] === "running" || parts[1] === "complete")) {
            // Strip ${PROJECT}- prefix
            const container = parts[0].replace(/\$\{PROJECT\}-?/, "fhevm-");
            waitTargets.push({
              container,
              expect: parts[1] as "running" | "complete",
            });
          }
        }

        // Check if this step uses RUN_COMPOSE (which could be run_compose_with_build)
        const supportsBuild = fullLine.includes("RUN_COMPOSE");

        steps.push({
          id: currentStepId || component,
          compose: component,
          envFile: `.env.${component}.local`,
          composeFile: `${component}-docker-compose.yml`,
          waitTargets,
          supportsBuild,
        });
      }
    }

    // Match kms-signer step (script call, no compose)
    if (line.includes("setup-kms-signer-address.sh") && currentStepId === "kms-signer") {
      steps.push({
        id: "kms-signer",
        compose: null,
        envFile: null,
        composeFile: null,
        waitTargets: [],
        supportsBuild: false,
      });
    }

    i++;
  }

  return steps;
}

// ---------------------------------------------------------------------------
// TypeScript trace: run the DAG in dry-run mode
// ---------------------------------------------------------------------------

interface TsStep {
  id: string;
  compose: string | null;
  composeFile: string | null;
  envFile: string | null;
  waitTargets: { container: string; expect: "running" | "complete" }[];
  supportsBuild: boolean;
}

function getTsSteps(): TsStep[] {
  return STEPS.map((step) => ({
    id: step.id,
    compose: step.compose,
    composeFile: step.compose ? `${step.compose}-docker-compose.yml` : null,
    envFile: step.env ? `.env.${step.env}.local` : null,
    waitTargets: step.services.map((s) => ({
      container: s.container,
      expect: s.expect,
    })),
    supportsBuild: step.supportsBuild,
  }));
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("diff test: bash script vs TypeScript DAG", () => {
  let bashSteps: GoldenStep[];
  let tsSteps: TsStep[];

  beforeAll(() => {
    bashSteps = parseBashScript();
    tsSteps = getTsSteps();
  });

  it("both have the same number of steps", () => {
    expect(tsSteps.length).toBe(bashSteps.length);
  });

  it("step IDs match in order", () => {
    const bashIds = bashSteps.map((s) => s.id);
    const tsIds = tsSteps.map((s) => s.id);
    expect(tsIds).toEqual(bashIds);
  });

  it("compose components match for each step", () => {
    for (let i = 0; i < bashSteps.length; i++) {
      expect(tsSteps[i].compose).toBe(bashSteps[i].compose);
    }
  });

  it("compose file names match for each step", () => {
    for (let i = 0; i < bashSteps.length; i++) {
      expect(tsSteps[i].composeFile).toBe(bashSteps[i].composeFile);
    }
  });

  it("env file names match for each step", () => {
    for (let i = 0; i < bashSteps.length; i++) {
      expect(tsSteps[i].envFile).toBe(bashSteps[i].envFile);
    }
  });

  it("wait targets match for each step", () => {
    for (let i = 0; i < bashSteps.length; i++) {
      const bash = bashSteps[i];
      const ts = tsSteps[i];

      // Compare containers and expected states
      expect(ts.waitTargets.length).toBe(bash.waitTargets.length);

      for (let j = 0; j < bash.waitTargets.length; j++) {
        expect(ts.waitTargets[j].container).toBe(bash.waitTargets[j].container);
        expect(ts.waitTargets[j].expect).toBe(bash.waitTargets[j].expect);
      }
    }
  });

  it("build support matches for each step", () => {
    for (let i = 0; i < bashSteps.length; i++) {
      // In bash, steps 1-4 (minio, core, kms-signer, database) use run_compose
      // (no build), steps 5-13 use ${RUN_COMPOSE} (which could be build)
      expect(tsSteps[i].supportsBuild).toBe(bashSteps[i].supportsBuild);
    }
  });
});

describe("diff test: dry-run executor trace", () => {
  beforeAll(() => {
    setDryRun(true);
    clearRecordedCommands();
  });

  it("full deploy dry-run produces correct docker compose up sequence", async () => {
    clearRecordedCommands();

    // Simulate the full deploy loop (without env file prep)
    for (const step of STEPS) {
      if (step.compose) {
        await composeUp({ component: step.compose });
      }
      for (const svc of step.services) {
        await waitForService(svc.container, svc.expect);
      }
    }

    const commands = getRecordedCommands();

    // Filter to just compose up commands
    const upCommands = commands.filter((c) =>
      c.argv.includes("up") && c.argv.includes("-d"),
    );

    // Should have 12 compose up commands (13 steps - 1 kms-signer with no compose)
    expect(upCommands.length).toBe(12);

    // Check the order
    const components = upCommands.map((c) => {
      // Extract the compose file name from the -f argument
      const fIdx = c.argv.indexOf("-f");
      if (fIdx >= 0) {
        const fValue = c.argv[fIdx + 1];
        // Extract component from "path/to/minio-docker-compose.yml"
        const match = fValue.match(/([^/]+)-docker-compose\.yml$/);
        return match ? match[1] : fValue;
      }
      return "unknown";
    });

    const expected = [
      "minio", "core", "database", "host-node", "gateway-node",
      "coprocessor", "kms-connector", "gateway-mocked-payment",
      "gateway-sc", "host-sc", "relayer", "test-suite",
    ];

    expect(components).toEqual(expected);

    setDryRun(false);
  });

  it("each compose up command has correct -p, --env-file, -f flags", async () => {
    setDryRun(true);
    clearRecordedCommands();

    for (const step of STEPS) {
      if (step.compose) {
        await composeUp({ component: step.compose });
      }
    }

    const commands = getRecordedCommands();

    for (const cmd of commands) {
      // Must have -p fhevm
      const pIdx = cmd.argv.indexOf("-p");
      expect(pIdx).toBeGreaterThan(-1);
      expect(cmd.argv[pIdx + 1]).toBe(PROJECT_NAME);

      // Must have --env-file
      const envIdx = cmd.argv.indexOf("--env-file");
      expect(envIdx).toBeGreaterThan(-1);
      expect(cmd.argv[envIdx + 1]).toContain(".env.");
      expect(cmd.argv[envIdx + 1]).toContain(".local");

      // Must have -f
      const fIdx = cmd.argv.indexOf("-f");
      expect(fIdx).toBeGreaterThan(-1);
      expect(cmd.argv[fIdx + 1]).toContain("-docker-compose.yml");
    }

    setDryRun(false);
  });

  it("--build flag propagates only to supportsBuild steps", async () => {
    setDryRun(true);
    clearRecordedCommands();

    for (const step of STEPS) {
      if (step.compose) {
        await composeUp({
          component: step.compose,
          build: step.supportsBuild,
        });
      }
    }

    const commands = getRecordedCommands();

    for (const cmd of commands) {
      const fIdx = cmd.argv.indexOf("-f");
      const composeFilePath = cmd.argv[fIdx + 1];
      const match = composeFilePath.match(/([^/]+)-docker-compose\.yml$/);
      const component = match ? match[1] : "";

      const step = STEPS.find((s) => s.compose === component);
      const hasBuild = cmd.argv.includes("--build");

      if (step?.supportsBuild) {
        expect(hasBuild).toBe(true);
      } else {
        expect(hasBuild).toBe(false);
      }
    }

    setDryRun(false);
  });

  it("cleanup produces docker compose down in reverse order", async () => {
    setDryRun(true);
    clearRecordedCommands();

    // Simulate the full cleanup (reverse order)
    const stepsWithCompose = [...STEPS].filter((s) => s.compose).reverse();
    for (const step of stepsWithCompose) {
      await composeDown({
        component: step.compose!,
        volumes: true,
        removeOrphans: true,
      });
    }

    const commands = getRecordedCommands();
    const downComponents = commands.map((c) => {
      const fIdx = c.argv.indexOf("-f");
      const match = c.argv[fIdx + 1]?.match(/([^/]+)-docker-compose\.yml$/);
      return match ? match[1] : "unknown";
    });

    const expected = [
      "test-suite", "relayer", "host-sc", "gateway-sc",
      "gateway-mocked-payment", "kms-connector", "coprocessor",
      "gateway-node", "host-node", "database", "core", "minio",
    ];

    expect(downComponents).toEqual(expected);

    setDryRun(false);
  });
});

describe("diff test: version defaults match bash script", () => {
  it("version defaults match those in fhevm-cli bash script", () => {
    // Extract the original bash fhevm-cli from git history (pinned SHA)
    const content = execSync(
      "git show a4a9aa47d204cd2c6b6320883b7f3c4121957cb6:test-suite/fhevm/fhevm-cli",
      { cwd: FHEVM_ROOT, encoding: "utf-8" },
    );

    const { VERSION_DEFAULTS } = require("../env.js");

    // Extract version assignments: `export VAR=${VAR:-"default"}`
    const versionRegex = /export\s+(\w+)=\$\{\1:-"([^"]+)"\}/g;
    let match;
    const bashVersions: Record<string, string> = {};

    while ((match = versionRegex.exec(content)) !== null) {
      bashVersions[match[1]] = match[2];
    }

    // Compare
    for (const [key, bashDefault] of Object.entries(bashVersions)) {
      if (key in VERSION_DEFAULTS) {
        expect(VERSION_DEFAULTS[key]).toBe(bashDefault);
      }
    }
  });
});
