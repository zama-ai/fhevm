import { Decrypt, createDecrypt, createInstance } from "../instance";
import { deployUserDecryptFixture } from "../relayer/UserDecrypt.fixture";
import { getSigners, initSigners } from "../signers";
import { Timing, displayTimings } from "./utils";

describe("Benchmark user decrypt", function () {
  let decrypt: Decrypt;
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
    this.httpz = await createInstance();
    const userDecrypt = await deployUserDecryptFixture();
    this.userDecryptAddress = await userDecrypt.getAddress();
    this.userDecrypt = userDecrypt;
  });

  it("benchmarks user decryption sequential", async function () {
    await new Promise((resolve) => setTimeout(resolve, 10000));
    decrypt = createDecrypt(this.httpz, this.signers.alice, [this.userDecryptAddress]);
    const userDecryptTimings: Timing[] = [];

    const types = [1, 4, 8, 16, 32, 64, 128, 256];
    const userDecryptPromise = types.reduce(async (p, type) => {
      const label = type === 1 ? "a boolean" : `a ${type}bits value`;
      await p;
      const timing: Timing = {
        description: `User decrypt (sequential) ${label}`,
        time: 0,
      };
      const start = Date.now();
      let handle: bigint;
      if (type == 1) {
        handle = await this.userDecrypt.resultBool();
      } else {
        handle = await this.userDecrypt[`result${type}`]();
      }
      await decrypt([{ ctHandle: handle, contractAddress: this.userDecryptAddress }]);
      timing.time = Date.now() - start;
      userDecryptTimings.push(timing);
    }, Promise.resolve());

    await userDecryptPromise;
    displayTimings(userDecryptTimings);
  });

  it("benchmarks user decryption parrallel", async function () {
    const userDecrypt = await deployUserDecryptFixture();
    this.userDecryptAddress = await userDecrypt.getAddress();
    this.userDecrypt = userDecrypt;
    await new Promise((resolve) => setTimeout(resolve, 10000));
    decrypt = createDecrypt(this.httpz, this.signers.alice, [this.userDecryptAddress]);
    const userDecryptTimings: Timing[] = [];

    const types = [1, 4, 8, 16, 32, 64, 128, 256];
    const userDecryptPromises = types.map(async (type) => {
      const label = type === 1 ? "a boolean" : `a ${type}bits value`;
      const timing: Timing = {
        description: `User decrypt (parrallel) ${label}`,
        time: 0,
      };
      const start = Date.now();
      let handle: bigint;
      if (type == 1) {
        handle = await this.userDecrypt.resultBool();
      } else {
        handle = await this.userDecrypt[`result${type}`]();
      }
      await decrypt([{ ctHandle: handle, contractAddress: this.userDecryptAddress }]);
      timing.time = Date.now() - start;
      userDecryptTimings.push(timing);
    });

    await Promise.all(userDecryptPromises);
    displayTimings(userDecryptTimings);
  });
});
