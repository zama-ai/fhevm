import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";

import { GatewayConfig, ProtocolPayment, ZamaOFT } from "../typechain-types";
import { loadTestVariablesFixture } from "./utils";

describe("ProtocolPayment", function () {
  // Define 1 $ZAMA token (using 18 decimals)
  const oneZamaToken = BigInt(10 ** 18);

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

  describe("Send fees to burner", function () {
    const protocolPaymentBalance = oneZamaToken * BigInt(10 ** 3);

    let protocolPaymentAddress: string;

    beforeEach(async function () {
      protocolPaymentAddress = await protocolPayment.getAddress();

      // Set an initial balance for the protocol payment contract using the owner's tokens
      await mockedZamaOFT.connect(owner).transfer(protocolPaymentAddress, protocolPaymentBalance);
    });

    it("Should send all protocol fees to the FeesSenderToBurner address", async function () {
      await protocolPayment.sendBalance();

      // Check that the protocol payment contract has no balance left
      const newProtocolPaymentBalance = await mockedZamaOFT.balanceOf(protocolPaymentAddress);
      expect(newProtocolPaymentBalance).to.equal(BigInt(0));

      // Check that the FeesSenderToBurner address has the ProtocolPayment's initial balance
      const newFeesSenderToBurnerBalance = await mockedZamaOFT.balanceOf(mockedFeesSenderToBurnerAddress);
      expect(newFeesSenderToBurnerBalance).to.equal(protocolPaymentBalance);
    });
  });
});
