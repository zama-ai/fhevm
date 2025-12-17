import { ethers, upgrades } from "hardhat";
import { AdminProvider, AdminProvider__factory, SanctionsList, SanctionsList__factory, FeeManager, FeeManager__factory, TestERC20__factory, TestERC20, DeploymentCoordinator, ConfidentialTokenFactory__factory, ConfidentialTokenFactory, WrapperFactory__factory, DeploymentCoordinator__factory, WrapperFactory, FeeManagerInteractor__factory, FeeManagerInteractor, RegulatedERC7984Upgradeable__factory, UniswapV2Factory__factory, UniswapV2Factory, SwapV0__factory, SwapV0, ERC7984TransferBatcher__factory, ERC7984TransferBatcher } from "../types";
import { Signers } from "./signers";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";

import UniswapV2FactoryArtifact from '@uniswap/v2-core/build/UniswapV2Factory.json';


export async function deploySanctionsListFixture() {
  const factory = (await ethers.getContractFactory("SanctionsList")) as SanctionsList__factory;
  const sanctionsList = (await factory.deploy()) as SanctionsList;
  const sanctionsListAddress = await sanctionsList.getAddress();
  return { sanctionsList, sanctionsListAddress };
}

export async function deployFeeManagerFixture(royalties: HardhatEthersSigner) {
  const [deployer] = await ethers.getSigners();
  const factory = (await ethers.getContractFactory("FeeManager")) as FeeManager__factory;
  const feeManager = (await factory.deploy(
    100,  // 1% wrap fee basis points
    100,  // 1% unwrap fee basis points
    ethers.parseEther("0.01"), // deploy fee
    ethers.parseEther("0.001"), // batch transfer fee
    royalties,
  )) as FeeManager;
  const feeManagerAddress = await feeManager.getAddress();

  // Grant FEE_MANAGER_ROLE to the deployer for test purposes
  const FEE_MANAGER_ROLE = await feeManager.FEE_MANAGER_ROLE();
  await feeManager.grantRole(FEE_MANAGER_ROLE, deployer.address);

  return { feeManager, feeManagerAddress };
}

export async function deployTestERC20Fixture(symbol: string, decimals: number = 6, name?: string) {
  const factory = (await ethers.getContractFactory("TestERC20")) as TestERC20__factory;
  if (name == undefined) {
    name = symbol;
  }
  return (await factory.deploy(name, symbol, decimals)) as TestERC20;
}

export async function deployTestUnsafeERC20Fixture(contract_name: string, symbol: string, decimals: number = 6, name?: string) {
  const factory = (await ethers.getContractFactory(contract_name)) as TestERC20__factory;
  if (name == undefined) {
    name = symbol;
  }
  return (await factory.deploy(name, symbol, decimals)) as TestERC20;
}

export async function deployERC20NoDecimalsFixture() {
  const ERC20NoDecimalsContract = await ethers.getContractFactory("ERC20NoDecimals");
  const tokenNoDecimals = await ERC20NoDecimalsContract.deploy("NoDecimalsToken", "NDT");
  await tokenNoDecimals.waitForDeployment();
  const tokenAsTestERC20 = await ethers.getContractAt("TestERC20", await tokenNoDecimals.getAddress());
  return { tokenNoDecimals: tokenAsTestERC20 };
}

export async function deployERC20InvalidDecimalsFixture() {
  const ERC20InvalidDecimalsContract = await ethers.getContractFactory("ERC20InvalidDecimals");
  const tokenInvalidDecimals = await ERC20InvalidDecimalsContract.deploy("InvalidDecimalsToken", "IDT");
  await tokenInvalidDecimals.waitForDeployment();
  const tokenAsTestERC20 = await ethers.getContractAt("TestERC20", await tokenInvalidDecimals.getAddress());
  return { tokenInvalidDecimals: tokenAsTestERC20 };
}

export async function deployAdminProviderFixture(signers: Signers, sanctionsList?: SanctionsList) {
  // Deploy and configure FeeManager with default fees
  const { feeManager, feeManagerAddress } = await deployFeeManagerFixture(signers.royalties);

  if (sanctionsList === undefined) {
    ({ sanctionsList } = await deploySanctionsListFixture());
  }
  const sanctionsListAddress = await sanctionsList.getAddress();

  const adminProviderFactory = (await ethers.getContractFactory("AdminProvider")) as AdminProvider__factory;
  const adminProvider = (await adminProviderFactory.deploy(feeManagerAddress, sanctionsListAddress, signers.regulator)) as AdminProvider;
  const adminProviderAddress = await adminProvider.getAddress();
  if (sanctionsList) {
    await adminProvider.setSanctionsList(sanctionsList);
  }

  return { adminProvider, adminProviderAddress, feeManager, feeManagerAddress, sanctionsList, sanctionsListAddress };
}

export async function deployConfidentialErc20Fixture(signers: Signers, adminProvider?: AdminProvider, underlying?: TestERC20, deployerIsWrapper = true) {
  // Deploy AdminProvider if not provided
  if (!adminProvider) {
      ({ adminProvider } = await deployAdminProviderFixture(signers));
  }
  const adminProviderAddress = await adminProvider.getAddress();
  const { coordinator } = await deployCoordinator(signers, adminProvider);

  const ERC7984 = await ethers.getContractFactory("RegulatedERC7984Upgradeable");
  const underlyingAddress = underlying ? await underlying.getAddress() : ethers.ZeroAddress;
  const cErc20 = await upgrades.deployProxy(ERC7984, [
    "Naraggara",
    "NARA",
    6,
    signers.deployer.address, // admin
    1, // rate
    underlyingAddress,
    await coordinator.getAddress(),
    signers.deployer.address // wrapperSetter (deployer will call setWrapper if needed)
  ]);
  await cErc20.waitForDeployment();
  if (deployerIsWrapper) {
    await cErc20.setWrapper(signers.deployer.address);
  }
  const cErc20Address = await cErc20.getAddress();

  return { cErc20, cErc20Address, adminProvider, adminProviderAddress };
}

export async function deployBurnableConfidentialErc20Fixture(signers: Signers, adminProvider?: AdminProvider, deployerIsWrapper = true) {
  // Deploy AdminProvider if not provided
  if (!adminProvider) {
      ({ adminProvider } = await deployAdminProviderFixture(signers));
  }
  const adminProviderAddress = await adminProvider.getAddress();
  const { coordinator } = await deployCoordinator(signers, adminProvider);


  const burnableErc7984Factory = await ethers.getContractFactory("BurnableRegulatedERC7984Upgradeable");
  const burnableCErc20 = await upgrades.deployProxy(burnableErc7984Factory, [
    "Naraggara",
    "NARA",
    6,
    signers.deployer.address, // admin
    1, // rate
    ethers.ZeroAddress,
    await coordinator.getAddress(),
    signers.deployer.address // wrapperSetter
  ]);
  await burnableCErc20.waitForDeployment();
  if (deployerIsWrapper) {
    await burnableCErc20.setWrapper(signers.deployer.address);
  }
  const burnableCErc20Address = await burnableCErc20.getAddress();

  return { burnableCErc20, burnableCErc20Address, adminProvider, adminProviderAddress };
}


export async function deployWrapperFactoryFixture(owner?: HardhatEthersSigner) {
  const wrapperFactory = (await ethers.getContractFactory("WrapperFactory")) as WrapperFactory__factory;
  const deployedWrapperFactory = (await wrapperFactory.deploy()) as WrapperFactory;
  const wrapperFactoryAddress = await deployedWrapperFactory.getAddress();

  if (owner) {
    await deployedWrapperFactory.transferOwnership(await owner.getAddress());
  }

  return { wrapperFactory: deployedWrapperFactory, wrapperFactoryAddress };
}

export async function deployConfidentialTokenFactoryFixture(owner?: HardhatEthersSigner) {
  const confidentialTokenFactory = (await ethers.getContractFactory("RegulatedERC7984UpgradeableFactory")) as ConfidentialTokenFactory__factory;
  const deployedConfidentialTokenFactory = (await confidentialTokenFactory.deploy()) as ConfidentialTokenFactory;
  const confidentialTokenFactoryAddress = await deployedConfidentialTokenFactory.getAddress();

  if (owner) {
    await deployedConfidentialTokenFactory.transferOwnership(await owner.getAddress());
  }

  return { confidentialTokenFactory: deployedConfidentialTokenFactory, confidentialTokenFactoryAddress };
}

export async function deployCoordinator(signers: Signers, adminProvider?: AdminProvider) {
  if (!adminProvider) {
      ({ adminProvider } = await deployAdminProviderFixture(signers));
  }

  const { confidentialTokenFactory } = await deployConfidentialTokenFactoryFixture();
  const { wrapperFactory } = await deployWrapperFactoryFixture();

  const coordinatorFactory = (await ethers.getContractFactory("DeploymentCoordinator")) as DeploymentCoordinator__factory;
  const coordinator = (await coordinatorFactory.deploy(
    adminProvider,
    wrapperFactory,
    confidentialTokenFactory
  )) as DeploymentCoordinator;

  // Set coordinator as owner of both factories (using 2-step ownership transfer)
  await confidentialTokenFactory.transferOwnership(await coordinator.getAddress());
  await wrapperFactory.transferOwnership(await coordinator.getAddress());

  // Accept ownership from the coordinator side
  await coordinator.acceptConfidentialTokenFactoryOwnership();
  await coordinator.acceptWrapperFactoryOwnership();

  return { adminProvider, coordinator, confidentialTokenFactory, wrapperFactory };
}

export async function deployWrapperFixture(signers: Signers) {
  const { adminProvider, coordinator, confidentialTokenFactory, wrapperFactory } = await deployCoordinator(signers);

  // Deploy wrapper implementation contract
  const wrapperUpgradeableFactory = await ethers.getContractFactory("WrapperUpgradeable");
  const wrapperImplementation = await wrapperUpgradeableFactory.deploy();
  await wrapperImplementation.waitForDeployment();

  // Set wrapper implementation on coordinator
  await coordinator.setWrapperImplementation(await wrapperImplementation.getAddress());

  // For compatibility with existing tests, we can return a "mock" wrapper structure
  // Tests will need to use coordinator.deploy() to actually create wrapper/token pairs
  const coordinatorAddress = await coordinator.getAddress();

  return {
    coordinator,
    coordinatorAddress,
    adminProvider,
    confidentialTokenFactory,
    wrapperFactory,
    wrapperImplementation,
  };
}

export async function deployUniswapFactoryFixture(deployer: HardhatEthersSigner) {
  const WETH9 = await ethers.getContractFactory("WETH9");
  const weth = await WETH9.deploy();

  const Factory = (await ethers.getContractFactory(
    UniswapV2FactoryArtifact.abi,
    UniswapV2FactoryArtifact.bytecode,
    deployer,
  )) as UniswapV2Factory__factory;
  const factory = (await Factory.deploy(deployer.address)) as UniswapV2Factory;

  const Router = await ethers.getContractFactory("UniswapV2Router02");
  const router = await Router.deploy(factory.target, weth.target);

  return { router, factory, weth };
}


export async function deploySwapV0Fixture(coordinator: DeploymentCoordinator) {
  const factory = (await ethers.getContractFactory("SwapV0")) as SwapV0__factory;
  const swapV0 = (await factory.deploy(coordinator)) as SwapV0;
  return { swapV0, swapV0Address: await swapV0.getAddress() };
}

export async function deployTransferBatcherFixture(deployer: HardhatEthersSigner, adminProvider: AdminProvider) {
  const transferBatcherFactory = (await ethers.getContractFactory("ERC7984TransferBatcher")) as ERC7984TransferBatcher__factory;
  const transferBatcher = (await transferBatcherFactory.connect(deployer).deploy(adminProvider)) as ERC7984TransferBatcher;
  const transferBatcherAddress = await transferBatcher.getAddress();
  return { transferBatcher, transferBatcherAddress };
}
