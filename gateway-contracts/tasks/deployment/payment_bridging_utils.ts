import { Wallet } from "ethers";
import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";

import { loadAddressEnvVar, pascalCaseToAddressEnvVar } from "../utils";
import { getRequiredEnvVar } from "../utils/loadVariables";
import { appendAddressToSolidityFile, createSolidityAddressesFile } from "./utils";

// Define the file names for registering the payment bridging contract addresses
const PAYMENT_BRIDGING_ADDRESSES_SOLIDITY_FILE_NAME = "PaymentBridgingAddresses.sol";

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
task("task:deployMockedPaymentBridgingContracts").setAction(async function (args, hre) {
  // Compile the mocked payment bridging contracts
  await hre.run("compile:specific", { contract: `contracts/mockedPaymentBridging` });

  // Deploy the mocked ZamaOFT contract
  const zamaOFTContractName = "ZamaOFT";
  const zamaOFTAddress = await deployMockedPaymentBridgingContract(zamaOFTContractName, hre, [
    zamaOFTContractName,
    "ZAMA",
    BigInt(10 ** 24),
  ]);

  // Add the new address to env (in real environment, it will be already deployed)
  loadAddressEnvVar(zamaOFTContractName, zamaOFTAddress);
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

task("task:setPaymentBridgingContractAddresses").setAction(async function () {
  setPaymentBridgingContractAddresses();
});