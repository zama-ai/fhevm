import MultiSendJson from "@safe-global/safe-contracts/build/artifacts/contracts/libraries/MultiSend.sol/MultiSend.json";
import { ContractFactory } from "ethers";
import { task } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

// Deploy the SafeSmartAccount contract
task("task:deploySafeL2").setAction(async function (
  _,
  { getNamedAccounts, ethers, deployments, network },
) {
  const { deploy } = deployments;
  const { deployer } = await getNamedAccounts();

  // Get the contract names
  const safeL2Name = "SafeL2";
  const proxyFactoryName = "SafeProxyFactory";

  // Deploy the Safe singleton implementation
  // Use the L2 version for easier debugging and because gas is cheap on gateway
  console.log(`Deploying ${safeL2Name}...`);
  const safeSingleton = await deploy(safeL2Name, {
    from: deployer,
    contract: safeL2Name,
    args: [],
    log: true,
    skipIfAlreadyDeployed: false,
  });

  // Deploy the proxy factory
  console.log(`Deploying ${proxyFactoryName}...`);
  const proxyFactory = await deploy(proxyFactoryName, {
    from: deployer,
    contract: proxyFactoryName,
    args: [],
    log: true,
    skipIfAlreadyDeployed: false,
  });

  // Get the environment variables for the Safe setup, using default values if not provided for
  // the optional parameters
  const owners = [deployer];
  const threshold = getRequiredEnvVar("SAFE_THRESHOLD");
  const to = getRequiredEnvVar("SAFE_TO", ethers.ZeroAddress);
  const data = getRequiredEnvVar("SAFE_DATA", "0x");
  const fallbackHandler = getRequiredEnvVar(
    "SAFE_FALLBACK_HANDLER",
    ethers.ZeroAddress,
  );
  const paymentToken = getRequiredEnvVar(
    "SAFE_PAYMENT_TOKEN",
    ethers.ZeroAddress,
  );
  const payment = getRequiredEnvVar("SAFE_PAYMENT", 0);
  const paymentReceiver = getRequiredEnvVar(
    "SAFE_PAYMENT_RECEIVER",
    ethers.ZeroAddress,
  );

  // Get contract instances
  const safeContract = await ethers.getContractAt(
    safeL2Name,
    safeSingleton.address,
  );
  const proxyFactoryContract = await ethers.getContractAt(
    proxyFactoryName,
    proxyFactory.address,
  );

  // Step 1, generate transaction data with:
  // - owners
  // - threshold
  // - other optional parameters
  const safeData = safeContract.interface.encodeFunctionData("setup", [
    owners,
    threshold,
    to,
    data,
    fallbackHandler,
    paymentToken,
    payment,
    paymentReceiver,
  ]);

  // The staticCall allows to predict the address of the upcoming Safe proxy before it is deployed
  const safeAddress =
    await proxyFactoryContract.createProxyWithNonce.staticCall(
      safeSingleton.address,
      safeData,
      0n,
    );

  if (safeAddress === ethers.ZeroAddress) {
    throw new Error("Safe address not found");
  }

  // Step 2, deploy the Safe proxy contract
  await proxyFactoryContract.createProxyWithNonce(
    safeSingleton.address,
    safeData,
    0n,
  );

  // Save the proxy deployment so it can be accessed via deployments.get
  const safeProxyName = "SafeL2Proxy";
  await deployments.save(safeProxyName, {
    abi: safeSingleton.abi,
    address: safeAddress,
  });

  console.log(
    `Deployed contract: ${safeProxyName}, address: ${safeAddress}, network: ${network.name}`,
  );
});

// Deploy the AdminModule contract
task("task:deployAdminModule").setAction(async function (
  _,
  { getNamedAccounts, deployments, network },
) {
  const { deploy } = deployments;
  const { deployer } = await getNamedAccounts();

  const adminAddress = getRequiredEnvVar("ADMIN_ADDRESS");
  const safeProxyAddress = getRequiredEnvVar("SAFE_ADDRESS");

  const contractName = "AdminModule";
  console.log(`Deploying ${contractName}...`);
  const { address } = await deploy(contractName, {
    from: deployer,
    args: [adminAddress, safeProxyAddress],
    log: true,
    skipIfAlreadyDeployed: false,
  });

  console.log(
    `Deployed contract: ${contractName}, network: ${network.name}, address: ${address}`,
  );
});

task("task:deployMultiSend").setAction(async function (
  _,
  { getNamedAccounts, deployments, network, ethers },
) {
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  const contractName = "MultiSend";
  console.log(`Deploying ${contractName}...`);
  const multiSendFactory = await new ContractFactory(
    MultiSendJson.abi,
    MultiSendJson.bytecode,
    deployerSigner,
  );
  const multiSend = await multiSendFactory.deploy();

  const multiSendAddress = await multiSend.getAddress();

  await deployments.save(contractName, {
    abi: MultiSendJson.abi,
    address: multiSendAddress,
  });

  console.log(
    `Deployed contract: ${contractName}, network: ${network.name}, address: ${multiSendAddress}`,
  );
});
