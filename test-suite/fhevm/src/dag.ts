import { getContainerIp, type ServiceExpect } from "./docker.js";
import { patchEnvVar } from "./env.js";
import { localEnvFile } from "./paths.js";
import { log } from "./log.js";
import { isDryRun, spawn } from "./executor.js";

export type StepId =
  | "minio"
  | "core"
  | "kms-signer"
  | "database"
  | "host-node"
  | "gateway-node"
  | "coprocessor"
  | "kms-connector"
  | "gateway-mocked-payment"
  | "gateway-sc"
  | "host-sc"
  | "relayer"
  | "test-suite";

export interface ServiceWait {
  container: string;
  expect: ServiceExpect;
}

export interface DeployContext {
  build: boolean;
  local: boolean;
}

export interface DeployStep {
  id: StepId;
  label: string;
  dependsOn: StepId[];
  /** Compose component name — null for steps that are script-only (e.g. kms-signer) */
  compose: string | null;
  /** Env component name — null for steps that don't use env files */
  env: string | null;
  services: ServiceWait[];
  supportsBuild: boolean;
  /** Custom logic to run after compose up + wait (e.g. minio IP patching, kms-signer setup) */
  postDeploy?: (ctx: DeployContext) => Promise<void>;
}

/** All deployment steps in execution order */
export const STEPS: DeployStep[] = [
  {
    id: "minio",
    label: "MinIO Services",
    dependsOn: [],
    compose: "minio",
    env: "minio",
    supportsBuild: false,
    services: [
      { container: "fhevm-minio", expect: "running" },
      { container: "fhevm-minio-setup", expect: "complete" },
    ],
    postDeploy: async () => {
      const ip = await getContainerIp("fhevm-minio");
      log.info(`Found fhevm-minio container IP: ${ip}`);
      await patchEnvVar(localEnvFile("coprocessor"), "AWS_ENDPOINT_URL", `http://${ip}:9000`);
      log.info(`Updated AWS_ENDPOINT_URL to http://${ip}:9000`);
    },
  },
  {
    id: "core",
    label: "Core Services",
    dependsOn: ["minio"],
    compose: "core",
    env: "core",
    supportsBuild: false,
    services: [
      { container: "kms-core", expect: "running" },
    ],
  },
  {
    id: "kms-signer",
    label: "KMS Signer Setup",
    dependsOn: ["core"],
    compose: null,
    env: null,
    supportsBuild: false,
    services: [],
    postDeploy: async () => {
      if (isDryRun()) {
        log.info("[dry-run] kms-signer: docker logs kms-core → extract key handle");
        log.info("[dry-run] kms-signer: fetch signer address from MinIO");
        log.info("[dry-run] kms-signer: patchEnvVar host-sc KMS_SIGNER_ADDRESS_0");
        log.info("[dry-run] kms-signer: patchEnvVar gateway-sc KMS_SIGNER_ADDRESS_0");
        return;
      }

      // Poll kms-core logs for the signing key handle
      const maxRetries = 30;
      const intervalMs = 5000;
      let keySignerId = "";

      log.info("Waiting for KMS signing key to be generated...");
      for (let i = 1; i <= maxRetries; i++) {
        const proc = spawn(["docker", "logs", "kms-core"], {
          stdout: "pipe",
          stderr: "pipe",
        });
        const output = await new Response(proc.stdout!).text();
        const stderr = await new Response(proc.stderr!).text();
        await proc.exited;

        const combined = output + stderr;
        const match = combined.match(
          /Successfully stored public centralized server signing key under the handle ([^ \n]*)/,
        );
        if (match) {
          keySignerId = match[1].trim();
          break;
        }

        if (i < maxRetries) {
          log.warn(`KMS signing key not found yet, waiting ${intervalMs / 1000}s... (${i}/${maxRetries})`);
          await Bun.sleep(intervalMs);
        } else {
          throw new Error("Failed to extract signing key ID from kms-core logs");
        }
      }

      log.info(`Signing Key ID: ${keySignerId}`);

      // Retrieve the signer address from MinIO via KMS public endpoint
      const signerAddressUrl = `http://localhost:9000/kms-public/PUB/VerfAddress/${keySignerId}`;
      log.info(`Retrieving KMS signer address from ${signerAddressUrl}`);

      const response = await fetch(signerAddressUrl);
      if (!response.ok) {
        throw new Error(`Failed to retrieve signer address: HTTP ${response.status}`);
      }
      const signerAddress = (await response.text()).trim();

      // Validate the address format
      if (!/^0x[a-fA-F0-9]{40}$/.test(signerAddress)) {
        log.warn(`Retrieved signer address doesn't match expected format: ${signerAddress}`);
        log.info("Using the address anyway, please verify manually.");
      }

      // Patch both host-sc and gateway-sc env files
      const hostEnv = localEnvFile("host-sc");
      const gatewayEnv = localEnvFile("gateway-sc");

      log.info(`Updating KMS_SIGNER_ADDRESS_0 in ${hostEnv}...`);
      await patchEnvVar(hostEnv, "KMS_SIGNER_ADDRESS_0", signerAddress);
      log.info(`KMS_SIGNER_ADDRESS_0 successfully updated to: ${signerAddress} in ${hostEnv}`);

      log.info(`Updating KMS_SIGNER_ADDRESS_0 in ${gatewayEnv}...`);
      await patchEnvVar(gatewayEnv, "KMS_SIGNER_ADDRESS_0", signerAddress);
      log.info(`KMS_SIGNER_ADDRESS_0 successfully updated to: ${signerAddress} in ${gatewayEnv}`);

      log.info("KMS signer address configuration files updated successfully!");
    },
  },
  {
    id: "database",
    label: "Database Service",
    dependsOn: ["core"],
    compose: "database",
    env: "database",
    supportsBuild: false,
    services: [
      { container: "coprocessor-and-kms-db", expect: "running" },
    ],
  },
  {
    id: "host-node",
    label: "Host Node Service",
    dependsOn: ["database"],
    compose: "host-node",
    env: "host-node",
    supportsBuild: true,
    services: [
      { container: "host-node", expect: "running" },
    ],
  },
  {
    id: "gateway-node",
    label: "Gateway Node Service",
    dependsOn: ["database"],
    compose: "gateway-node",
    env: "gateway-node",
    supportsBuild: true,
    services: [
      { container: "gateway-node", expect: "running" },
    ],
  },
  {
    id: "coprocessor",
    label: "Coprocessor Services",
    dependsOn: ["database"],
    compose: "coprocessor",
    env: "coprocessor",
    supportsBuild: true,
    services: [
      { container: "coprocessor-and-kms-db", expect: "running" },
      { container: "coprocessor-db-migration", expect: "complete" },
      { container: "coprocessor-host-listener", expect: "running" },
      { container: "coprocessor-gw-listener", expect: "running" },
      { container: "coprocessor-tfhe-worker", expect: "running" },
      { container: "coprocessor-zkproof-worker", expect: "running" },
      { container: "coprocessor-sns-worker", expect: "running" },
      { container: "coprocessor-transaction-sender", expect: "running" },
    ],
  },
  {
    id: "kms-connector",
    label: "KMS Connector Services",
    dependsOn: ["database"],
    compose: "kms-connector",
    env: "kms-connector",
    supportsBuild: true,
    services: [
      { container: "coprocessor-and-kms-db", expect: "running" },
      { container: "kms-connector-db-migration", expect: "complete" },
      { container: "kms-connector-gw-listener", expect: "running" },
      { container: "kms-connector-kms-worker", expect: "running" },
      { container: "kms-connector-tx-sender", expect: "running" },
    ],
  },
  {
    id: "gateway-mocked-payment",
    label: "Gateway Mocked Payment",
    dependsOn: ["kms-connector"],
    compose: "gateway-mocked-payment",
    env: "gateway-mocked-payment",
    supportsBuild: true,
    services: [
      { container: "gateway-deploy-mocked-zama-oft", expect: "complete" },
      { container: "gateway-set-relayer-mocked-payment", expect: "complete" },
    ],
  },
  {
    id: "gateway-sc",
    label: "Gateway Contracts",
    dependsOn: ["gateway-mocked-payment"],
    compose: "gateway-sc",
    env: "gateway-sc",
    supportsBuild: true,
    services: [
      { container: "gateway-sc-deploy", expect: "complete" },
      { container: "gateway-sc-add-network", expect: "complete" },
      { container: "gateway-sc-trigger-keygen", expect: "complete" },
      { container: "gateway-sc-trigger-crsgen", expect: "complete" },
      { container: "gateway-sc-add-pausers", expect: "complete" },
    ],
  },
  {
    id: "host-sc",
    label: "Host Contracts",
    dependsOn: ["gateway-sc"],
    compose: "host-sc",
    env: "host-sc",
    supportsBuild: true,
    services: [
      { container: "host-sc-deploy", expect: "complete" },
      { container: "host-sc-add-pausers", expect: "complete" },
    ],
  },
  {
    id: "relayer",
    label: "Relayer Services",
    dependsOn: ["host-sc"],
    compose: "relayer",
    env: "relayer",
    supportsBuild: true,
    services: [
      { container: "fhevm-relayer", expect: "running" },
    ],
  },
  {
    id: "test-suite",
    label: "Test Suite",
    dependsOn: ["relayer"],
    compose: "test-suite",
    env: "test-suite",
    supportsBuild: true,
    services: [
      { container: "fhevm-test-suite-e2e-debug", expect: "running" },
    ],
  },
];

/** All valid step IDs */
export const STEP_IDS: StepId[] = STEPS.map((s) => s.id);

/** Get the index of a step by ID, returns -1 if not found */
export function getStepIndex(id: StepId): number {
  return STEPS.findIndex((s) => s.id === id);
}

/** Get a step by ID */
export function getStep(id: StepId): DeployStep | undefined {
  return STEPS.find((s) => s.id === id);
}

/**
 * Filter steps for execution based on --resume or --only flags.
 * - Normal mode: returns all steps
 * - --resume: returns from the resume step onward
 * - --only: returns just the specified step
 */
export function filterSteps(
  opts: { resume?: StepId; only?: StepId },
): DeployStep[] {
  if (opts.only) {
    const step = STEPS.find((s) => s.id === opts.only);
    return step ? [step] : [];
  }

  if (opts.resume) {
    const idx = getStepIndex(opts.resume);
    if (idx === -1) return [];
    return STEPS.slice(idx);
  }

  return [...STEPS];
}

/**
 * Determine which steps need cleanup before execution.
 * Returns steps with compose files that should be torn down, in reverse order.
 * - Normal mode: all steps (full cleanup)
 * - --resume: steps from the resume point onward
 * - --only: just the single step
 */
export function stepsToCleanup(
  opts: { resume?: StepId; only?: StepId },
): DeployStep[] {
  let steps: DeployStep[];

  if (opts.only) {
    const step = STEPS.find((s) => s.id === opts.only);
    steps = step ? [step] : [];
  } else if (opts.resume) {
    const idx = getStepIndex(opts.resume);
    if (idx === -1) return [];
    steps = STEPS.slice(idx);
  } else {
    steps = [...STEPS];
  }

  // Only include steps that have compose files, reversed
  return steps.filter((s) => s.compose !== null).reverse();
}

/**
 * Validate the DAG: check that all dependsOn references are valid step IDs
 * and there are no cycles (since execution is sequential, we just verify references).
 */
export function validateDag(): void {
  const ids = new Set(STEP_IDS);
  for (const step of STEPS) {
    for (const dep of step.dependsOn) {
      if (!ids.has(dep)) {
        throw new Error(`Step "${step.id}" depends on unknown step "${dep}"`);
      }
      if (dep === step.id) {
        throw new Error(`Step "${step.id}" depends on itself`);
      }
    }
  }
}
