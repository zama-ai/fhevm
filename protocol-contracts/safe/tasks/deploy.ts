import MultiSendJson from "@safe-global/safe-contracts/build/artifacts/contracts/libraries/MultiSend.sol/MultiSend.json";
import { ContractFactory } from "ethers";
import { task } from "hardhat/config";

import { getRequiredEnvVar, getDeployedAddress } from "./utils/loadVariables";

// Deploy the SafeSmartAccount contract
// Example usage:
// npx hardhat task:deploySafe --network gateway-testnet
task("task:deploySafe").setAction(async function (
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

  // Set the Safe setup parameters:
  // - owners: the deployer only
  // - threshold: 1
  // - remaining optional parameters: use default values if not provided
  const owners = [deployer];
  const threshold = 1;
  const to = ethers.ZeroAddress;
  const data = "0x";
  const fallbackHandler = ethers.ZeroAddress;
  const paymentToken = ethers.ZeroAddress;
  const payment = 0;
  const paymentReceiver = ethers.ZeroAddress;

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
  const safeProxyAddress =
    await proxyFactoryContract.createProxyWithNonce.staticCall(
      safeSingleton.address,
      safeData,
      0n,
    );

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
    address: safeProxyAddress,
  });

  console.log(
    `Deployed contract: ${safeProxyName}, address: ${safeProxyAddress}, network: ${network.name}`,
  );
});

// Deploy the AdminModule contract
// Example usage:
// npx hardhat task:deployAdminModule
task("task:deployAdminModule").setAction(async function (
  _,
  { getNamedAccounts, deployments, network },
) {
  const { deploy } = deployments;
  const { deployer } = await getNamedAccounts();

  const adminAddress = getRequiredEnvVar("ADMIN_ADDRESS");

  let safeProxyAddress;
  if (network.name === "hardhat") {
    safeProxyAddress = getRequiredEnvVar("SAFE_PROXY_ADDRESS");
  } else {
    safeProxyAddress = await getDeployedAddress(network.name, "SafeL2Proxy");
  }

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

// Deploy the MultiSend contract
// Example usage:
// npx hardhat task:deployMultiSend
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
