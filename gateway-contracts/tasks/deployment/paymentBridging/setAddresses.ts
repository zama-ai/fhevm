import { task, types } from "hardhat/config";

import { getRequiredEnvVar, loadAddressEnvVarsFromFile, pascalCaseToAddressEnvVar } from "../../utils";
import { appendAddressToSolidityFile, createSolidityAddressesFile } from "../utils";
import { MOCKED_PAYMENT_BRIDGING_ADDRESSES_ENV_FILE_NAME } from "./mocked";

// Define the file names for registering the payment bridging contract addresses
const PAYMENT_BRIDGING_ADDRESSES_SOLIDITY_FILE_NAME = "PaymentBridgingAddresses.sol";

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
