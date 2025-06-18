```ts
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";
import { Counter, Counter__factory } from "../types";
import { expect } from "chai";

type Signers = {
  deployer: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
};

async function deployFixture() {
  const factory = (await ethers.getContractFactory("Counter")) as Counter__factory;
  const counterContract = (await factory.deploy()) as Counter;
  const counterContractAddress = await counterContract.getAddress();

  return { counterContract, counterContractAddress };
}

describe("Counter", function () {
  let signers: Signers;
  let counterContract: Counter;

  before(async function () {
    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { deployer: ethSigners[0], alice: ethSigners[1], bob: ethSigners[2] };
  });

  beforeEach(async () => {
    ({ counterContract } = await deployFixture());
  });

  it("count should be zero after deployment", async function () {
    const count = await counterContract.getCount();
    console.log(`Counter.getCount() === ${count}`);
    // Expect initial count to be 0 after deployment
    expect(count).to.eq(0);
  });

  it("increment the counter by 1", async function () {
    const countBeforeInc = await counterContract.getCount();
    const tx = await counterContract.connect(signers.alice).increment();
    await tx.wait();
    const countAfterInc = await counterContract.getCount();
    expect(countAfterInc).to.eq(countBeforeInc + 1n);
  });

  it("decrement the counter by 1", async function () {
    // First increment, count becomes 1
    let tx = await counterContract.connect(signers.alice).increment();
    await tx.wait();
    // Then decrement, count goes back to 0
    tx = await counterContract.connect(signers.alice).decrement();
    await tx.wait();
    const count = await counterContract.getCount();
    expect(count).to.eq(0);
  });
});
```