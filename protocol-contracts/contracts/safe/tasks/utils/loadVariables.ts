import fs from "fs";
import path from "path";

// Get the required environment variable, throw an error if it's not set
// We only check if the variable is set, not if it's empty
export function getRequiredEnvVar(name: string, defaultValue?: any): string {
  if (!(name in process.env)) {
    throw new Error(`"${name}" env variable is not set`);
  }
  if (process.env[name] === "") {
    if (defaultValue === undefined) {
      throw new Error(`"${name}" env variable is not set`);
    }
    return defaultValue;
  }
  return process.env[name]!;
}

// Gets the deployed contract address from hardhat-deploy artifacts
export async function getDeployedAddress(
  network: string,
  contractName: string,
): Promise<string> {
  const deploymentsPath = path.join(
    "deployments",
    network,
    `${contractName}.json`,
  );
  try {
    const data = JSON.parse(fs.readFileSync(deploymentsPath, "utf8"));
    if (!data.address) {
      throw new Error(`No address found in ${deploymentsPath}`);
    }
    return data.address;
  } catch (error) {
    throw new Error(
      `Failed to read deployment from ${deploymentsPath}: ${error}`,
    );
  }
}
