import fs from "node:fs/promises";
import path from "node:path";

import { captureContractTask, runContractTask } from "./contracts";

type ContractSurface = "gateway" | "host";

type UpgradeAction =
  | { type: "skip"; name: string; reason: string }
  | { type: "upgrade"; name: string }
  | { type: "deploy-new"; name: string };

const parseReinitializerVersion = (source: string, filePath: string) => {
  const match = source.match(/REINITIALIZER_VERSION\s*=\s*(\d+)/);
  if (!match) {
    throw new Error(`Failed to parse REINITIALIZER_VERSION from ${filePath}`);
  }
  return Number(match[1]);
};

const manifestFromService = async (component: "host-sc" | "gateway-sc", service: string) => {
  const raw = await captureContractTask(component, service, "cat /app/upgrade-manifest.json");
  const manifest = JSON.parse(raw) as string[];
  if (!Array.isArray(manifest)) {
    throw new Error("upgrade-manifest.json must be an array");
  }
  return manifest;
};

const serviceContractSource = (component: "host-sc" | "gateway-sc", service: string, name: string) =>
  captureContractTask(component, service, `cat ${JSON.stringify(`contracts/${name}.sol`)}`);

const previousContractSource = async (previousRoot: string, name: string) => {
  const filePath = path.join(previousRoot, "contracts", `${name}.sol`);
  try {
    return { exists: true as const, filePath, source: await fs.readFile(filePath, "utf8") };
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") {
      return { exists: false as const, filePath, source: "" };
    }
    throw error;
  }
};

const planContractUpgrades = async (
  component: "host-sc" | "gateway-sc",
  service: string,
  previousRoot: string,
): Promise<UpgradeAction[]> => {
  const manifest = await manifestFromService(component, service);
  const actions: UpgradeAction[] = [];

  for (const name of manifest) {
    const currentSource = await serviceContractSource(component, service, name);
    const currentFile = `contracts/${name}.sol`;
    const previous = await previousContractSource(previousRoot, name);

    if (!previous.exists) {
      actions.push({ type: "deploy-new", name });
      continue;
    }

    const previousVersion = parseReinitializerVersion(previous.source, previous.filePath);
    const currentVersion = parseReinitializerVersion(currentSource, currentFile);
    if (previousVersion === currentVersion) {
      actions.push({ type: "skip", name, reason: `reinitializer unchanged: ${previousVersion}` });
      continue;
    }

    actions.push({ type: "upgrade", name });
  }

  return actions;
};

const quoted = (value: string) => JSON.stringify(value);

export const runContractUpgrades = async (surface: ContractSurface, previousRoot: string) => {
  const component = surface === "gateway" ? "gateway-sc" : "host-sc";
  const service = surface === "gateway" ? "gateway-sc-compat-upgrade" : "host-sc-compat-upgrade";
  const actions = await planContractUpgrades(component, service, previousRoot);

  for (const action of actions) {
    if (action.type === "skip") {
      console.log(`Skipping ${action.name} (${action.reason})`);
      continue;
    }

    if (action.type === "deploy-new") {
      console.log(`Deploying ${action.name} (new in target release)`);
      await runContractTask(component, service, `npx hardhat ${quoted(`task:deploy${action.name}`)}`);
      continue;
    }

    console.log(`Upgrading ${action.name}`);
    await runContractTask(
      component,
      service,
      [
        "npx hardhat",
        quoted(`task:upgrade${action.name}`),
        "--current-implementation",
        quoted(`previous-contracts/contracts/${action.name}.sol:${action.name}`),
        "--new-implementation",
        quoted(`contracts/${action.name}.sol:${action.name}`),
        "--use-internal-proxy-address true",
        "--verify-contract false",
      ].join(" "),
    );
  }
};
