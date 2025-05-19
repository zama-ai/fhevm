import { expect } from "chai";

import { deployEncryptedERC20Fixture } from "../encryptedERC20/EncryptedERC20.fixture";
import { Decrypt, createDecrypt, createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { Timing, displayTimings } from "./utils";

describe("Benchmark ERC20", function () {
  const timings: Timing[] = [];
  let decrypt: Decrypt;
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
    this.httpz = await createInstance();
    const erc20 = await deployEncryptedERC20Fixture();
    this.erc20Address = await erc20.getAddress();
    this.erc20 = erc20;
  });

  it("benchmarks erc20", async function () {
    decrypt = createDecrypt(this.httpz, this.signers.alice, [this.erc20Address]);
    // Minting the contract
    const mintTiming: Timing = {
      description: "Mint 1000 tokens",
      time: 0,
    };
    let start = Date.now();
    const txMint = await this.erc20.mint(1000);
    const t1 = await txMint.wait();
    expect(t1.status).to.eq(1);
    mintTiming.time = Date.now() - start;
    timings.push(mintTiming);

    // Create input
    const inputTiming: Timing = {
      description: "Create an input with euint64 500",
      time: 0,
    };
    start = Date.now();
    const input = this.httpz.createEncryptedInput(this.erc20Address, this.signers.alice.address);
    input.add64(500);
    const encryptedTransferAmount = await input.encrypt();
    inputTiming.time = Date.now() - start;
    timings.push(inputTiming);

    // Transfer
    const transferTiming: Timing = {
      description: "Transfer 500 tokens",
      time: 0,
    };
    start = Date.now();
    const tx = await this.erc20["transfer(address,bytes32,bytes)"](
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    const t2 = await tx.wait();
    expect(t2?.status).to.eq(1);
    transferTiming.time = Date.now() - start;
    timings.push(transferTiming);

    // Bench reencrypt
    const reencryptTiming: Timing = {
      description: "Reencryption of a euint64 balance",
      time: 0,
    };
    await new Promise((resolve) => setTimeout(resolve, 10000));
    start = Date.now();
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);
    await decrypt([{ ctHandle: balanceHandleAlice, contractAddress: this.erc20Address }]);
    reencryptTiming.time = Date.now() - start;
    timings.push(reencryptTiming);

    console.log(displayTimings(timings));
  });
});
