import { getServiceByName, type EnvFileName, type ServiceDefinition } from "../config/service-map";
import { ExitCode, FhevmCliError } from "../errors";

export interface BootStep {
  number: number;
  name: string;
  displayName: string;
  serviceNames: string[];
  parallelGroup?: number;
  envFilesToRegenerate?: EnvFileName[];
}

export const BOOT_STEPS: readonly BootStep[] = [
  {
    number: 1,
    name: "minio",
    displayName: "MinIO (S3 storage)",
    serviceNames: ["minio", "minio-setup"],
  },
  {
    number: 2,
    name: "kms-core",
    displayName: "KMS Core",
    serviceNames: ["kms-core"],
  },
  {
    number: 3,
    name: "kms-signer",
    displayName: "KMS Signer Discovery",
    serviceNames: [],
    envFilesToRegenerate: ["gateway-sc", "host-sc"],
  },
  {
    number: 4,
    name: "postgres",
    displayName: "PostgreSQL",
    serviceNames: ["db"],
  },
  {
    number: 5,
    name: "host-node",
    displayName: "Host Node (Anvil)",
    serviceNames: ["host-node"],
    parallelGroup: 1,
  },
  {
    number: 6,
    name: "gateway-node",
    displayName: "Gateway Node (Anvil)",
    serviceNames: ["gateway-node"],
    parallelGroup: 1,
  },
  {
    number: 7,
    name: "gateway-mocked-payment",
    displayName: "Gateway Mocked Payment",
    serviceNames: ["gateway-deploy-mocked-zama-oft", "gateway-set-relayer-mocked-payment"],
  },
  {
    number: 8,
    name: "gateway-contracts",
    displayName: "Gateway Smart Contracts",
    serviceNames: [
      "gateway-sc-deploy",
      "gateway-sc-add-network",
      "gateway-sc-add-pausers",
      "gateway-sc-trigger-keygen",
      "gateway-sc-trigger-crsgen",
    ],
  },
  {
    number: 9,
    name: "host-contracts",
    displayName: "Host Smart Contracts",
    serviceNames: ["host-sc-deploy", "host-sc-add-pausers"],
  },
  {
    number: 10,
    name: "kms-connector",
    displayName: "KMS Connector + FHE Key Discovery",
    serviceNames: [
      "kms-connector-db-migration",
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
    ],
  },
  {
    number: 11,
    name: "coprocessor",
    displayName: "Coprocessor (8 services)",
    serviceNames: [
      "coprocessor-db-migration",
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
      "coprocessor-gw-listener",
      "coprocessor-tfhe-worker",
      "coprocessor-zkproof-worker",
      "coprocessor-sns-worker",
      "coprocessor-transaction-sender",
    ],
  },
  {
    number: 12,
    name: "relayer",
    displayName: "Relayer",
    serviceNames: ["relayer-db", "relayer-db-migration", "relayer"],
  },
  {
    number: 13,
    name: "test-suite",
    displayName: "Test Suite",
    serviceNames: ["test-suite-e2e-debug"],
  },
] as const;

export function getStepByNumber(num: number): BootStep | undefined {
  return BOOT_STEPS.find((step) => step.number === num);
}

export function getStepByName(name: string): BootStep | undefined {
  return BOOT_STEPS.find((step) => step.name === name.toLowerCase());
}

export function resolveStepRef(ref: string): BootStep {
  const normalized = ref.trim();
  if (!normalized) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "boot-steps",
      message: "step reference is empty",
    });
  }

  const asNumber = Number.parseInt(normalized, 10);
  const byNumber = Number.isFinite(asNumber) && String(asNumber) === normalized ? getStepByNumber(asNumber) : undefined;
  const byName = getStepByName(normalized.toLowerCase());
  const step = byNumber ?? byName;

  if (!step) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "boot-steps",
      message: `unknown step reference: ${ref}`,
    });
  }

  return step;
}

export function getStepsFromNumber(startFrom: number): BootStep[] {
  return BOOT_STEPS.filter((step) => step.number >= startFrom);
}

export function getParallelGroup(steps: readonly BootStep[], currentIndex: number): BootStep[] {
  const current = steps[currentIndex];
  if (!current) {
    return [];
  }

  if (current.parallelGroup === undefined) {
    return [current];
  }

  const group: BootStep[] = [current];
  for (let index = currentIndex + 1; index < steps.length; index += 1) {
    const next = steps[index];
    if (!next || next.parallelGroup !== current.parallelGroup) {
      break;
    }
    group.push(next);
  }

  return group;
}

export function getServicesForStep(step: BootStep): ServiceDefinition[] {
  return step.serviceNames.map((name) => {
    const service = getServiceByName(name);
    if (!service) {
      throw new FhevmCliError({
        exitCode: ExitCode.CONFIG,
        step: `step-${step.number}`,
        message: `unknown service in step ${step.number}: ${name}`,
      });
    }
    return service;
  });
}

export function getStepsTeardownOrder(fromStep: number): BootStep[] {
  return getStepsFromNumber(fromStep).slice().reverse();
}
