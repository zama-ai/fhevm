import type { RolloutRunContext } from "../src/commands/rollout-run";

export type RolloutEnv = Record<string, string>;

export const logPhase = (label: string) => {
  console.log(`\n[rollout] ${label}`);
};

const fullyQualifiedContractName = (sourceRoot: string, contractName: string) =>
  `${sourceRoot}/${contractName}.sol:${contractName}`;

const upgradeCommand = (task: string, currentImplementation: string, newImplementation: string) =>
  [
    "npx hardhat",
    task,
    `--current-implementation ${currentImplementation}`,
    `--new-implementation ${newImplementation}`,
    "--verify-contract false",
    "--use-internal-proxy-address true",
  ].join(" ");

export const upgradeContract = async (runTask: (command: string) => Promise<void>, task: string, name: string) => {
  const current = fullyQualifiedContractName("previous-contracts", name);
  const next = fullyQualifiedContractName("contracts", name);
  console.log(`[contracts] ${name}: ${current} -> ${next}`);
  await runTask(upgradeCommand(task, current, next));
};
