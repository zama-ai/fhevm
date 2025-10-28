import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";

import { GatewayConfig, ProtocolPayment, ZamaOFT } from "../typechain-types";
import { createRandomWallet, loadTestVariablesFixture } from "./utils";

describe("ProtocolPayment", function () {
  // Define 1 $ZAMA token (using 18 decimals)
  const oneZamaToken = BigInt(10 ** 18);

  // Define fake tx sender
  const fakeTxSender = createRandomWallet();

  let gatewayConfig: GatewayConfig;
  let protocolPayment: ProtocolPayment;
  let mockedZamaOFT: ZamaOFT;
  let mockedFeesSenderToBurnerAddress: string;
  let owner: Wallet;
  let inputVerificationPrice: bigint;
  let publicDecryptionPrice: bigint;
  let userDecryptionPrice: bigint;
  let newInputVerificationPrice: bigint;
  let newPublicDecryptionPrice: bigint;
  let newUserDecryptionPrice: bigint;

  before(async function () {
    // Initialize globally used variables before each test
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixtureData.gatewayConfig;
    protocolPayment = fixtureData.protocolPayment;
    mockedZamaOFT = fixtureData.mockedZamaOFT;
    mockedFeesSenderToBurnerAddress = fixtureData.mockedFeesSenderToBurnerAddress;
    owner = fixtureData.owner;
    inputVerificationPrice = fixtureData.inputVerificationPrice;
    publicDecryptionPrice = fixtureData.publicDecryptionPrice;
    userDecryptionPrice = fixtureData.userDecryptionPrice;

    // Define new prices
    newInputVerificationPrice = inputVerificationPrice + oneZamaToken * BigInt(2);
    newPublicDecryptionPrice = publicDecryptionPrice + oneZamaToken * BigInt(3);
    newUserDecryptionPrice = userDecryptionPrice + oneZamaToken * BigInt(4);
  });

  describe("Setters and getters", function () {
    it("Should set the input verification price", async function () {
      expect(await protocolPayment.getInputVerificationPrice()).to.equal(inputVerificationPrice);
      await protocolPayment.connect(owner).setInputVerificationPrice(newInputVerificationPrice);
      expect(await protocolPayment.getInputVerificationPrice()).to.equal(newInputVerificationPrice);
    });

    it("Should set the public decryption price", async function () {
      expect(await protocolPayment.getPublicDecryptionPrice()).to.equal(publicDecryptionPrice);
      await protocolPayment.connect(owner).setPublicDecryptionPrice(newPublicDecryptionPrice);
      expect(await protocolPayment.getPublicDecryptionPrice()).to.equal(newPublicDecryptionPrice);
    });

    it("Should set the user decryption price", async function () {
      expect(await protocolPayment.getUserDecryptionPrice()).to.equal(userDecryptionPrice);
      await protocolPayment.connect(owner).setUserDecryptionPrice(newUserDecryptionPrice);
      expect(await protocolPayment.getUserDecryptionPrice()).to.equal(newUserDecryptionPrice);
    });
  });

  describe("Fee collection errors", function () {
    it("Should revert because sender is not the InputVerification contract for input verification fee collection", async function () {
      await expect(protocolPayment.connect(fakeTxSender).collectInputVerificationFee(fakeTxSender.address))
        .to.be.revertedWithCustomError(protocolPayment, "SenderNotInputVerificationContract")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because sender is not the Decryption contract for public decryption fee collection", async function () {
      await expect(protocolPayment.connect(fakeTxSender).collectPublicDecryptionFee(fakeTxSender.address))
        .to.be.revertedWithCustomError(protocolPayment, "SenderNotDecryptionContract")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because sender is not the Decryption contract for user decryption fee collection", async function () {
      await expect(protocolPayment.connect(fakeTxSender).collectUserDecryptionFee(fakeTxSender.address))
        .to.be.revertedWithCustomError(protocolPayment, "SenderNotDecryptionContract")
        .withArgs(fakeTxSender.address);
    });
  });
});
