import { FhevmType, getHCU } from '@fhevm/hardhat-plugin';
import { expect } from 'chai';
import { network } from 'hardhat';

import type { FHECounterUserDecrypt, FHECounterUserDecrypt__factory } from '../../types/ethers-contracts/index.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

// v2's HCU suite runs on the 752-line `FHEVMTestSuite1` corpus (E2E-0b). The counter carries the same
// arithmetic: the first increment adds to an UNINITIALIZED count, so `FHE.add` trivially encrypts a zero
// first (TrivialEncrypt + FheAdd); the second increment is one FheAdd on an initialized count.

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

describe('FHECounter HCU', function () {
  let signers: Signers;
  let counter: FHECounterUserDecrypt;
  let counterAddress: Hex;

  before(async function () {
    signers = await getSigners(connection);
  });

  beforeEach(async () => {
    if (!fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }
    const factory: FHECounterUserDecrypt__factory = await ethers.getContractFactory('FHECounterUserDecrypt');
    counter = await factory.deploy();
    counterAddress = (await counter.getAddress()) as Hex;
  });

  async function increment(value: number): Promise<{
    receipt: NonNullable<Awaited<ReturnType<Awaited<ReturnType<typeof counter.increment>>['wait']>>>;
    count: Hex;
  }> {
    const encrypted = await fhevm
      .createEncryptedInput(counterAddress, signers.alice.address as Hex)
      .add32(value)
      .encrypt();
    const [handle] = encrypted.handles;
    if (handle === undefined) throw new Error('encrypt() returned no handle');
    const tx = await counter.connect(signers.alice).increment(handle, encrypted.inputProof);
    const receipt = await tx.wait();
    if (receipt === null) throw new Error('Expected a transaction receipt');
    return { receipt, count: (await counter.getCount()) as Hex };
  }

  it('the first increment trivially encrypts the zero count, then adds', async function () {
    const { receipt, count } = await increment(1);
    const hcu = fhevm.computeTransactionHCU(receipt);

    const expected = getHCU('TrivialEncrypt', 'Uint32') + getHCU('FheAdd', 'Uint32');
    expect(hcu.globalHCU).to.eq(expected);
    expect(hcu.HCUDepthByHandle[count]).to.eq(expected);
    expect(hcu.maxHCUDepth).to.eq(expected);
    expect(fhevm.typeof(count)).to.eq('euint32');
    expect(FhevmType[FhevmType.euint32]).to.eq(fhevm.typeof(count));
  });

  it('the second increment is a single FheAdd on an initialized count', async function () {
    await increment(1);
    const { receipt, count } = await increment(2);
    const hcu = fhevm.computeTransactionHCU(receipt);

    expect(hcu.globalHCU).to.eq(getHCU('FheAdd', 'Uint32'));
    // Depth is per transaction: the previous count enters this receipt with no depth of its own.
    expect(hcu.HCUDepthByHandle[count]).to.eq(getHCU('FheAdd', 'Uint32'));
  });
});
