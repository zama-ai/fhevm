import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";

// Deploy all contracts in a single task
// TODO: Separate into multiple tasks when addresses are fixed (using proxy pattern)
task("task:deployContracts")
  .addParam("deployerPrivateKey", "The deployer private key")
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const deployer = new ethers.Wallet(taskArguments.deployerPrivateKey).connect(ethers.provider);
    const HTTPZ = await ethers.getContractFactory("HTTPZ", deployer);
    const httpz = await HTTPZ.deploy();

    // Wait for the deployment to be confirmed
    await httpz.waitForDeployment();

    const httpzAddress = await httpz.getAddress();

    console.log("HTTPZ contract deployed to:", httpzAddress);

    const dummyPaymentManagerAddress = "0x1234567890abcdef1234567890abcdef12345678";

    // Deploy ZKPoKManager contract
    const ZKPoKManager = await ethers.getContractFactory("ZKPoKManager", deployer);
    const zkpokManager = await ZKPoKManager.deploy(httpzAddress, dummyPaymentManagerAddress);

    // Wait for the deployment to be confirmed
    await zkpokManager.waitForDeployment();

    const zkpokManagerAddress = await zkpokManager.getAddress();

    console.log("ZKPoKManager contract deployed to:", zkpokManagerAddress);

    // TODO: Deploy CiphertextStorage contract
    // const CiphertextStorage = await ethers.getContractFactory("CiphertextStorage", deployer);
    // const ciphertextStorage = await CiphertextStorage.deploy(httpzAddress);

    // // Wait for the deployment to be confirmed
    // await ciphertextStorage.waitForDeployment();

    // const ciphertextStorageAddress = await ciphertextStorage.getAddress();

    // console.log("CiphertextStorage contract deployed to:", ciphertextStorageAddress);

    // TODO: Deploy ACLManager contract
    // const ACLManager = await ethers.getContractFactory("ACLManager", deployer);
    // const aclManager = await ACLManager.deploy(httpzAddress, ciphertextStorageAddress);

    // // Wait for the deployment to be confirmed
    // await aclManager.waitForDeployment();

    // const aclManagerAddress = await aclManager.getAddress();

    // console.log("ACLManager contract deployed to:", aclManagerAddress);

    // TODO: Deploy DecryptionManager contract
    // const DecryptionManager = await ethers.getContractFactory("DecryptionManager", deployer);
    // const decryptionManager = await DecryptionManager.deploy(
    //   httpzAddress,
    //   dummyPaymentManagerAddress,
    //   aclManagerAddress,
    // );

    // // Wait for the deployment to be confirmed
    // await decryptionManager.waitForDeployment();

    // const decryptionManagerAddress = await decryptionManager.getAddress();

    // console.log("DecryptionManager contract deployed to:", decryptionManagerAddress);
  });
