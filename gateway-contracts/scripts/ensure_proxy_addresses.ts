#!/usr/bin/env node
import { execSync } from "child_process";
import * as dotenv from "dotenv";
import * as fs from "fs";
import * as path from "path";

function main(): void {
  try {
    // Get env path
    const envPath = process.env.ENV_PATH;

    if (!envPath) {
      console.error("ENV_PATH is not set");
      process.exit(1);
    } else if (!fs.existsSync(envPath)) {
      console.error("ENV_PATH does not exist");
      process.exit(1);
    }

    // Get addresses directory
    const addressesDir = "addresses";

    // Get gateway config address env file
    const gatewayConfigAddressEnv = path.join(addressesDir, ".env.gateway");

    let shouldGenerateAddresses = false;

    // If addresses/ directory doesn't exist, is empty, or gatewayConfig address file missing
    if (
      !fs.existsSync(addressesDir) ||
      fs.readdirSync(addressesDir).length === 0 ||
      !fs.existsSync(gatewayConfigAddressEnv)
    ) {
      shouldGenerateAddresses = true;
    } else {
      const envVars = dotenv.parse(fs.readFileSync(envPath));
      const configVars = dotenv.parse(fs.readFileSync(gatewayConfigAddressEnv));

      const envAddress = envVars.GATEWAY_CONFIG_ADDRESS;
      const fileAddress = configVars.GATEWAY_CONFIG_ADDRESS;

      if (!envAddress) {
        console.error(`GATEWAY_CONFIG_ADDRESS is not set in ${envPath}`);
        process.exit(1);
      }
      if (!fileAddress) {
        console.error(`GATEWAY_CONFIG_ADDRESS is not set in ${gatewayConfigAddressEnv}`);
        process.exit(1);
      }

      // If GATEWAY_CONFIG_ADDRESS in env path does not match the one in addresses/ directory,
      // this most likely indicates that the addresses need to be regenerated in order to match
      // the ones used in local development.
      if (envAddress !== fileAddress) {
        shouldGenerateAddresses = true;
      }
    }

    if (shouldGenerateAddresses) {
      console.log(`Generating contract addresses in development environment.`);
      // Deploy empty proxies and generate addresses
      execSync(`make deploy-empty-proxies`, {
        stdio: "inherit",
        env: process.env,
      });
    } else {
      console.log("Contract addresses match local development environment.");
    }
  } catch (error) {
    console.error("Error:", error);
    process.exit(1);
  }
}

main();
