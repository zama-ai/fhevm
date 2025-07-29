import { OperationType } from "@safe-global/types-kit";
import dotenv from "dotenv";
import { EventLog, Log, Wallet } from "ethers";
import fs from "fs";
import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import path from "path";

import { ADDRESSES_DIR } from "../../hardhat.config";
import { getRequiredEnvVar } from "../utils/loadVariables";
import { pascalCaseToSnakeCase } from "../utils/stringOps";

async function deploySafeSmartAccount(
  name: string,
  { ethers }: HardhatRuntimeEnvironment,
  owners: string[],
  threshold: number,
) {
  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Deploy a new Safe contract
  const safeFactory = await ethers.getContractFactory("Safe", deployer);
  const safe = await safeFactory.deploy();
  const safeAddress = await safe.getAddress();

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
  const safeData = safe.interface.encodeFunctionData("setup", [
    owners,
    threshold,
    to,
    data,
    fallbackHandler,
    paymentToken,
    payment,
    paymentReceiver,
  ]);

  // Setup the Safe proxy factory
  const saltNonce = 0n;
  const txResponse = await safeProxyFactory.createProxyWithNonce(safeAddress, safeData, saltNonce);
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

  const envFilePath = path.join(ADDRESSES_DIR, ".env.gateway");
  const nameSnakeCase = pascalCaseToSnakeCase(name);
  const envContent = `${nameSnakeCase.toUpperCase()}_ADDRESS=${safeProxyAddress}\n`;

  // Ensure the ADDRESSES_DIR exists or create it
  fs.mkdirSync(ADDRESSES_DIR, { recursive: true });

  // Write the contract's address in the envFilePath file
  fs.appendFileSync(envFilePath, envContent, { encoding: "utf8", flag: "a" });
}

task("task:deployOwnerSafeSmartAccount")
  .addParam("owners", "List of addresses that control the OwnerSafeSmartAccount.", undefined, types.json)
  .addParam(
    "threshold",
    "Number of required confirmations for a OwnerSafeSmartAccount transaction.",
    undefined,
    types.int,
  )
  .setAction(async function ({ owners, threshold }, hre) {
    // Compile contracts from external dependencies (e.g., Safe Smart Account).
    // These are temporarily stored by `hardhat-dependency-compiler`.
    // See the `dependencyCompiler` field in `hardhat.config.ts` for configuration details.
    await hre.run("compile:specific", { contract: "hardhat-dependency-compiler" });

    await deploySafeSmartAccount("OwnerSafeSmartAccount", hre, owners, threshold);
  });

task("task:deployPauserSafeSmartAccount")
  .addParam("owners", "List of addresses that control the PauserSafeSmartAccount.", undefined, types.json)
  .addParam(
    "threshold",
    "Number of required confirmations for a PauserSafeSmartAccount transaction.",
    undefined,
    types.int,
  )
  .setAction(async function ({ owners, threshold }, hre) {
    // Compile contracts from external dependencies (e.g., Safe Smart Account).
    // These are temporarily stored by `hardhat-dependency-compiler`.
    // See the `dependencyCompiler` field in `hardhat.config.ts` for configuration details.
    await hre.run("compile:specific", { contract: "hardhat-dependency-compiler" });

    await deploySafeSmartAccount("PauserSafeSmartAccount", hre, owners, threshold);
  });

task("task:transferGatewayOwnership", "Transfers ownership of the GatewayConfig contract to the OwnerSafeSmartAccount")
  .addParam("currentOwnerPrivateKey", "Private key of the owner of the GatewayConfig contract", undefined, types.string)
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ currentOwnerPrivateKey, useInternalProxyAddress }, { ethers }) {
    // Get the currentOwner wallet.
    const currentOwner = new Wallet(currentOwnerPrivateKey).connect(ethers.provider);

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
    const gatewayConfigContract = await ethers.getContractAt("Ownable2StepUpgradeable", gatewayConfigContractAddress);

    // Get the OwnerSafeSmartAccount contract from the Safe factory
    const ownerSmartAccountSnakeCase = pascalCaseToSnakeCase("OwnerSafeSmartAccount");
    const ownerSmartAccountAddressEnvVarName = `${ownerSmartAccountSnakeCase.toUpperCase()}_ADDRESS`;
    const ownerSmartAccountAddress = getRequiredEnvVar(ownerSmartAccountAddressEnvVarName);

    console.log(`Transferring Gateway ownership to OwnerSafeSmartAccount at address: ${ownerSmartAccountAddress}`);

    // Step 1 - Transfer ownership of the contract to the OwnerSafeSmartAccount.
    await gatewayConfigContract.connect(currentOwner).transferOwnership(ownerSmartAccountAddress);

    console.log(
      `Ownership of Gateway at address ${gatewayConfigContractAddress} successfully transferred to OwnerSafeSmartAccount at address: ${ownerSmartAccountAddress}`,
    );
  });

task("task:acceptGatewayOwnership", "Accepts ownership of the GatewayConfig contract from the OwnerSafeSmartAccount")
  .addParam(
    "signerPrivateKey",
    "Private key of one of the owners of the OwnerSafeSmartAccount",
    undefined,
    types.string,
  )
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ signerPrivateKey, useInternalProxyAddress }, { ethers }) {
    // Get the signer wallet.
    const signer = new Wallet(signerPrivateKey).connect(ethers.provider);

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

    // Get the OwnerSafeSmartAccount contract from the Safe factory
    const ownerSmartAccountSnakeCase = pascalCaseToSnakeCase("OwnerSafeSmartAccount");
    const ownerSmartAccountAddressEnvVarName = `${ownerSmartAccountSnakeCase.toUpperCase()}_ADDRESS`;
    const ownerSmartAccountAddress = getRequiredEnvVar(ownerSmartAccountAddressEnvVarName);
    const ownerSmartAccount = await ethers.getContractAt("Safe", ownerSmartAccountAddress);

    console.log(`Accepting ownership from OwnerSafeSmartAccount at address: ${ownerSmartAccountAddress}`);

    // Prepare the Safe transaction to accept ownership.
    const value = 0; // Ether value.
    const data = gatewayConfigContract.interface.encodeFunctionData("acceptOwnership"); // Data payload for the transaction.
    const operation = OperationType.Call; // Operation type.
    const safeTxGas = 0; // Gas that should be used for the safe transaction.
    const baseGas = 0; // Gas costs for that are independent of the transaction execution(e.g. base transaction fee, signature check, payment of the refund)
    const gasPrice = 0; // Maximum gas price that should be used for this transaction.
    const gasToken = ethers.ZeroAddress; // Token address (or 0 if ETH) that is used for the payment.
    const refundReceiver = ethers.ZeroAddress; // Address of receiver of gas payment (or 0 if tx.origin).
    const nonce = await ownerSmartAccount.nonce();

    // Get the transaction hash for the Safe transaction.
    const transactionHash = await ownerSmartAccount.getTransactionHash(
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
    const bytesDataHash = ethers.getBytes(transactionHash);

    // Sign the transaction hash with the signer account.
    const signedMessage = await signer.signMessage(bytesDataHash);
    const flatSig = signedMessage.replace(/1b$/, "1f").replace(/1c$/, "20");
    const signatureBytes = "0x" + flatSig.slice(2);

    // Step 2 - Execute the Safe transaction to accept ownership.
    const execTransactionResponse = await ownerSmartAccount.execTransaction(
      gatewayConfigContractAddress,
      value,
      data,
      operation,
      safeTxGas,
      baseGas,
      gasPrice,
      gasToken,
      refundReceiver,
      signatureBytes,
    );
    await execTransactionResponse.wait();
    console.log(
      `Ownership of GatewayConfig at address ${gatewayConfigContractAddress} successfully accepted from OwnerSafeSmartAccount at address: ${ownerSmartAccountAddress}`,
    );
  });
