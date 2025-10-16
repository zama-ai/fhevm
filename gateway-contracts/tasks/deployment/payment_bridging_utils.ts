import { Wallet } from "ethers";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";

import { getRequiredEnvVar, loadAddressEnvVarsFromFile, pascalCaseToAddressEnvVar } from "../utils";
import {
  appendAddressToEnvFile,
  appendAddressToSolidityFile,
  createEnvAddressesFile,
  createSolidityAddressesFile,
} from "./utils";

// Define the file names for registering the payment bridging contract addresses
const PAYMENT_BRIDGING_ADDRESSES_SOLIDITY_FILE_NAME = "PaymentBridgingAddresses.sol";

// Define the file name for registering the mocked payment bridging contract addresses
const MOCKED_PAYMENT_BRIDGING_ADDRESSES_ENV_FILE_NAME = ".env.mocked_payment_bridging";

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

// Deploy the mocked payment bridging contracts
// Currently, only the ZamaOFT contract is deployed as the FeesSenderToBurner contract can be
// simply mocked with a random address (there is no logic to test on this contract)
// We keep the command general enough if we ever need to consider additional contracts in the future
task("task:deployMockedPaymentBridgingContracts").setAction(async function (_, hre) {
  // Empty the mocked payment bridging contracts env file
  createEnvAddressesFile(MOCKED_PAYMENT_BRIDGING_ADDRESSES_ENV_FILE_NAME);

  // Compile the mocked payment bridging contracts
  await hre.run("compile:specific", { contract: `contracts/mockedPaymentBridging` });

  // Deploy the mocked ZamaOFT contract
  const zamaOFTContractName = "ZamaOFT";
  const zamaOFTAddress = await deployMockedPaymentBridgingContract(zamaOFTContractName, hre, [
    zamaOFTContractName,
    "ZAMA",
    BigInt(10 ** 24),
  ]);

  // Add the new address to the mocked payment bridging contracts env file
  appendAddressToEnvFile(zamaOFTContractName, zamaOFTAddress, MOCKED_PAYMENT_BRIDGING_ADDRESSES_ENV_FILE_NAME);
});

function setPaymentBridgingContractAddress(name: string) {
  const address = getRequiredEnvVar(pascalCaseToAddressEnvVar(name));

  console.log(`Setting ${name} address`);
  appendAddressToSolidityFile(name, address, PAYMENT_BRIDGING_ADDRESSES_SOLIDITY_FILE_NAME);

  console.log(`${name} address ${address} written successfully!\n`);
}

// Set the payment bridging contract addresses in the solidity file
export function setPaymentBridgingContractAddresses() {
  // Initialize the solidity file for payment bridging addresses
  createSolidityAddressesFile(PAYMENT_BRIDGING_ADDRESSES_SOLIDITY_FILE_NAME);

  // Set the payment bridging contract addresses in the solidity file
  setPaymentBridgingContractAddress("ZamaOFT");
  setPaymentBridgingContractAddress("FeesSenderToBurner");

  console.log("Payment bridging contract addresses set successfully!\n");
}

// Set the payment bridging contract addresses in the environment variables
task("task:setPaymentBridgingContractAddresses")
  .addParam(
    "useInternalAddresses",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalAddresses }) {
    if (useInternalAddresses) {
      loadAddressEnvVarsFromFile(MOCKED_PAYMENT_BRIDGING_ADDRESSES_ENV_FILE_NAME);
    }

    setPaymentBridgingContractAddresses();
  });
