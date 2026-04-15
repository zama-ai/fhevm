import fs from "node:fs/promises";
import path from "node:path";

import { runContractTask } from "./contracts";

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

const manifestFromTree = async (root: string) => {
  const raw = await fs.readFile(path.join(root, "upgrade-manifest.json"), "utf8");
  const manifest = JSON.parse(raw) as string[];
  if (!Array.isArray(manifest)) {
    throw new Error("upgrade-manifest.json must be an array");
  }
  return manifest;
};

const contractSource = async (root: string, name: string) => {
  const filePath = path.join(root, "contracts", `${name}.sol`);
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
  previousRoot: string,
  currentRoot: string,
): Promise<UpgradeAction[]> => {
  const manifest = await manifestFromTree(currentRoot);
  const actions: UpgradeAction[] = [];

  for (const name of manifest) {
    const current = await contractSource(currentRoot, name);
    const previous = await contractSource(previousRoot, name);

    if (!previous.exists) {
      actions.push({ type: "deploy-new", name });
      continue;
    }

    const previousVersion = parseReinitializerVersion(previous.source, previous.filePath);
    const currentVersion = parseReinitializerVersion(current.source, current.filePath);
    if (previousVersion === currentVersion) {
      actions.push({ type: "skip", name, reason: `reinitializer unchanged: ${previousVersion}` });
      continue;
    }

    actions.push({ type: "upgrade", name });
  }

  return actions;
};

const quoted = (value: string) => JSON.stringify(value);

export const runContractUpgrades = async (surface: ContractSurface, previousRoot: string, currentRoot: string) => {
  const component = surface === "gateway" ? "gateway-sc" : "host-sc";
  const service = surface === "gateway" ? "gateway-sc-compat-upgrade" : "host-sc-compat-upgrade";
  const actions = await planContractUpgrades(previousRoot, currentRoot);

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
