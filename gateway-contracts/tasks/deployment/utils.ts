import fs from "fs";
import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";
import path from "path";

import { ADDRESSES_DIR } from "../../hardhat.config";
import { pascalCaseToCamelCase, pascalCaseToSnakeCase } from "../utils/stringOps";

// Define the empty proxy names for the different contracts
export const GATEWAY_CONFIG_EMPTY_PROXY_NAME = "EmptyUUPSProxyGatewayConfig";
export const REGULAR_EMPTY_PROXY_NAME = "EmptyUUPSProxy";

// A helper task to update a contract's address in their .sol and .env file in the `addresses` directory
task("task:setContractAddress")
  .addParam("name", "The name of the contract (PascalCase)")
  .addParam("address", "The address of the contract")
  .setAction(async function ({ name, address }: TaskArguments) {
    const nameSnakeCase = pascalCaseToSnakeCase(name);
    const envFilePath = path.join(ADDRESSES_DIR, ".env.gateway");
    const solidityFilePath = path.join(ADDRESSES_DIR, "GatewayAddresses.sol");
    const envContent = `${nameSnakeCase.toUpperCase()}_ADDRESS=${address}\n`;
    const solidityTemplate = `address constant ${pascalCaseToCamelCase(name)}Address = ${address};\n`;

    try {
      // Append the contract's address in the addresses/.env.gateway file
      fs.appendFileSync(envFilePath, envContent, { encoding: "utf8", flag: "a" });

      // Append the contract's address in the addresses/GatewayAddresses.sol file
      fs.appendFileSync(solidityFilePath, solidityTemplate, {
        encoding: "utf8",
        flag: "a",
      });
      console.log(`${name} address ${address} written successfully!\n`);
    } catch (err) {
      console.error(`Failed to write ${name} address:`, err);
    }
  });
