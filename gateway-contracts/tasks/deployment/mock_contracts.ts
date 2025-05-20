import fs from "fs";
import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";
import path from "path";

const MOCK_DIR = path.join(__dirname, "../../contracts/mocks");

// Deploy the mock contracts (one or all)
task("task:deployGatewayMockContracts")
  .setDescription("Deploys one or all mock contracts")
  .addOptionalParam("name", "The name in PascalCase of the mock contract to deploy")
  .setAction(async function (taskArguments: TaskArguments, hre) {
    // Compile the mock contracts
    await hre.run("compile:specific", { contract: "contracts/mocks" });

    // Find and filter mock contracts from /contracts/mocks directory
    let mockContracts = fs
      .readdirSync(MOCK_DIR)
      .filter((file) => file.endsWith(".sol"))
      .map((file) => file.replace(".sol", ""));

    // Validate the "name" parameter and set it for deployment if present
    const name = taskArguments.name;
    if (name) {
      if (!mockContracts.includes(name)) {
        throw new Error(`Invalid mock contract name: ${name}`);
      }
      mockContracts = [name];
    }

    // Deploy mock contract for given "name" or all available mock contracts
    for (const mockContract of mockContracts) {
      const mockContractFactory = await hre.ethers.getContractFactory(mockContract);
      const mockContractDeployment = await mockContractFactory.deploy();
      const mockContractAddress = await mockContractDeployment.getAddress();
      console.log(`${mockContract} code set successfully at address: ${mockContractAddress}\n`);
    }

    console.log("Mock contract deployment done!");
  });
