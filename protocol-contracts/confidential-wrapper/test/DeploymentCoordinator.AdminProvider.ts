import { expect } from "chai";
import { ethers } from "hardhat";
import { getSigners, Signers } from "./signers";
import { deployWrapperFixture, deployAdminProviderFixture, deployTestERC20Fixture } from "./fixtures";
import { deployConfidentialToken } from "./utils";
import {
  DeploymentCoordinator,
  AdminProvider,
  WrapperUpgradeable,
  RegulatedERC7984Upgradeable,
  TestERC20,
} from "../types";

describe("DeploymentCoordinator - Dynamic AdminProvider Update", function () {
  let signers: Signers;
  let coordinator: DeploymentCoordinator;
  let adminProvider: AdminProvider;
  let tokenA: TestERC20;
  let tokenB: TestERC20;
  let wrapperA: WrapperUpgradeable;
  let wrapperB: WrapperUpgradeable;
  let cTokenA: RegulatedERC7984Upgradeable;
  let cTokenB: RegulatedERC7984Upgradeable;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async function () {
    // Deploy the full system using fixture
    ({ coordinator, adminProvider } = await deployWrapperFixture(signers));

    // Deploy two test tokens
    tokenA = await deployTestERC20Fixture("TKNA", 18);
    tokenB = await deployTestERC20Fixture("TKNB", 6);

    // Deploy wrapper/cToken pairs for both tokens
    const { wrapper: wrapA, cTokenAddress: cTokenAddressA } = await deployConfidentialToken(
      coordinator,
      tokenA,
      signers.deployer
    );
    wrapperA = wrapA;
    cTokenA = await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenAddressA);

    const { wrapper: wrapB, cTokenAddress: cTokenAddressB } = await deployConfidentialToken(
      coordinator,
      tokenB,
      signers.deployer
    );
    wrapperB = wrapB;
    cTokenB = await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenAddressB);
  });

  it("should update AdminProvider for all deployed wrappers and tokens simultaneously", async function () {
    // Verify all contracts initially use the same AdminProvider
    const initialAdminProviderAddress = await adminProvider.getAddress();

    expect(await wrapperA.adminProvider()).to.equal(initialAdminProviderAddress);
    expect(await wrapperB.adminProvider()).to.equal(initialAdminProviderAddress);
    expect(await cTokenA.adminProvider()).to.equal(initialAdminProviderAddress);
    expect(await cTokenB.adminProvider()).to.equal(initialAdminProviderAddress);

    // Verify initial regulator
    const initialRegulator = await adminProvider.regulator();
    expect(await cTokenA.regulator()).to.equal(initialRegulator);
    expect(await cTokenB.regulator()).to.equal(initialRegulator);

    // Deploy a new AdminProvider with a different regulator
    const { adminProvider: newAdminProvider } = await deployAdminProviderFixture(signers);
    await newAdminProvider.setRegulator(signers.alice.address); // Use Alice as new regulator

    const newAdminProviderAddress = await newAdminProvider.getAddress();

    // Update AdminProvider in coordinator - this should affect ALL deployed contracts
    await expect(coordinator.setAdminProvider(newAdminProvider))
      .to.emit(coordinator, "AdminProviderUpdated")
      .withArgs(initialAdminProviderAddress, newAdminProviderAddress);

    // Verify all wrappers and tokens now see the new AdminProvider WITHOUT any upgrade
    expect(await wrapperA.adminProvider()).to.equal(newAdminProviderAddress);
    expect(await wrapperB.adminProvider()).to.equal(newAdminProviderAddress);
    expect(await cTokenA.adminProvider()).to.equal(newAdminProviderAddress);
    expect(await cTokenB.adminProvider()).to.equal(newAdminProviderAddress);

    // Verify they all see the new regulator
    expect(await cTokenA.regulator()).to.equal(signers.alice.address);
    expect(await cTokenB.regulator()).to.equal(signers.alice.address);

    // Verify the contracts maintain their own independent storage
    expect(await wrapperA.originalToken()).to.equal(await tokenA.getAddress());
    expect(await wrapperB.originalToken()).to.equal(await tokenB.getAddress());
    expect(await wrapperA.originalToken()).to.not.equal(await wrapperB.originalToken());
  });
});
