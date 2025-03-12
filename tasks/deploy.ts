import dotenv from "dotenv";
import fs from "fs";
import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";
import path from "path";

// Get the required environment variable, throw an error if it's not set
// We only check if the variable is set, not if it's empty
function getRequiredEnvVar(name: string): string {
  if (!(name in process.env)) {
    throw new Error(`"${name}" env variable is not set`);
  }
  return process.env[name]!;
}

// Write the content to a file
function writeEnvFile(filePath: string, content: string): void {
  try {
    fs.writeFileSync(filePath, content, { flag: "w" });
    console.log(`Content written to ${filePath} successfully!\n`);
  } catch (err) {
    console.error(`Failed to write to ${filePath}:`, err);
  }
}

// Deploy the HTTPZ contract
task("task:deployHttpz")
  .addParam("deployerPrivateKey", "The deployer private key")
  .addParam("numAdmins", "The number of admins")
  .addParam("numKmsNodes", "The number of KMS nodes")
  .addParam("numCoprocessors", "The number of coprocessors")
  .addParam("numNetworks", "The number of L1 networks")
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);

    // Parse the protocol metadata
    const protocolMetadata = {
      name: getRequiredEnvVar("PROTOCOL_NAME"),
      website: getRequiredEnvVar("PROTOCOL_WEBSITE"),
    };

    // Parse the admin addresses (index starts from 1)
    const adminAddresses = [];
    for (let idx = 1; idx <= taskArguments.numAdmins; idx++) {
      adminAddresses.push(getRequiredEnvVar(`ADMIN_ADDRESS_${idx}`));
    }

    // Parse the KMS threshold
    const kmsThreshold = getRequiredEnvVar("KMS_THRESHOLD");

    // Parse the KMS nodes (index starts from 1)
    const kmsNodes = [];
    for (let idx = 1; idx <= taskArguments.numKmsNodes; idx++) {
      kmsNodes.push({
        connectorAddress: getRequiredEnvVar(`KMS_NODE_ADDRESS_${idx}`),
        identity: getRequiredEnvVar(`KMS_NODE_IDENTITY_${idx}`),
        ipAddress: getRequiredEnvVar(`KMS_NODE_IP_ADDRESS_${idx}`),
        daUrl: getRequiredEnvVar(`KMS_NODE_DA_URL_${idx}`),
      });
    }

    // Parse the coprocessors (index starts from 1)
    const coprocessors = [];
    for (let idx = 1; idx <= taskArguments.numCoprocessors; idx++) {
      coprocessors.push({
        transactionSenderAddress: getRequiredEnvVar(`COPROCESSOR_ADDRESS_${idx}`),
        identity: getRequiredEnvVar(`COPROCESSOR_IDENTITY_${idx}`),
        daUrl: getRequiredEnvVar(`COPROCESSOR_DA_URL_${idx}`),
        s3BucketUrl: getRequiredEnvVar(`COPROCESSOR_S3_BUCKET_URL_${idx}`),
      });
    }

    // Parse the L1 network (index starts from 1)
    const layer1Networks = [];
    for (let idx = 1; idx <= taskArguments.numNetworks; idx++) {
      layer1Networks.push({
        chainId: getRequiredEnvVar(`NETWORK_CHAIN_ID_${idx}`),
        httpzExecutor: getRequiredEnvVar(`NETWORK_HTTPZ_EXECUTOR_${idx}`),
        aclAddress: getRequiredEnvVar(`NETWORK_ACL_ADDRESS_${idx}`),
        name: getRequiredEnvVar(`NETWORK_NAME_${idx}`),
        website: getRequiredEnvVar(`NETWORK_WEBSITE_${idx}`),
      });
    }

    const HTTPZ = await ethers.getContractFactory("HTTPZ", deployer);
    const httpz = await HTTPZ.deploy(
      protocolMetadata,
      adminAddresses,
      kmsThreshold,
      kmsNodes,
      coprocessors,
      layer1Networks,
    );

    // Wait for the deployment to be confirmed
    await httpz.waitForDeployment();

    const httpzAddress = await httpz.getAddress();

    console.log("HTTPZ contract deployed to:", httpzAddress);
    console.log("Protocol metadata:", protocolMetadata);
    console.log("Admin addresses:", adminAddresses, "\n");
    console.log("KMS threshold:", kmsThreshold, "\n");
    console.log("KMS nodes:", kmsNodes, "\n");
    console.log("Coprocessors:", coprocessors, "\n");
    console.log("L1 networks:", layer1Networks, "\n");

    // Save the HTTPZ address to the .env.httpz file
    const envFilePath = path.join(__dirname, "../addresses/.env.httpz");
    const content = `HTTPZ_ADDRESS=${httpzAddress}`;
    writeEnvFile(envFilePath, content);
  });

// Deploy the ZKPoKManager contract
task("task:deployZkpokManager")
  .addParam("deployerPrivateKey", "The deployer private key")
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
    const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

    const dummyPaymentManagerAddress = "0x0000000000000000000000000000000000000000";

    const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);

    // Deploy ZKPoKManager contract
    const ZKPoKManager = await ethers.getContractFactory("ZKPoKManager", deployer);
    const zkpokManager = await ZKPoKManager.deploy(httpzAddress, dummyPaymentManagerAddress);

    // Wait for the deployment to be confirmed
    await zkpokManager.waitForDeployment();

    const zkpokManagerAddress = await zkpokManager.getAddress();

    console.log("ZKPoKManager contract deployed to:", zkpokManagerAddress);

    // Save the ZKPoKManager address to the .env.zkpok_manager file
    const envFilePath = path.join(__dirname, "../addresses/.env.zkpok_manager");
    const content = `ZKPOK_MANAGER_ADDRESS=${zkpokManagerAddress}`;
    writeEnvFile(envFilePath, content);
  });

// Deploy the KeyManager contract
task("task:deployKeyManager")
  .addParam("deployerPrivateKey", "The deployer private key")
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);

    const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
    const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

    const fheParamsName = getRequiredEnvVar("FHE_PARAMS_NAME");
    const fheParamsDigest = getRequiredEnvVar("FHE_PARAMS_DIGEST");

    const KeyManager = await ethers.getContractFactory("KeyManager", deployer);
    const keyManager = await KeyManager.deploy(httpzAddress, fheParamsName, fheParamsDigest);

    // Wait for the deployment to be confirmed
    await keyManager.waitForDeployment();

    const keyManagerAddress = await keyManager.getAddress();

    console.log("KeyManager contract deployed to:", keyManagerAddress);

    const envFilePath = path.join(__dirname, "../addresses/.env.key_manager");
    const content = `KEY_MANAGER_ADDRESS=${keyManagerAddress}`;
    writeEnvFile(envFilePath, content);
  });

// Deploy the CiphertextManager contract
task("task:deployCiphertextManager")
  .addParam("deployerPrivateKey", "The deployer private key")
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);

    const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
    const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

    const parsedEnvKeyManager = dotenv.parse(fs.readFileSync("addresses/.env.key_manager"));
    const keyManagerAddress = parsedEnvKeyManager.KEY_MANAGER_ADDRESS;

    const CiphertextManager = await ethers.getContractFactory("CiphertextManager", deployer);
    const ciphertextManager = await CiphertextManager.deploy(httpzAddress, keyManagerAddress);

    // Wait for the deployment to be confirmed
    await ciphertextManager.waitForDeployment();

    const ciphertextManagerAddress = await ciphertextManager.getAddress();

    console.log("CiphertextManager contract deployed to:", ciphertextManagerAddress);

    const envFilePath = path.join(__dirname, "../addresses/.env.ciphertext_manager");
    const content = `CIPHERTEXT_MANAGER_ADDRESS=${ciphertextManagerAddress}`;
    writeEnvFile(envFilePath, content);
  });

// Deploy the ACLManager contract
task("task:deployAclManager")
  .addParam("deployerPrivateKey", "The deployer private key")
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);

    const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
    const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

    const parsedEnvCiphertextManager = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_manager"));
    const ciphertextManagerAddress = parsedEnvCiphertextManager.CIPHERTEXT_MANAGER_ADDRESS;

    const ACLManager = await ethers.getContractFactory("ACLManager", deployer);
    const aclManager = await ACLManager.deploy(httpzAddress, ciphertextManagerAddress);

    // Wait for the deployment to be confirmed
    await aclManager.waitForDeployment();

    const aclManagerAddress = await aclManager.getAddress();

    console.log("ACLManager contract deployed to:", aclManagerAddress);

    const envFilePath = path.join(__dirname, "../addresses/.env.acl_manager");
    const content = `ACL_MANAGER_ADDRESS=${aclManagerAddress}`;
    writeEnvFile(envFilePath, content);
  });

// Deploy the DecryptionManager contract
task("task:deployDecryptionManager")
  .addParam("deployerPrivateKey", "The deployer private key")
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);

    const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
    const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

    const parsedEnvAclManager = dotenv.parse(fs.readFileSync("addresses/.env.acl_manager"));
    const aclManagerAddress = parsedEnvAclManager.ACL_MANAGER_ADDRESS;

    const parsedEnvCiphertextManager = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_manager"));
    const ciphertextManagerAddress = parsedEnvCiphertextManager.CIPHERTEXT_MANAGER_ADDRESS;

    const dummyPaymentManagerAddress = "0x0000000000000000000000000000000000000000";

    const DecryptionManager = await ethers.getContractFactory("DecryptionManager", deployer);
    const decryptionManager = await DecryptionManager.deploy(
      httpzAddress,
      aclManagerAddress,
      ciphertextManagerAddress,
      dummyPaymentManagerAddress,
    );

    // Wait for the deployment to be confirmed
    await decryptionManager.waitForDeployment();

    const decryptionManagerAddress = await decryptionManager.getAddress();

    console.log("DecryptionManager contract deployed to:", decryptionManagerAddress);

    const envFilePath = path.join(__dirname, "../addresses/.env.decryption_manager");
    const content = `DECRYPTION_MANAGER_ADDRESS=${decryptionManagerAddress}`;
    writeEnvFile(envFilePath, content);
  });
