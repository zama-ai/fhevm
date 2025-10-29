import fs from "fs";
import path from "path";

import {
  ADDRESSES_DIR,
  GATEWAY_ADDRESSES_ENV_FILE_NAME,
  GATEWAY_ADDRESSES_SOLIDITY_FILE_NAME,
} from "../../hardhat.config";
import { pascalCaseToAddressEnvVar, pascalCaseToCamelCase } from "../utils/stringOps";

// Define the empty proxy names for the different contracts
export const GATEWAY_CONFIG_EMPTY_PROXY_NAME = "EmptyUUPSProxyGatewayConfig";
export const REGULAR_EMPTY_PROXY_NAME = "EmptyUUPSProxy";

// Create an empty env file
export function createEnvAddressesFile(fileName: string) {
  // Make sure the addresses directory exists
  fs.mkdirSync(ADDRESSES_DIR, { recursive: true });

  const envFilePath = path.join(ADDRESSES_DIR, fileName);
  fs.writeFileSync(envFilePath, "", { flag: "w" });
}

// Create an empty solidity file with the Solidity header
export function createSolidityAddressesFile(fileName: string) {
  // Make sure the addresses directory exists
  fs.mkdirSync(ADDRESSES_DIR, { recursive: true });

  const solidityFilePath = path.join(ADDRESSES_DIR, fileName);
  const solidityHeader = `// SPDX-License-Identifier: BSD-3-Clause-Clear\npragma solidity ^0.8.24;\n\n`;
  fs.writeFileSync(solidityFilePath, solidityHeader, {
    encoding: "utf8",
    flag: "w",
  });
}

// Append the contract's address in the solidity file
export function appendAddressToSolidityFile(name: string, address: string, solidityFileName: string) {
  const solidityFilePath = path.join(ADDRESSES_DIR, solidityFileName);

  // Make sure the addresses directory exists
  if (!fs.existsSync(ADDRESSES_DIR)) {
    throw new Error(`Addresses directory "${ADDRESSES_DIR}" not found`);
  }

  // Make sure the solidity file exists. We need this to make sure the file has been initialized
  // with the Solidity header
  if (!fs.existsSync(solidityFilePath)) {
    throw new Error(`Solidity file "${solidityFilePath}" not found for contract ${name} with address ${address}`);
  }

  const solidityTemplate = `address constant ${pascalCaseToCamelCase(name)}Address = ${address};\n`;

  fs.appendFileSync(solidityFilePath, solidityTemplate, {
    encoding: "utf8",
    flag: "a",
  });
}

// Append the contract's address in the env file
export function appendAddressToEnvFile(name: string, address: string, envFileName: string) {
  const envFilePath = path.join(ADDRESSES_DIR, envFileName);

  const envContent = `${pascalCaseToAddressEnvVar(name)}=${address}\n`;

  // Make sure the addresses directory exists. We don't need to check if the actual file exists as
  // `appendFileSync` will create the file if it doesn't
  if (!fs.existsSync(ADDRESSES_DIR)) {
    throw new Error(`Addresses directory "${ADDRESSES_DIR}" not found`);
  }

  // Append the contract's address in the addresses/.env.gateway file
  fs.appendFileSync(envFilePath, envContent, { encoding: "utf8", flag: "a" });
}

// Update a gateway contract's address in the .env and solidity files in the `./addresses` directory
export function setGatewayContractAddress(name: string, address: string) {
  appendAddressToEnvFile(name, address, GATEWAY_ADDRESSES_ENV_FILE_NAME);
  appendAddressToSolidityFile(name, address, GATEWAY_ADDRESSES_SOLIDITY_FILE_NAME);

  console.log(`${name} address ${address} written successfully!\n`);
}
