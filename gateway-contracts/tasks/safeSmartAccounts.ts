import { OperationType } from "@safe-global/types-kit";
import dotenv from "dotenv";
import { EventLog, Log, Wallet, getBytes } from "ethers";
import fs from "fs";
import { task, types } from "hardhat/config";
import path from "path";

import { ADDRESSES_DIR } from "../hardhat.config";
import { getRequiredEnvVar } from "./utils/loadVariables";
import { pascalCaseToSnakeCase } from "./utils/stringOps";

async function getSortedSignatures(signers: Wallet[], transactionHash: string): Promise<string> {
  const bytesDataHash = getBytes(transactionHash);

  let signatureBytes = "0x";

  // Get the addresses of the signers
  const signerAddresses = await Promise.all(signers.map((signer) => signer.getAddress()));

  // Sort the signers by their addresses
  // Gnosis Safe requires signatures to be provided in ascending order of the signer addresses
  // for security and efficiency reasons. See https://docs.safe.global/advanced/smart-account-signatures.
  const sortedSigners = signers.sort((a, b) => {
    const addressA = signerAddresses[signers.indexOf(a)];
    const addressB = signerAddresses[signers.indexOf(b)];
    return addressA.localeCompare(addressB, "en", { sensitivity: "base" });
  });

  // Sign the transaction hash with each signer
  for (const signer of sortedSigners) {
    const signedMessage = await signer.signMessage(bytesDataHash);
    const flatSig = signedMessage.replace(/1b$/, "1f").replace(/1c$/, "20");
    signatureBytes += flatSig.slice(2);
  }

  return signatureBytes;
}

task(
  "task:deploySafeSmartAccountImplementation",
  "Deploys the Safe Smart Account singleton implementation contract",
).setAction(async function (_, { ethers, run }) {
  // Compile contracts from external dependencies (e.g., Safe Smart Account).
  // These are temporarily stored by `hardhat-dependency-compiler`.
  // See the `dependencyCompiler` field in `hardhat.config.ts` for configuration details.
  await run("compile:specific", { contract: "hardhat-dependency-compiler" });

  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Deploy a new Safe implementation contract
  const safeFactory = await ethers.getContractFactory("Safe", deployer);
  const safe = await safeFactory.deploy();
  const safeAddress = await safe.getAddress();

  console.log(`Safe Smart Account implementation deployed at address: ${safeAddress}`);
});

task("task:deploySafeSmartAccountProxy", "Deploys the Safe Smart Account contract")
  .addParam(
    "safeSmartAccountImplAddress",
    "Address of the Safe Smart Account implementation contract.",
    undefined,
    types.string,
  )
  .addParam("owners", "List of addresses that control the OwnerSafeSmartAccount.", undefined, types.json)
  .addParam(
    "threshold",
    "Number of required confirmations for a OwnerSafeSmartAccount transaction.",
    undefined,
    types.int,
  )
  .setAction(async function ({ safeSmartAccountImplAddress, owners, threshold }, { ethers, run }) {
    // Compile contracts from external dependencies (e.g., Safe Smart Account).
    // These are temporarily stored by `hardhat-dependency-compiler`.
    // See the `dependencyCompiler` field in `hardhat.config.ts` for configuration details.
    await run("compile:specific", { contract: "hardhat-dependency-compiler" });

    // Get a deployer wallet
    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

    // Get the Safe implementation contract
    const safeImplementation = await ethers.getContractAt("Safe", safeSmartAccountImplAddress);

    // Deploy a new SafeProxyFactory contract
    const safeProxyFactoryFactory = await ethers.getContractFactory("SafeProxyFactory", deployer);
    const safeProxyFactory = await safeProxyFactoryFactory.deploy();

    // Prepare the setup transaction data
    const to = ethers.ZeroAddress; // Contract address for optional delegate call.
    const data = "0x"; // Data payload for optional delegate call.
    const fallbackHandler = ethers.ZeroAddress; // Handler for fallback calls to this contract.
    const paymentToken = ethers.ZeroAddress; // Token that should be used for the payment (0 is ETH).
    const payment = 0; // Value that should be paid.
    const paymentReceiver = ethers.ZeroAddress; // Address that should receive the payment (or 0 if tx.origin).

    // Encode the setup function data
    const safeData = safeImplementation.interface.encodeFunctionData("setup", [
      owners,
      threshold,
      to,
      data,
      fallbackHandler,
      paymentToken,
      payment,
      paymentReceiver,
    ]);

    // Create the Safe proxy factory
    const saltNonce = 0n;
    const txResponse = await safeProxyFactory.createProxyWithNonce(safeSmartAccountImplAddress, safeData, saltNonce);
    const txReceipt = await txResponse.wait();
    if (!txReceipt) {
      throw new Error("Create Safe proxy transaction receipt not found");
    }

    // Get the Safe proxy address from the ProxyCreation event
    const event = txReceipt.logs
      .filter((l: EventLog | Log) => l instanceof EventLog)
      .find((l: EventLog) => l.eventName === safeProxyFactory.getEvent("ProxyCreation").name);
    if (!event) {
      throw new Error("ProxyCreation event not found in transaction receipt");
    }
    const safeProxyAddress = event.args.proxy;

    if (safeProxyAddress === ethers.ZeroAddress) {
      throw new Error("Safe proxy address not found");
    }

    console.log(`Safe Smart Account proxy successfully deployed at address: ${safeProxyAddress}\n`);
  });

task(
  "task:transferGatewayOwnership",
  "Transfers ownership of the GatewayConfig contract to the passed Safe Smart Account address.",
)
  .addParam(
    "currentOwnerPrivateKey",
    "Private key of the current owner of the GatewayConfig contract.",
    undefined,
    types.string,
  )
  .addParam("safeSmartAccountAddress", "Address of the Safe Smart Account receiving ownership", undefined, types.string)
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ currentOwnerPrivateKey, safeSmartAccountAddress, useInternalProxyAddress }, { ethers }) {
    // Get the currentOwner wallet.
    const currentOwner = new Wallet(currentOwnerPrivateKey).connect(ethers.provider);

    if (useInternalProxyAddress) {
      const gatewayEnvFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);

      if (!fs.existsSync(gatewayEnvFilePath)) {
        throw new Error(`Environment file not found: ${gatewayEnvFilePath}`);
      }
      dotenv.config({ path: gatewayEnvFilePath, override: true });
    }

    // Get the GatewayConfig contract
    const gatewayConfigSnakeCase = pascalCaseToSnakeCase("GatewayConfig");
    const gatewayConfigAddressEnvVarName = `${gatewayConfigSnakeCase.toUpperCase()}_ADDRESS`;
    const gatewayConfigContractAddress = getRequiredEnvVar(gatewayConfigAddressEnvVarName);
    const gatewayConfigContract = await ethers.getContractAt("GatewayConfig", gatewayConfigContractAddress);

    // Step 1 - Transfer ownership of the contract to the Safe Smart Account.
    await gatewayConfigContract.connect(currentOwner).transferOwnership(safeSmartAccountAddress);

    console.log(
      `Ownership of Gateway at address ${gatewayConfigContractAddress} successfully transferred to Safe Smart Account at address: ${safeSmartAccountAddress}`,
    );
  });

task("task:acceptGatewayOwnership", "Accepts ownership of the GatewayConfig contract from the Safe Smart Account")
  .addParam("ownerPrivateKeys", "List of private keys of the owners of the Safe Smart Account.", undefined, types.json)
  .addParam(
    "safeSmartAccountAddress",
    "Address of the Safe Smart Account proxy accepting ownership.",
    undefined,
    types.string,
  )
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used.",
    false,
    types.boolean,
  )
  .setAction(async function ({ ownerPrivateKeys, safeSmartAccountAddress, useInternalProxyAddress }, { ethers, run }) {
    // Compile contracts from external dependencies (e.g., Safe Smart Account).
    // These are temporarily stored by `hardhat-dependency-compiler`.
    // See the `dependencyCompiler` field in `hardhat.config.ts` for configuration details.
    await run("compile:specific", { contract: "hardhat-dependency-compiler" });

    // Get the signers' wallets.
    const signers: Wallet[] = ownerPrivateKeys.map((ownerPrivateKey: string) =>
      new Wallet(ownerPrivateKey).connect(ethers.provider),
    );

    if (useInternalProxyAddress) {
      const gatewayEnvFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);

      if (!fs.existsSync(gatewayEnvFilePath)) {
        throw new Error(`Environment file not found: ${gatewayEnvFilePath}`);
      }
      dotenv.config({ path: gatewayEnvFilePath, override: true });
    }

    // Get the GatewayConfig contract
    const gatewayConfigSnakeCase = pascalCaseToSnakeCase("GatewayConfig");
    const gatewayConfigAddressEnvVarName = `${gatewayConfigSnakeCase.toUpperCase()}_ADDRESS`;
    const gatewayConfigContractAddress = getRequiredEnvVar(gatewayConfigAddressEnvVarName);
    const gatewayConfigContract = await ethers.getContractAt("GatewayConfig", gatewayConfigContractAddress);

    // Get the Safe Smart Account contract from the Safe factory
    const safeSmartAccount = await ethers.getContractAt("Safe", safeSmartAccountAddress);

    // Prepare the Safe transaction to accept ownership.
    const value = 0; // Ether value.
    const data = gatewayConfigContract.interface.encodeFunctionData("acceptOwnership"); // Data payload for the transaction.
    const operation = OperationType.Call; // Operation type.
    const safeTxGas = 0; // Gas that should be used for the safe transaction.
    const baseGas = 0; // Gas costs for that are independent of the transaction execution(e.g. base transaction fee, signature check, payment of the refund)
    const gasPrice = 0; // Maximum gas price that should be used for this transaction.
    const gasToken = ethers.ZeroAddress; // Token address (or 0 if ETH) that is used for the payment.
    const refundReceiver = ethers.ZeroAddress; // Address of receiver of gas payment (or 0 if tx.origin).
    const nonce = await safeSmartAccount.nonce();

    // Get the transaction hash for the Safe transaction.
    const transactionHash = await safeSmartAccount.getTransactionHash(
      gatewayConfigContractAddress,
      value,
      data,
      operation,
      safeTxGas,
      baseGas,
      gasPrice,
      gasToken,
      refundReceiver,
      nonce,
    );

    // Sort the signers by their addresses
    // Gnosis Safe requires signatures to be provided in ascending order of the signer addresses
    // for security and efficiency reasons. See https://docs.safe.global/advanced/smart-account-signatures.
    const signatures = await getSortedSignatures(signers, transactionHash);

    // Step 2 - Execute the Safe transaction to accept ownership.
    const execTransactionResponse = await safeSmartAccount.execTransaction(
      gatewayConfigContractAddress,
      value,
      data,
      operation,
      safeTxGas,
      baseGas,
      gasPrice,
      gasToken,
      refundReceiver,
      signatures,
    );
    await execTransactionResponse.wait();
    console.log(
      `Ownership of Gateway at address ${gatewayConfigContractAddress} successfully accepted by the Safe Smart Account at address: ${safeSmartAccountAddress}`,
    );
  });

task("task:updateGatewayPauser", "Updates the pauser of the GatewayConfig contract using a Safe Smart Account")
  .addParam("ownerPrivateKeys", "List of private keys of the owners of the Safe Smart Account.", undefined, types.json)
  .addParam(
    "safeSmartAccountAddress",
    "Address of the Safe Smart Account proxy updating the pauser.",
    undefined,
    types.string,
  )
  .addParam("newPauser", "Address of the new pauser to be set in the GatewayConfig contract.", undefined, types.string)
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used.",
    false,
    types.boolean,
  )
  .setAction(async function (
    { ownerPrivateKeys, safeSmartAccountAddress, useInternalProxyAddress, newPauser },
    { ethers, run },
  ) {
    // Compile contracts from external dependencies (e.g., Safe Smart Account).
    // These are temporarily stored by `hardhat-dependency-compiler`.
    // See the `dependencyCompiler` field in `hardhat.config.ts` for configuration details.
    await run("compile:specific", { contract: "hardhat-dependency-compiler" });

    // Get the signers' wallets.
    const signers: Wallet[] = ownerPrivateKeys.map((ownerPrivateKey: string) =>
      new Wallet(ownerPrivateKey).connect(ethers.provider),
    );

    if (useInternalProxyAddress) {
      const gatewayEnvFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);

      if (!fs.existsSync(gatewayEnvFilePath)) {
        throw new Error(`Environment file not found: ${gatewayEnvFilePath}`);
      }
      dotenv.config({ path: gatewayEnvFilePath, override: true });
    }

    // Get the GatewayConfig contract from the Ownable2StepUpgradeable factory
    const gatewayConfigSnakeCase = pascalCaseToSnakeCase("GatewayConfig");
    const gatewayConfigAddressEnvVarName = `${gatewayConfigSnakeCase.toUpperCase()}_ADDRESS`;
    const gatewayConfigContractAddress = getRequiredEnvVar(gatewayConfigAddressEnvVarName);
    const gatewayConfigContract = await ethers.getContractAt("GatewayConfig", gatewayConfigContractAddress);

    // Get the Safe Smart Account contract from the Safe factory
    const safeSmartAccount = await ethers.getContractAt("Safe", safeSmartAccountAddress);

    // Prepare the Safe transaction to update the pauser.
    const value = 0; // Ether value.
    const data = gatewayConfigContract.interface.encodeFunctionData("updatePauser", [newPauser]); // Data payload for the transaction.
    const operation = OperationType.Call; // Operation type.
    const safeTxGas = 0; // Gas that should be used for the safe transaction.
    const baseGas = 0; // Gas costs for that are independent of the transaction execution(e.g. base transaction fee, signature check, payment of the refund)
    const gasPrice = 0; // Maximum gas price that should be used for this transaction.
    const gasToken = ethers.ZeroAddress; // Token address (or 0 if ETH) that is used for the payment.
    const refundReceiver = ethers.ZeroAddress; // Address of receiver of gas payment (or 0 if tx.origin).
    const nonce = await safeSmartAccount.nonce();

    // Get the transaction hash for the Safe transaction.
    const transactionHash = await safeSmartAccount.getTransactionHash(
      gatewayConfigContractAddress,
      value,
      data,
      operation,
      safeTxGas,
      baseGas,
      gasPrice,
      gasToken,
      refundReceiver,
      nonce,
    );

    // Sort the signers by their addresses
    // Gnosis Safe requires signatures to be provided in ascending order of the signer addresses
    // for security and efficiency reasons. See https://docs.safe.global/advanced/smart-account-signatures.
    const signatures = await getSortedSignatures(signers, transactionHash);

    // Execute the Safe transaction to update the pauser.
    const execTransactionResponse = await safeSmartAccount.execTransaction(
      gatewayConfigContractAddress,
      value,
      data,
      operation,
      safeTxGas,
      baseGas,
      gasPrice,
      gasToken,
      refundReceiver,
      signatures,
    );
    await execTransactionResponse.wait();
    console.log(
      `Pauser of Gateway at address ${gatewayConfigContractAddress} successfully updated to address: ${newPauser}`,
    );
  });
