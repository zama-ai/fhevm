import dotenv from "dotenv";
import fs from "fs";
import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";
import path from "path";

function writeEnvFile(filePath: string, content: string): void {
  try {
    fs.writeFileSync(filePath, content, { flag: "w" });
    console.log(`Content written to ${filePath} successfully!`);
  } catch (err) {
    console.error(`Failed to write to ${filePath}:`, err);
  }
}

// Deploy the HTTPZ contract
task("task:deployHttpz")
  .addParam("deployerPrivateKey", "The deployer private key")
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);
    const HTTPZ = await ethers.getContractFactory("HTTPZ", deployer);
    const httpz = await HTTPZ.deploy();

    // Wait for the deployment to be confirmed
    await httpz.waitForDeployment();

    const httpzAddress = await httpz.getAddress();

    console.log("HTTPZ contract deployed to:", httpzAddress);

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

// // Deploy the CiphertextStorage contract
// task("task:deployCiphertextStorage")
//   .addParam("deployerPrivateKey", "The deployer private key", types.string)
//   .setAction(async function (taskArguments: TaskArguments, { ethers }) {
//     const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);

//     const parsedEnvHttpz = dotenv.parse(fs.readFileSync('addresses/.env.httpz'));
//     const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

//     const CiphertextStorage = await ethers.getContractFactory("CiphertextStorage", deployer);
//     const ciphertextStorage = await CiphertextStorage.deploy(httpzAddress);

//     // Wait for the deployment to be confirmed
//     await ciphertextStorage.waitForDeployment();

//     const ciphertextStorageAddress = await ciphertextStorage.getAddress();

//     console.log("CiphertextStorage contract deployed to:", ciphertextStorageAddress);

// });

// Deploy the ACLManager contract
// task("task:deployAclManager")
//   .addParam("deployerPrivateKey", "The deployer private key", types.string)
//   .setAction(async function (taskArguments: TaskArguments, { ethers }) {
//     const parsedEnvHttpz = dotenv.parse(fs.readFileSync('addresses/.env.httpz'));
//     const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

//     const dummyPaymentManagerAddress = "0x0000000000000000000000000000000000000000";

//     const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);
//     const ACLManager = await ethers.getContractFactory("ACLManager", deployer);
//     const aclManager = await ACLManager.deploy(httpzAddress, dummyPaymentManagerAddress);

//     // Wait for the deployment to be confirmed
//     await aclManager.waitForDeployment();

//     const aclManagerAddress = await aclManager.getAddress();

//     console.log("ACLManager contract deployed to:", aclManagerAddress);

// });

// Deploy the DecryptionManager contract
// task("task:deployDecryptionManager")
//   .addParam("deployerPrivateKey", "The deployer private key", types.string)
//   .addParam("httpzAddress", "The HTTPZ contract address")
//   .addParam("paymentManagerAddress", "The payment manager address")
//   .addParam("aclManagerAddress", "The ACLManager contract address")
//   .setAction(async function (taskArguments: TaskArguments, { ethers }) {
//     const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);

//     const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
//     const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;

//     const dummyPaymentManagerAddress = "0x0000000000000000000000000000000000000000";
//     const dummyAclManagerAddress = "0x0000000000000000000000000000000000000000";

//     const DecryptionManager = await ethers.getContractFactory("DecryptionManager", deployer);
//     const decryptionManager = await DecryptionManager.deploy(
//       httpzAddress,
//       dummyPaymentManagerAddress,
//       dummyAclManagerAddress,
//     );

//     // Wait for the deployment to be confirmed
//     await decryptionManager.waitForDeployment();

//     const decryptionManagerAddress = await decryptionManager.getAddress();

//     console.log("DecryptionManager contract deployed to:", decryptionManagerAddress);
//   });

// Initialize the HTTPZ contract
task("task:initHttpz")
  .addParam("deployerPrivateKey", "The deployer private key")
  .addParam("adminPrivateKey", "The admin private key")
  .addParam("protocolMetadata", "The protocol metadata")
  .addParam("adminAddresses", "The admin addresses")
  .addParam("kmsThreshold", "The KMS threshold")
  .addParam("kmsNodes", "The KMS nodes")
  .addParam("coprocessors", "The coprocessors")
  .addParam("layer1Network", "The L1 network")
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);
    const admin = new ethers.Wallet(taskArguments.adminPrivateKey).connect(ethers.provider);

    // Load the HTTPZ contract
    const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
    const httpzAddress = parsedEnvHttpz.HTTPZ_ADDRESS;
    const HTTPZ = await ethers.getContractFactory("HTTPZ", deployer);
    const httpz = (await HTTPZ.attach(httpzAddress)) as any;

    // Parse the protocol metadata
    const metadata = JSON.parse(taskArguments.protocolMetadata);
    // Parse the admin addresses
    const adminAddresses = JSON.parse(taskArguments.adminAddresses);
    // Parse the KMS nodes
    const kmsNodes = JSON.parse(taskArguments.kmsNodes);
    // Parse the coprocessors
    const coprocessors = JSON.parse(taskArguments.coprocessors);

    // Initialize the HTTPZ contract
    const initializeTx = await httpz
      .connect(deployer)
      .initialize(metadata, adminAddresses, taskArguments.kmsThreshold, kmsNodes, coprocessors);
    await initializeTx.wait();
    console.log("HTTPZ contract initialized successfully!");
    console.log("Protocol metadata:", metadata);
    console.log("Admin addresses:", adminAddresses, "\n");
    console.log("KMS threshold:", taskArguments.kmsThreshold, "\n");
    console.log("KMS nodes:", kmsNodes, "\n");
    console.log("Adding coprocessors:", coprocessors, "\n");

    // Parse the L1 network
    const layer1Network = JSON.parse(taskArguments.layer1Network);

    // Add the L1 network
    const addLayer1NetworkTx = await httpz.connect(admin).addNetwork(layer1Network);
    await addLayer1NetworkTx.wait();
    console.log("L1 network added successfully!");
    console.log("L1 network:", layer1Network, "\n");
  });
