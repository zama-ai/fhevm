import { assert, expect } from 'chai';
import type { Contract } from 'ethers';
import { ethers } from 'hardhat';

import { createInstance } from '../instance';
import { getSigner } from '../signers';

/// PROBE ONLY. Drives the coprocessor's multi-output path end to end with the
/// synthetic `FHE.reverse` operator: one operation, N output handles.
describe('multi-output operations', function () {
  // Asserting a handle is NOT decryptable costs a full relayer readiness
  // timeout (12 retries x 8s), and one test does that twice on top of a real
  // decrypt, which does not fit the 300s default.
  this.timeout(900_000);

  before(async function () {
    this.signer = await getSigner(119);
    this.instance = await createInstance();
    const factory = await ethers.getContractFactory('FHEVMMultiOutputTestSuite');
    const contract = await factory.connect(this.signer).deploy();
    await contract.waitForDeployment();
    this.suite = contract as unknown as Contract;
  });

  const handlesFromResults = async (suite: Contract): Promise<string[]> => {
    const length = Number(await suite.resultsLength());
    const handles: string[] = [];
    for (let i = 0; i < length; i++) {
      handles.push((await suite.resultAt(i)) as string);
    }
    return handles;
  };

  const decryptAll = async (instance: any, handles: string[]): Promise<bigint[]> => {
    // Duplicate handles are legitimate (see the dedup case), and publicDecrypt
    // is keyed by handle, so ask once per distinct handle and re-expand.
    const distinct = [...new Set(handles)];
    const res = await instance.publicDecrypt(distinct);
    return handles.map((h) => res.clearValues[h as `0x${string}`] as bigint);
  };

  it('multi-output binds every result to its own handle', async function () {
    const values = [11n, 22n, 33n, 44n];
    const tx = await this.suite.reverseRevealAll(values);
    await tx.wait();

    const handles = await handlesFromResults(this.suite);
    assert.equal(handles.length, 4, 'one operation must produce four handles');
    assert.equal(new Set(handles).size, 4, 'each output needs a distinct handle');

    const decrypted = await decryptAll(this.instance, handles);
    // output i holds input n-1-i: a routing error shows up as a wrong value,
    // not as a coincidentally correct one.
    assert.deepEqual(decrypted, [44n, 33n, 22n, 11n]);
  });

  it('multi-output keeps a permission per output, not per operation', async function () {
    const values = [7n, 8n, 9n, 10n];
    // Reveal outputs 1 and 2 only. Leaving output 0 unrevealed is deliberate:
    // the worker reads a group's op-level fields off the output_index 0 row, so
    // an unallowed first output is the case most likely to mis-handle a group.
    const tx = await this.suite.reverseRevealSome(values, [1, 2]);
    await tx.wait();

    const handles = await handlesFromResults(this.suite);
    assert.equal(handles.length, 4);

    const revealed = await decryptAll(this.instance, [handles[1], handles[2]]);
    assert.deepEqual(revealed, [9n, 8n], 'revealed outputs decrypt to their own values');

    // The unrevealed siblings must not be publicly decryptable, even though
    // the operation they came from succeeded.
    await expect(this.instance.publicDecrypt([handles[0]])).to.be.rejected;
    await expect(this.instance.publicDecrypt([handles[3]])).to.be.rejected;
  });

  it('multi-output feeds a consumer from the sibling it names', async function () {
    const values = [100n, 200n, 300n, 400n];
    // out[1] == 300; add 5.
    const tx = await this.suite.reverseThenAdd(values, 1, 5n);
    await tx.wait();

    const handles = await handlesFromResults(this.suite);
    assert.equal(handles.length, 1);
    const [sum] = await decryptAll(this.instance, handles);
    assert.equal(sum, 305n, 'the consumer must read output 1, not output 0');
  });

  it('multi-output groups in one transaction stay separate', async function () {
    const tx = await this.suite.reverseTwoGroups([1n, 2n, 3n], [4n, 5n, 6n]);
    await tx.wait();

    const handles = await handlesFromResults(this.suite);
    assert.equal(handles.length, 6, 'two groups of three');
    assert.equal(new Set(handles).size, 6, 'distinct inputs must mint distinct handles per group');

    const decrypted = await decryptAll(this.instance, handles);
    assert.deepEqual(decrypted, [3n, 2n, 1n, 6n, 5n, 4n], 'neither group may overwrite the other');
  });

  it('multi-output dedups an identical group without truncating it', async function () {
    // Handles are content-addressed and no per-call nonce enters the preimage,
    // so two identical reverses derive the same three handles. The listener's
    // ON CONFLICT DO NOTHING must leave one complete group of three behind --
    // not a group whose row count disagrees with its declared output_count.
    // Inputs unique to this test: reusing the separation test's [1,2,3] would
    // derive handles it already computed, so a dedup failure could still pass.
    const tx = await this.suite.reverseTwoGroups([71n, 72n, 73n], [71n, 72n, 73n]);
    await tx.wait();

    const handles = await handlesFromResults(this.suite);
    assert.equal(handles.length, 6, 'the contract still stores six entries');
    assert.equal(new Set(handles).size, 3, 'identical operations derive identical handles');
    assert.deepEqual(handles.slice(0, 3), handles.slice(3), 'the second group repeats the first');

    // The decisive part: the deduped group still computes. If output_count were
    // written as 6, or rows dropped, every handle here would fail to resolve.
    const decrypted = await decryptAll(this.instance, handles);
    assert.deepEqual(decrypted, [73n, 72n, 71n, 73n, 72n, 71n]);
  });

  it('multi-output rejects a group larger than the operator cap', async function () {
    const tooMany = Array.from({ length: 9 }, (_, i) => BigInt(i + 1));
    await expect(this.suite.reverseRevealAll(tooMany)).to.be.reverted;
  });
});
