import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { IKMSGeneration, KMSGeneration } from "../../typechain-types";
import { generateKey } from "../KMSGeneration";
import { KeyTypeEnum, createByteInput, loadTestVariablesFixture } from "../utils";

describe("Key resharing tasks", function () {
  // Define the private key of the new owner (Account 2)
  const newOwnerPrivateKey = "0x7ae52cf0d3011ef7fecbe22d9537aeda1a9e42a0596e8def5d49970eb59e7a40";
  const newOwner = new Wallet(newOwnerPrivateKey).connect(hre.ethers.provider);
  let kmsGeneration: KMSGeneration;
  let owner: Wallet;
  let kmsTxSenders: HardhatEthersSigner[];
  let kmsSigners: HardhatEthersSigner[];

  before(async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    kmsGeneration = fixtureData.kmsGeneration;
    owner = fixtureData.owner;
    kmsTxSenders = fixtureData.kmsTxSenders;
    kmsSigners = fixtureData.kmsSigners;
  });

  it("Should trigger PRSS init", async function () {
    await hre.run("task:prssInit", { useInternalGatewayConfigAddress: true });
    const filter = kmsGeneration.filters.PRSSInit();
    const events = await kmsGeneration.queryFilter(filter);
    expect(events.length).to.be.greaterThan(0);
  });

  it("Should trigger key reshare same set", async function () {
    // Define the key digests.
    const serverKeyDigest: IKMSGeneration.KeyDigestStruct = {
      keyType: KeyTypeEnum.Server,
      digest: createByteInput(),
    };
    const publicKeyDigest: IKMSGeneration.KeyDigestStruct = {
      keyType: KeyTypeEnum.Public,
      digest: createByteInput(),
    };
    const keyDigests = [serverKeyDigest, publicKeyDigest];

    // Get the gateway's chain ID.
    const gatewayChainId = hre.network.config.chainId!;

    const keyId = await generateKey(kmsGeneration, owner, gatewayChainId, kmsTxSenders, kmsSigners, keyDigests);
    await hre.run("task:keyReshareSameSet", { keyId: keyId.toString(), useInternalGatewayConfigAddress: true });

    const filter = kmsGeneration.filters.KeyReshareSameSet();
    const events = await kmsGeneration.queryFilter(filter);
    expect(events.length).to.be.greaterThan(0);
  });

  it("Should revert because key is not generated", async function () {
    await expect(hre.run("task:keyReshareSameSet", { keyId: "9999", useInternalGatewayConfigAddress: true })).to.be
      .reverted;
  });
});
