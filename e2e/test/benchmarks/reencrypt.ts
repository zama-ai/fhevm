import { expect } from "chai";

import { deployReencryptFixture } from "../gateway/Reencrypt.fixture";
import { Decrypt, createDecrypt, createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { Timing, displayTimings } from "./utils";

describe("Benchmarks", function () {
  const timings: Timing[] = [];
  let decrypt: Decrypt;
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
    this.fhevm = await createInstance();
    const reencrypt = await deployReencryptFixture();
    this.reencryptAddress = await reencrypt.getAddress();
    this.reencrypt = reencrypt;
  });

  it("benchmarks reencrypt sequential", async function () {
    await new Promise((resolve) => setTimeout(resolve, 10000));
    decrypt = createDecrypt(this.fhevm, this.signers.alice, this.reencryptAddress);
    let reencryptTimings: Timing[] = [];

    const types = [1, 4, 8, 16, 32, 64, 128, 256];
    const reencryptPromise = types.reduce(async (p, type) => {
      const label = type === 1 ? "a boolean" : `a ${type}bits value`;
      await p;
      let timing: Timing = {
        description: `Reencrypt (sequential) ${label}`,
        time: 0,
      };
      let start = Date.now();
      let handle: bigint;
      if (type == 1) {
        handle = await this.reencrypt.resultBool();
      } else {
        handle = await this.reencrypt[`result${type}`]();
      }
      const result = await decrypt(handle);
      timing.time = Date.now() - start;
      reencryptTimings.push(timing);
    }, Promise.resolve());

    await reencryptPromise;
    displayTimings(reencryptTimings);
  });

  it("benchmarks reencrypt parrallel", async function () {
    const reencrypt = await deployReencryptFixture();
    this.reencryptAddress = await reencrypt.getAddress();
    this.reencrypt = reencrypt;
    await new Promise((resolve) => setTimeout(resolve, 10000));
    decrypt = createDecrypt(this.fhevm, this.signers.alice, this.reencryptAddress);
    let reencryptTimings: Timing[] = [];

    const types = [1, 4, 8, 16, 32, 64, 128, 256];
    const reencryptPromises = types.map(async (type) => {
      const label = type === 1 ? "a boolean" : `a ${type}bits value`;
      let timing: Timing = {
        description: `Reencrypt (parrallel) ${label}`,
        time: 0,
      };
      let start = Date.now();
      let handle: bigint;
      if (type == 1) {
        handle = await this.reencrypt.resultBool();
      } else {
        handle = await this.reencrypt[`result${type}`]();
      }
      const result = await decrypt(handle);
      timing.time = Date.now() - start;
      reencryptTimings.push(timing);
    });

    await Promise.all(reencryptPromises);
    displayTimings(reencryptTimings);
  });
});
