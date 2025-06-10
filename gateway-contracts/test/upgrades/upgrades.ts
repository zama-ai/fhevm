import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import { ethers, upgrades } from "hardhat";

import {
  CiphertextCommitsV2Example__factory,
  CiphertextCommits__factory,
  CoprocessorContextsV2Example__factory,
  CoprocessorContexts__factory,
  DecryptionV2Example__factory,
  Decryption__factory,
  EmptyUUPSProxyGatewayConfig__factory,
  EmptyUUPSProxy__factory,
  GatewayConfigV2Example__factory,
  GatewayConfig__factory,
  InputVerificationV2Example__factory,
  InputVerification__factory,
  KMSGenerationV2Example__factory,
  KMSGeneration__factory,
  MultichainACLV2Example__factory,
  MultichainACL__factory,
} from "../../typechain-types";
import { createAndFundRandomWallet, loadTestVariablesFixture, toValues } from "../utils";

describe("Upgrades", function () {
  let owner: Wallet;
  let regularEmptyUUPSFactory: EmptyUUPSProxy__factory;
  let gatewayConfigEmptyUUPSFactory: EmptyUUPSProxyGatewayConfig__factory;
  let ciphertextCommitsFactoryV1: CiphertextCommits__factory;
  let ciphertextCommitsFactoryV2: CiphertextCommitsV2Example__factory;
  let coprocessorContextsFactoryV1: CoprocessorContexts__factory;
  let coprocessorContextsFactoryV2: CoprocessorContextsV2Example__factory;
  let decryptionFactoryV1: Decryption__factory;
  let decryptionFactoryV2: DecryptionV2Example__factory;
  let gatewayConfigFactoryV1: GatewayConfig__factory;
  let gatewayConfigFactoryV2: GatewayConfigV2Example__factory;
  let inputVerificationFactoryV1: InputVerification__factory;
  let inputVerificationFactoryV2: InputVerificationV2Example__factory;
  let kmsGenerationFactoryV1: KMSGeneration__factory;
  let kmsGenerationFactoryV2: KMSGenerationV2Example__factory;
  let MultichainACLFactoryV1: MultichainACL__factory;
  let MultichainACLFactoryV2: MultichainACLV2Example__factory;

  before(async function () {
    owner = new Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    regularEmptyUUPSFactory = await ethers.getContractFactory("EmptyUUPSProxy", owner);
    gatewayConfigEmptyUUPSFactory = await ethers.getContractFactory("EmptyUUPSProxyGatewayConfig", owner);

    ciphertextCommitsFactoryV1 = await ethers.getContractFactory("CiphertextCommits", owner);
    ciphertextCommitsFactoryV2 = await ethers.getContractFactory("CiphertextCommitsV2Example", owner);

    coprocessorContextsFactoryV1 = await ethers.getContractFactory("CoprocessorContexts", owner);
    coprocessorContextsFactoryV2 = await ethers.getContractFactory("CoprocessorContextsV2Example", owner);

    decryptionFactoryV1 = await ethers.getContractFactory("Decryption", owner);
    decryptionFactoryV2 = await ethers.getContractFactory("DecryptionV2Example", owner);

    gatewayConfigFactoryV1 = await ethers.getContractFactory("GatewayConfig", owner);
    gatewayConfigFactoryV2 = await ethers.getContractFactory("GatewayConfigV2Example", owner);

    inputVerificationFactoryV1 = await ethers.getContractFactory("InputVerification", owner);
    inputVerificationFactoryV2 = await ethers.getContractFactory("InputVerificationV2Example", owner);

    kmsGenerationFactoryV1 = await ethers.getContractFactory("KMSGeneration", owner);
    kmsGenerationFactoryV2 = await ethers.getContractFactory("KMSGenerationV2Example", owner);

    MultichainACLFactoryV1 = await ethers.getContractFactory("MultichainACL", owner);
    MultichainACLFactoryV2 = await ethers.getContractFactory("MultichainACLV2Example", owner);
  });

  it("Should deploy upgradable MultichainACL", async function () {
    const nonceBef = await ethers.provider.getTransactionCount(owner);
    const emptyUUPS = await upgrades.deployProxy(regularEmptyUUPSFactory, [], {
      initializer: "initialize",
      kind: "uups",
    });
    const MultichainACL = await upgrades.upgradeProxy(emptyUUPS, MultichainACLFactoryV1);
    await MultichainACL.waitForDeployment();
    expect(await MultichainACL.getVersion()).to.equal("MultichainACL v0.2.0");
    const MultichainACLV2 = await upgrades.upgradeProxy(MultichainACL, MultichainACLFactoryV2);
    await MultichainACLV2.waitForDeployment();
    expect(await MultichainACLV2.getVersion()).to.equal("MultichainACL v1000.0.0");
    const multichainACLAddress = ethers.getCreateAddress({
      from: owner.address,
      nonce: nonceBef, // using nonce of nonceBef instead of nonceBef+1 here, since the original implementation has already been deployer during the setup phase, and hardhat-upgrades plugin is able to detect this and not redeploy twice same contract
    });
    expect(multichainACLAddress).to.equal(await MultichainACLV2.getAddress());
  });

  it("Should deploy upgradable CiphertextCommits", async function () {
    const emptyUUPS = await upgrades.deployProxy(regularEmptyUUPSFactory, [], {
      initializer: "initialize",
      kind: "uups",
    });
    const ciphertextCommits = await upgrades.upgradeProxy(emptyUUPS, ciphertextCommitsFactoryV1);
    await ciphertextCommits.waitForDeployment();
    expect(await ciphertextCommits.getVersion()).to.equal("CiphertextCommits v0.2.0");
    const ciphertextCommitsV2 = await upgrades.upgradeProxy(ciphertextCommits, ciphertextCommitsFactoryV2);
    await ciphertextCommitsV2.waitForDeployment();
    expect(await ciphertextCommitsV2.getVersion()).to.equal("CiphertextCommits v1000.0.0");
  });

  it("Should deploy upgradable CoprocessorContexts", async function () {
    const emptyUUPS = await upgrades.deployProxy(regularEmptyUUPSFactory, [], {
      initializer: "initialize",
      kind: "uups",
    });
    const coprocessorContexts = await upgrades.upgradeProxy(emptyUUPS, coprocessorContextsFactoryV1);
    await coprocessorContexts.waitForDeployment();
    expect(await coprocessorContexts.getVersion()).to.equal("CoprocessorContexts v0.1.0");
    const coprocessorContextsV2 = await upgrades.upgradeProxy(coprocessorContexts, coprocessorContextsFactoryV2);
    await coprocessorContextsV2.waitForDeployment();
    expect(await coprocessorContextsV2.getVersion()).to.equal("CoprocessorContexts v1000.0.0");
  });

  it("Should deploy upgradable Decryption", async function () {
    const emptyUUPS = await upgrades.deployProxy(regularEmptyUUPSFactory, [], {
      initializer: "initialize",
      kind: "uups",
    });
    const decryption = await upgrades.upgradeProxy(emptyUUPS, decryptionFactoryV1);
    await decryption.waitForDeployment();
    expect(await decryption.getVersion()).to.equal("Decryption v0.2.0");
    const decryptionV2 = await upgrades.upgradeProxy(decryption, decryptionFactoryV2);
    await decryptionV2.waitForDeployment();
    expect(await decryptionV2.getVersion()).to.equal("Decryption v1000.0.0");
  });

  it("Should deploy upgradable GatewayConfig", async function () {
    const emptyUUPS = await upgrades.deployProxy(gatewayConfigEmptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const gatewayConfig = await upgrades.upgradeProxy(emptyUUPS, gatewayConfigFactoryV1);
    await gatewayConfig.waitForDeployment();
    expect(await gatewayConfig.getVersion()).to.equal("GatewayConfig v0.2.0");
    const gatewayConfigV2 = await upgrades.upgradeProxy(gatewayConfig, gatewayConfigFactoryV2);
    await gatewayConfigV2.waitForDeployment();
    expect(await gatewayConfigV2.getVersion()).to.equal("GatewayConfig v1000.0.0");
  });

  it("Should deploy upgradable KMSGeneration", async function () {
    const emptyUUPS = await upgrades.deployProxy(regularEmptyUUPSFactory, [], {
      initializer: "initialize",
      kind: "uups",
    });
    const kmsGeneration = await upgrades.upgradeProxy(emptyUUPS, kmsGenerationFactoryV1);
    await kmsGeneration.waitForDeployment();
    expect(await kmsGeneration.getVersion()).to.equal("KMSGeneration v0.1.0");
    const kmsGenerationV2 = await upgrades.upgradeProxy(kmsGeneration, kmsGenerationFactoryV2);
    await kmsGenerationV2.waitForDeployment();
    expect(await kmsGenerationV2.getVersion()).to.equal("KMSGeneration v1000.0.0");
  });

  it("Should deploy upgradable InputVerification", async function () {
    const emptyUUPS = await upgrades.deployProxy(regularEmptyUUPSFactory, [], {
      initializer: "initialize",
      kind: "uups",
    });
    const inputVerification = await upgrades.upgradeProxy(emptyUUPS, inputVerificationFactoryV1);
    await inputVerification.waitForDeployment();
    expect(await inputVerification.getVersion()).to.equal("InputVerification v0.2.0");
    const inputVerificationV2 = await upgrades.upgradeProxy(inputVerification, inputVerificationFactoryV2);
    await inputVerificationV2.waitForDeployment();
    expect(await inputVerificationV2.getVersion()).to.equal("InputVerification v1000.0.0");
  });

  it("Should allow original owner to upgrade the GatewayConfig, transfer ownership and no longer upgrade the contract", async function () {
    // Create a new gateway contract in order to avoid upgrading the original one and thus break
    // some tests if it's not re-compiled in the mean time
    const emptyUUPS = await upgrades.deployProxy(gatewayConfigEmptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const gatewayConfig = await upgrades.upgradeProxy(emptyUUPS, gatewayConfigFactoryV1);
    await gatewayConfig.waitForDeployment();
    expect(await gatewayConfig.getVersion()).to.equal("GatewayConfig v0.2.0");

    const newSigner = await createAndFundRandomWallet();
    await gatewayConfig.transferOwnership(newSigner);
    await gatewayConfig.connect(newSigner).acceptOwnership();

    // Old owner should not be able to upgrade the contract
    const gatewayConfigV2ExampleFactoryOldOwner = await ethers.getContractFactory("GatewayConfigV2Example", owner);
    await expect(upgrades.upgradeProxy(gatewayConfig, gatewayConfigV2ExampleFactoryOldOwner)).to.be.reverted;

    // New owner should be able to upgrade the contract
    const gatewayConfigV2ExampleFactoryNewOwner = await ethers.getContractFactory("GatewayConfigV2Example", newSigner);
    const gatewayConfigV2 = await upgrades.upgradeProxy(gatewayConfig, gatewayConfigV2ExampleFactoryNewOwner);

    await gatewayConfigV2.waitForDeployment();
    expect(await gatewayConfigV2.getVersion()).to.equal("GatewayConfig v1000.0.0");
  });
});
