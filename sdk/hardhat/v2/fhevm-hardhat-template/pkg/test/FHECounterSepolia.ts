import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers, fhevm, deployments } from "hardhat";
import { FHECounter } from "../types";
import { expect } from "chai";
import { FhevmType } from "@fhevm/hardhat-plugin";

type Signers = {
  alice: HardhatEthersSigner;
};

describe("FHECounterSepolia", function () {
  let signers: Signers;
  let fheCounterContract: FHECounter;
  let fheCounterContractAddress: string;
  let step: number;
  let steps: number;

  function progress(message: string) {
    console.log(`${++step}/${steps} ${message}`);
  }

  before(async function () {
    if (fhevm.isMock) {
      console.warn(`This hardhat test suite can only run on Sepolia Testnet`);
      this.skip();
    }

    try {
      const FHECounterDeployement = await deployments.get("FHECounter");
      fheCounterContractAddress = FHECounterDeployement.address;
      fheCounterContract = await ethers.getContractAt("FHECounter", FHECounterDeployement.address);
    } catch (e) {
      (e as Error).message += ". Call 'npx hardhat deploy --network sepolia'";
      throw e;
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { alice: ethSigners[0] };
  });

  beforeEach(async () => {
    step = 0;
    steps = 0;
  });

  it("increment the counter by 1", async function () {
    steps = 10;

    this.timeout(4 * 40000);

    progress("Encrypting '0'...");
    const encryptedZero = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(0)
      .encrypt();

    progress(
      `Call increment(0) FHECounter=${fheCounterContractAddress} handle=${ethers.hexlify(encryptedZero.handles[0])} signer=${signers.alice.address}...`,
    );
    let tx = await fheCounterContract
      .connect(signers.alice)
      .increment(encryptedZero.handles[0], encryptedZero.inputProof);
    await tx.wait();

    progress(`Call FHECounter.getCount()...`);
    const encryptedCountBeforeInc = await fheCounterContract.getCount();
    expect(encryptedCountBeforeInc).to.not.eq(ethers.ZeroHash);

    progress(`Decrypting FHECounter.getCount()=${encryptedCountBeforeInc}...`);
    const clearCountBeforeInc = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedCountBeforeInc,
      fheCounterContractAddress,
      signers.alice,
    );
    progress(`Clear FHECounter.getCount()=${clearCountBeforeInc}`);

    progress(`Encrypting '1'...`);
    const encryptedOne = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(1)
      .encrypt();

    progress(
      `Call increment(1) FHECounter=${fheCounterContractAddress} handle=${ethers.hexlify(encryptedOne.handles[0])} signer=${signers.alice.address}...`,
    );
    tx = await fheCounterContract.connect(signers.alice).increment(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    progress(`Call FHECounter.getCount()...`);
    const encryptedCountAfterInc = await fheCounterContract.getCount();

    progress(`Decrypting FHECounter.getCount()=${encryptedCountAfterInc}...`);
    const clearCountAfterInc = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedCountAfterInc,
      fheCounterContractAddress,
      signers.alice,
    );
    progress(`Clear FHECounter.getCount()=${clearCountAfterInc}`);

    expect(clearCountAfterInc - clearCountBeforeInc).to.eq(1);
  });
});
