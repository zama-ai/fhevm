import { expect } from "chai";

import { deployEncryptedERC20Fixture } from "../encryptedERC20/EncryptedERC20.fixture";
import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";

type Timing = {
  description: string;
  time: number;
};

describe("Benchmarks", function () {
  const timings: Timing[] = [];
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
  });

  before(async function () {
    const erc20 = await deployEncryptedERC20Fixture();
    this.erc20Address = await erc20.getAddress();
    this.erc20 = erc20;
    this.fhevm = await createInstance();
  });

  it("benchmark erc20", async function () {
    // Minting the contract
    let mintTiming: Timing = {
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
    let inputTiming: Timing = {
      description: "Create an input with euint64 500",
      time: 0,
    };
    start = Date.now();
    const input = this.fhevm.createEncryptedInput(this.erc20Address, this.signers.alice.address);
    input.add64(500);
    const encryptedTransferAmount = await input.encrypt();
    inputTiming.time = Date.now() - start;
    timings.push(inputTiming);

    // Transfer
    let transferTiming: Timing = {
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

    const { publicKey: publicKeyAlice, privateKey: privateKeyAlice } = this.fhevm.generateKeypair();
    const eip712 = this.fhevm.createEIP712(publicKeyAlice, this.erc20Address);

    let reencryptTiming: Timing = {
      description: "Reencryption of a euint64 balance",
      time: 0,
    };
    await new Promise((resolve) => setTimeout(resolve, 10000));
    start = Date.now();
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);
    const signatureAlice = await this.signers.alice.signTypedData(
      eip712.domain,
      { Reencrypt: eip712.types.Reencrypt },
      eip712.message,
    );
    const balanceAlice = await this.fhevm.reencrypt(
      balanceHandleAlice,
      privateKeyAlice,
      publicKeyAlice,
      signatureAlice.replace("0x", ""),
      this.erc20Address,
      this.signers.alice.address,
    );
    reencryptTiming.time = Date.now() - start;
    timings.push(reencryptTiming);

    console.log(timings);
  });
});
