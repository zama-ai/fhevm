import { Wallet } from "ethers";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";

import { getRequiredEnvVar } from "../../utils";
import { appendAddressToEnvFile, createEnvAddressesFile } from "../utils";

// Define the file name for registering the mocked payment bridging contract addresses
export const MOCKED_PAYMENT_BRIDGING_ADDRESSES_ENV_FILE_NAME = ".env.mocked_payment_bridging";

// Deploy a mocked payment bridging contract
async function deployMockedPaymentBridgingContract(
  name: string,
  hre: HardhatRuntimeEnvironment,
  initializeArgs: unknown[],
): Promise<string> {
  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  console.log(`Deploying ${name}...`);
  const contractFactory = await hre.ethers.getContractFactory(name, deployer);

  // If initializeArgs is a non-empty array, unpack the arguments, else directly call deploy
  const contract =
    Array.isArray(initializeArgs) && initializeArgs.length > 0
      ? await contractFactory.deploy(...initializeArgs)
      : await contractFactory.deploy();

  const contractAddress = await contract.getAddress();

  console.log(`${name} deployed successfully at address: ${contractAddress}\n`);

  return contractAddress;
}

// Deploy the mocked ZamaOFT contract
// Currently, only the ZamaOFT contract is deployed as the FeesSenderToBurner contract can be
// simply mocked with a random address (there is no logic to test on this contract)
// We keep the command general enough if we ever need to consider additional contracts in the future
// Initial supply is in mocked $ZAMA tokens (NOT in base units with 18 decimals)
task("task:deployMockedZamaOFT")
  .addOptionalParam(
    "initialSupply",
    "The initial supply of mocked $ZAMA tokens to deploy the ZamaOFT contract with",
    BigInt(10 ** 6),
    types.bigint,
  )
  .setAction(async function ({ initialSupply }, hre) {
    // Empty the mocked payment bridging contracts env file
    createEnvAddressesFile(MOCKED_PAYMENT_BRIDGING_ADDRESSES_ENV_FILE_NAME);

    // Compile the mocked payment bridging contracts
    await hre.run("compile:specific", { contract: `contracts/mockedPaymentBridging` });

    // Convert the initial supply to mocked $ZAMA base units (using 18 decimals)
    const initialSupplyInMockedZamaBaseUnits = initialSupply * BigInt(10 ** 18);

    // Deploy the mocked ZamaOFT contract
    const zamaOFTContractName = "ZamaOFT";
    const zamaOFTAddress = await deployMockedPaymentBridgingContract(zamaOFTContractName, hre, [
      zamaOFTContractName,
      "ZAMA",
      initialSupplyInMockedZamaBaseUnits,
    ]);

    // Add the new address to the mocked payment bridging contracts env file
    appendAddressToEnvFile(zamaOFTContractName, zamaOFTAddress, MOCKED_PAYMENT_BRIDGING_ADDRESSES_ENV_FILE_NAME);
  });
