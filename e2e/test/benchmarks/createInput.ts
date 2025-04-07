import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { Timing, displayTimings } from "./utils";

describe("Benchmark input creation", function () {
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
    this.httpz = await createInstance();
  });

  it("should create 10 inputs and send them in parrallel", async function () {
    const input = this.httpz.createEncryptedInput(
      "0x1337AA343Db8D44238Fe40486aDeECdf354e1f60",
      this.signers.alice.address,
    );
    input.add4(9n);
    input.add128(13n);
    const ciphertext = await input._prove();
    const timings: Timing[] = await Promise.all(
      new Array(10).fill(null).map(async (_, i) => {
        const timing: Timing = {
          description: `Create input ${i}`,
          time: 0,
        };
        const start = Date.now();
        await input._verify(ciphertext);
        timing.time = Date.now() - start;
        return timing;
      }),
    );
    displayTimings(timings);
  });
});
