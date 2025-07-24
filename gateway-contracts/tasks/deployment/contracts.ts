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

// Helper function to deploy a contract implementation to its proxy
async function deployContractImplementation(
  name: string,
  hre: HardhatRuntimeEnvironment,
  initializeArgs?: unknown[],
): Promise<string> {
  const { ethers, upgrades } = hre;

  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
  const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

  // Get contract factories
  const proxyImplementation = await ethers.getContractFactory("EmptyUUPSProxy", deployer);
  const newImplem = await ethers.getContractFactory(name, deployer);

  const envFilePath = path.join(ADDRESSES_DIR, `.env.gateway`);
  if (!fs.existsSync(envFilePath)) {
    throw new Error(`Environment file not found: ${envFilePath}`);
  }
  dotenv.config({ path: envFilePath, override: true });

  // Determine env variable name for the proxy contract address
  const nameSnakeCase = pascalCaseToSnakeCase(name);
  const addressEnvVarName = `${nameSnakeCase.toUpperCase()}_ADDRESS`;

  // Get the proxy address
  const proxyAddress = getRequiredEnvVar(addressEnvVarName);
  if (!proxyAddress) {
    throw new Error(`Address variable ${addressEnvVarName} not found in ${envFilePath}`);
  }

  // Force import
  const proxy = await upgrades.forceImport(proxyAddress, proxyImplementation);

  // Set the upgrade options
  const upgradeOptions = {
    call: {
      fn: "initializeFromEmptyProxy",
      args: [] as unknown[],
    },
  };
  if (initializeArgs !== undefined && initializeArgs.length > 0) {
    upgradeOptions.call.args = initializeArgs;
  }

  // Upgrade the proxy
  await upgrades.upgradeProxy(proxy, newImplem, upgradeOptions);

  console.log(`${name} implementation set successfully at address: ${proxyAddress}\n`);
  return proxyAddress;
}

async function deployMultisigSmartAccount(
  name: string,
  { run, ethers }: HardhatRuntimeEnvironment,
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
  if (owners.length === 0) {
    owners.push(await deployer.getAddress());
  }
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

  await run("task:setContractAddress", {
    name,
    address: safeProxyAddress,
  });
}

// Deploy the GatewayConfig contract
task("task:deployGatewayConfig").setAction(async function (_, hre) {
  // Parse the protocol metadata
  const protocolMetadata = {
    name: getRequiredEnvVar("PROTOCOL_NAME"),
    website: getRequiredEnvVar("PROTOCOL_WEBSITE"),
  };

  // Parse the pauser smart account address
  const pauserAddress = getRequiredEnvVar(`PAUSER_ADDRESS`);

  // Parse the MPC threshold
  const mpcThreshold = getRequiredEnvVar("MPC_THRESHOLD");

  // Parse the decryption response thresholds
  const publicDecryptionThreshold = getRequiredEnvVar("PUBLIC_DECRYPTION_THRESHOLD");
  const userDecryptionThreshold = getRequiredEnvVar("USER_DECRYPTION_THRESHOLD");

  // Parse the KMS nodes
  const numKmsNodes = parseInt(getRequiredEnvVar("NUM_KMS_NODES"));
  const kmsNodes = [];
  for (let idx = 0; idx < numKmsNodes; idx++) {
    kmsNodes.push({
      txSenderAddress: getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`),
      ipAddress: getRequiredEnvVar(`KMS_NODE_IP_ADDRESS_${idx}`),
    });
  }

  // Parse the coprocessors
  const numCoprocessors = parseInt(getRequiredEnvVar("NUM_COPROCESSORS"));
  const coprocessors = [];
  for (let idx = 0; idx < numCoprocessors; idx++) {
    coprocessors.push({
      txSenderAddress: getRequiredEnvVar(`COPROCESSOR_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`),
      s3BucketUrl: getRequiredEnvVar(`COPROCESSOR_S3_BUCKET_URL_${idx}`),
    });
  }

  // Parse the custodians
  const numCustodians = parseInt(getRequiredEnvVar("NUM_CUSTODIANS"));
  const custodians = [];
  for (let idx = 0; idx < numCustodians; idx++) {
    custodians.push({
      txSenderAddress: getRequiredEnvVar(`CUSTODIAN_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`CUSTODIAN_SIGNER_ADDRESS_${idx}`),
      encryptionKey: getRequiredEnvVar(`CUSTODIAN_ENCRYPTION_KEY_${idx}`),
    });
  }

  console.log("Pauser address:", pauserAddress);
  console.log("Protocol metadata:", protocolMetadata);
  console.log("MPC threshold:", mpcThreshold);
  console.log("Public decryption threshold:", publicDecryptionThreshold);
  console.log("User decryption threshold:", userDecryptionThreshold);
  console.log("KMS nodes:", kmsNodes);
  console.log("Coprocessors:", coprocessors);
  console.log("Custodians:", custodians);

  await deployContractImplementation("GatewayConfig", hre, [
    pauserAddress,
    protocolMetadata,
    mpcThreshold,
    publicDecryptionThreshold,
    userDecryptionThreshold,
    kmsNodes,
    coprocessors,
    custodians,
  ]);
});

// Deploy the InputVerification contract
task("task:deployInputVerification").setAction(async function (_, hre) {
  await deployContractImplementation("InputVerification", hre);
});

// Deploy the KmsManagement contract
task("task:deployKmsManagement").setAction(async function (_, hre) {
  const fheParamsName = getRequiredEnvVar("FHE_PARAMS_NAME");
  const fheParamsDigest = getRequiredEnvVar("FHE_PARAMS_DIGEST");

  console.log("FHE params name:", fheParamsName);
  console.log("FHE params digest:", fheParamsDigest);

  await deployContractImplementation("KmsManagement", hre, [fheParamsName, fheParamsDigest]);
});

// Deploy the CiphertextCommits contract
task("task:deployCiphertextCommits").setAction(async function (_, hre) {
  await deployContractImplementation("CiphertextCommits", hre);
});

// Deploy the MultichainAcl contract
task("task:deployMultichainAcl").setAction(async function (_, hre) {
  await deployContractImplementation("MultichainAcl", hre);
});

// Deploy the Decryption contract
task("task:deployDecryption").setAction(async function (_, hre) {
  await deployContractImplementation("Decryption", hre);
});

task("task:deployOwnerSmartAccount")
  .addOptionalParam("owners", "List of addresses that control the OwnerSmartAccount.", [], types.json)
  .addOptionalParam("threshold", "Number of required confirmations for a OwnerSmartAccount transaction.", 1, types.int)
  .setAction(async function ({ owners, threshold }, hre) {
    // Compile contracts from external dependencies (e.g., Safe Smart Account).
    // These are temporarily stored by `hardhat-dependency-compiler`.
    // See the `dependencyCompiler` field in `hardhat.config.ts` for configuration details.
    await hre.run("compile:specific", { contract: "hardhat-dependency-compiler" });

    await deployMultisigSmartAccount("OwnerSmartAccount", hre, owners, threshold);
  });

task("task:deployPauserSmartAccount")
  .addOptionalParam("owners", "List of addresses that control the PauserSmartAccount.", [], types.json)
  .addOptionalParam("threshold", "Number of required confirmations for a PauserSmartAccount transaction.", 1, types.int)
  .setAction(async function ({ owners, threshold }, hre) {
    // Compile contracts from external dependencies (e.g., Safe Smart Account).
    // These are temporarily stored by `hardhat-dependency-compiler`.
    // See the `dependencyCompiler` field in `hardhat.config.ts` for configuration details.
    await hre.run("compile:specific", { contract: "hardhat-dependency-compiler" });

    await deployMultisigSmartAccount("PauserSmartAccount", hre, owners, threshold);
  });

task("task:transferOwnershipsToOwnerSmartAccount")
  .addOptionalParam(
    "useInternalProxyAddress",
    "If proxy address from the /addresses directory should be used",
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers }) {
    // Get a deployer wallet.
    const deployerPrivateKey = getRequiredEnvVar("DEPLOYER_PRIVATE_KEY");
    const deployer = new Wallet(deployerPrivateKey).connect(ethers.provider);

    const contracts = [
      "GatewayConfig",
      "InputVerification",
      "KmsManagement",
      "CiphertextCommits",
      "MultichainAcl",
      "Decryption",
    ];

    let ownerSmartAccountAddress: string;
    const nameSnakeCase = pascalCaseToSnakeCase("OwnerSmartAccount");
    const addressEnvVarName = `${nameSnakeCase.toUpperCase()}_ADDRESS`;
    if (useInternalProxyAddress) {
      const envFilePath = path.join(ADDRESSES_DIR, `.env.${nameSnakeCase}`);

      if (!fs.existsSync(envFilePath)) {
        throw new Error(`Environment file not found: ${envFilePath}`);
      }
      const parsedEnv = dotenv.parse(fs.readFileSync(envFilePath));
      ownerSmartAccountAddress = parsedEnv[addressEnvVarName];
      if (!ownerSmartAccountAddress) {
        throw new Error(`Address variable ${addressEnvVarName} not found in ${envFilePath}`);
      }
    } else {
      ownerSmartAccountAddress = getRequiredEnvVar(addressEnvVarName);
    }
    const ownerSmartAccount = await ethers.getContractAt("Safe", ownerSmartAccountAddress, deployer);

    // Setup the signers for the Safe transaction.
    const signers = [deployer];

    // Get the addresses of the signers.
    const addresses = await Promise.all(signers.map((signer) => signer.getAddress()));

    // Sort the signers by their addresses. The `Safe.execTransaction` expects that the signatures
    // are sorted by owner address. This is required to easily validate no confirmation duplicates exist.
    const sortedSigners = signers.sort((a, b) => {
      const addressA = addresses[signers.indexOf(a)];
      const addressB = addresses[signers.indexOf(b)];
      return addressA.localeCompare(addressB, "en", { sensitivity: "base" });
    });

    console.log(`Transferring ownerships to OwnerSmartAccount at address: ${ownerSmartAccountAddress}`);

    for (const contractName of contracts) {
      let contractAddress: string;

      const nameSnakeCase = pascalCaseToSnakeCase(contractName);
      const addressEnvVarName = `${nameSnakeCase.toUpperCase()}_ADDRESS`;
      if (useInternalProxyAddress) {
        const envFilePath = path.join(ADDRESSES_DIR, `.env.${nameSnakeCase}`);

        if (!fs.existsSync(envFilePath)) {
          throw new Error(`Environment file not found: ${envFilePath}`);
        }
        const parsedEnv = dotenv.parse(fs.readFileSync(envFilePath));
        contractAddress = parsedEnv[addressEnvVarName];
        if (!contractAddress) {
          throw new Error(`Address variable ${addressEnvVarName} not found in ${envFilePath}`);
        }
      } else {
        contractAddress = getRequiredEnvVar(addressEnvVarName);
      }

      // Step 1: Transfer ownership of the contract to the OwnerSmartAccount.
      const contract = await ethers.getContractAt("Ownable2StepUpgradeable", contractAddress, deployer);
      await contract.connect(deployer).transferOwnership(ownerSmartAccountAddress);

      // Prepare the Safe transaction to accept ownership.
      const value = 0; // Ether value.
      const data = contract.interface.encodeFunctionData("acceptOwnership"); // Data payload for the transaction.
      const operation = OperationType.Call; // Operation type.
      const safeTxGas = 0; // Gas that should be used for the safe transaction.
      const baseGas = 0; // Gas costs for that are independent of the transaction execution(e.g. base transaction fee, signature check, payment of the refund)
      const gasPrice = 0; // Maximum gas price that should be used for this transaction.
      const gasToken = ethers.ZeroAddress; // Token address (or 0 if ETH) that is used for the payment.
      const refundReceiver = ethers.ZeroAddress; // Address of receiver of gas payment (or 0 if tx.origin).
      const nonce = await ownerSmartAccount.nonce();

      // Get the transaction hash for the Safe transaction.
      const transactionHash = await ownerSmartAccount.getTransactionHash(
        contractAddress,
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

      // Sign the transaction hash with each signer.
      let signatureBytes = "0x";
      for (let i = 0; i < sortedSigners.length; i++) {
        const signedMessage = await sortedSigners[i].signMessage(bytesDataHash);
        const flatSig = signedMessage.replace(/1b$/, "1f").replace(/1c$/, "20");
        signatureBytes += flatSig.slice(2);
      }

      // Step 2: Execute the Safe transaction to accept ownership.
      const execTransactionResponse = await ownerSmartAccount.execTransaction(
        contractAddress,
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
    }
  });

// Deploy all the contracts
task("task:deployAllGatewayContracts").setAction(async function (_, hre) {
  // Deploy the EmptyUUPS proxy contracts
  await hre.run("task:deployEmptyUUPSProxies");

  // Compile the implementation contracts
  // The deployEmptyUUPSProxies task has generated the contracts' addresses in `addresses/*.sol`.
  // Contracts thus need to be compiled after deploying the EmptyUUPS proxy contracts in order to
  // use these addresses. Otherwise, irrelevant addresses will be used and, although deployment will
  // succeed, most transactions made to the contracts will revert as inter-contract calls will fail.
  await hre.run("compile:specific", { contract: "contracts" });

  console.log("Deploy GatewayConfig contract:");
  await hre.run("task:deployGatewayConfig");

  console.log("Deploy InputVerification contract:");
  await hre.run("task:deployInputVerification");

  console.log("Deploy KmsManagement contract:");
  await hre.run("task:deployKmsManagement");

  console.log("Deploy CiphertextCommits contract:");
  await hre.run("task:deployCiphertextCommits");

  console.log("Deploy MultichainAcl contract:");
  await hre.run("task:deployMultichainAcl");

  console.log("Deploy Decryption contract:");
  await hre.run("task:deployDecryption");

  console.log("Contract deployment done!");
});
