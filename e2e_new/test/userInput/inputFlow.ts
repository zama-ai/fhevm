import { expect } from "chai";
import { ethers } from "hardhat";

import { createInstances } from "../instance";
import { getSigners, initSigners } from "../signers";

describe("Input Flow", function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory("TestInput");
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    this.contractAddress = await this.contract.getAddress();
    await this.contract.waitForDeployment();
    this.instances = await createInstances(this.signers);
  });

  it("test user input uint64 (non-trivial)", async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(
      this.contractAddress,
      this.signers.alice.address
    );
    inputAlice.add64(18446744073709550042n);
    const encryptedAmount = await inputAlice.encrypt();
    encryptedAmount.handles.forEach((handle: any, index: any) => {
      // Assuming handle is a Uint8Array or Buffer
      console.log(
        `  Handle ${index}: 0x${Buffer.from(handle).toString("hex")}`
      );
    });
    console.log(
      "InputProof: 0x" + Buffer.from(encryptedAmount.inputProof).toString("hex")
    );
    const tx = await this.contract.requestUint64NonTrivial(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof
    );
    const receipt = await tx.wait();
    expect(receipt.status).to.equal(1);
  });
});
