import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { ZKPoKManager } from "../typechain-types";
import { createEIP712ResponseZKPoK, deployHTTPZFixture, getSignaturesZKPoK } from "./utils";

describe("ZKPoKManager", function () {
  async function deployZKPoKManagerFixture() {
    const { httpz, coprocessorSigners, admin, signers } = await deployHTTPZFixture();
    const ZKPoKManagerContract = await hre.ethers.getContractFactory("ZKPoKManager");
    const zkpokManager = await ZKPoKManagerContract.deploy(httpz, "0xDA9FeD390f02F559E62240a112aBd2FAe06DCdB5");
    return { httpz, zkpokManager, coprocessorSigners, admin, signers };
  }

  describe("Verify proof request", async function () {
    const zkProofId = "0";
    const contractChainId = "123";
    let zkpokManager: ZKPoKManager;
    before(async function () {
      const fixture = await loadFixture(deployZKPoKManagerFixture);
      await fixture.httpz.connect(fixture.admin).addNetwork({
        chainId: contractChainId,
        httpzLibrary: hre.ethers.Wallet.createRandom().address,
        acl: hre.ethers.Wallet.createRandom().address,
        name: "Network name",
        website: "network-website.com",
      });
      zkpokManager = fixture.zkpokManager;
    });

    it("Should success", async function () {
      // Given
      const contractAddress = "0xa83114A443dA1CecEFC50368531cACE9F37fCCcb";
      const userAddress = "0x388C818CA8B9251b393131C08a736A67ccB19297";
      const ctProofHandle = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

      // When
      const txResponse = zkpokManager.verifyProofRequest(contractChainId, contractAddress, userAddress, ctProofHandle);

      // Then
      await expect(txResponse)
        .to.emit(zkpokManager, "VerifyProofRequest")
        .withArgs(zkProofId, contractChainId, contractAddress, userAddress, ctProofHandle);
    });

    it("Should revert with NetworkNotRegistered", async function () {
      // Given
      const fakeChainId = "456";
      const contractAddress = "0xa83114A443dA1CecEFC50368531cACE9F37fCCcb";
      const userAddress = "0x388C818CA8B9251b393131C08a736A67ccB19297";
      const ctProofHandle = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

      // When
      const txResponse = zkpokManager.verifyProofRequest(fakeChainId, contractAddress, userAddress, ctProofHandle);

      // Then
      await expect(txResponse).revertedWithCustomError(zkpokManager, "NetworkNotRegistered").withArgs(fakeChainId);
    });
  });

  describe("Verify proof response", async function () {
    const zkProofId = "0";
    const contractChainId = "123";
    const contractAddress = "0xa83114A443dA1CecEFC50368531cACE9F37fCCcb";
    const userAddress = "0x388C818CA8B9251b393131C08a736A67ccB19297";
    let zkpokManager: ZKPoKManager;
    let coprocessorSigners: HardhatEthersSigner[];
    let fakeSigner: HardhatEthersSigner;

    beforeEach(async function () {
      const fixture = await loadFixture(deployZKPoKManagerFixture);
      await fixture.httpz.connect(fixture.admin).addNetwork({
        chainId: contractChainId,
        httpzLibrary: hre.ethers.Wallet.createRandom().address,
        acl: hre.ethers.Wallet.createRandom().address,
        name: "Network name",
        website: "network-website.com",
      });
      zkpokManager = fixture.zkpokManager;
      coprocessorSigners = fixture.coprocessorSigners;
      fakeSigner = fixture.signers[0];
      const ctProofHandle = hre.ethers.randomBytes(32);
      await zkpokManager.verifyProofRequest(contractChainId, contractAddress, userAddress, ctProofHandle);
    });

    it("Should success", async function () {
      // Given
      const zkpokManagerAddress = await zkpokManager.getAddress();
      const handles = [hre.ethers.randomBytes(32), hre.ethers.randomBytes(32)];
      const eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        zkpokManagerAddress,
        handles,
        userAddress,
        contractAddress,
        contractChainId,
      );
      const [signature1, signature2, signature3] = await getSignaturesZKPoK(eip712Message, coprocessorSigners);

      // When
      await zkpokManager.connect(coprocessorSigners[1]).verifyProofResponse(zkProofId, handles, signature1);
      let txResponse = zkpokManager.connect(coprocessorSigners[2]).verifyProofResponse(zkProofId, handles, signature2);
      let lateTxResponse = zkpokManager
        .connect(coprocessorSigners[2])
        .verifyProofResponse(zkProofId, handles, signature3);

      // Then
      await expect(txResponse)
        .to.emit(zkpokManager, "VerifyProofResponse")
        .withArgs(zkProofId, handles, [signature1, signature2]);
      await expect(lateTxResponse).to.not.emit(zkpokManager, "VerifyProofResponse");
    });

    it("Should revert with InvalidCoprocessorSigner", async function () {
      // Given
      const handles = [hre.ethers.randomBytes(32), hre.ethers.randomBytes(32)];
      const zkpokManagerAddress = await zkpokManager.getAddress();
      const eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        zkpokManagerAddress,
        handles,
        userAddress,
        contractAddress,
        contractChainId,
      );
      const [signature1] = await getSignaturesZKPoK(eip712Message, [fakeSigner]);

      // When
      let txResponse = zkpokManager.verifyProofResponse(zkProofId, handles, signature1);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(zkpokManager, "InvalidCoprocessorSigner")
        .withArgs(fakeSigner.address);
    });

    it("Should revert with CoprocessorHasAlreadySigned", async function () {
      // Given
      const handles = [hre.ethers.randomBytes(32), hre.ethers.randomBytes(32)];
      const zkpokManagerAddress = await zkpokManager.getAddress();
      const eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        zkpokManagerAddress,
        handles,
        userAddress,
        contractAddress,
        contractChainId,
      );
      const [signature1] = await getSignaturesZKPoK(eip712Message, coprocessorSigners);

      // When
      await zkpokManager.verifyProofResponse(zkProofId, handles, signature1);
      let txResponse = zkpokManager.verifyProofResponse(zkProofId, handles, signature1);

      // Then
      await expect(txResponse)
        .revertedWithCustomError(zkpokManager, "CoprocessorHasAlreadySigned")
        .withArgs(zkProofId, coprocessorSigners[0].address);
    });
  });
});
