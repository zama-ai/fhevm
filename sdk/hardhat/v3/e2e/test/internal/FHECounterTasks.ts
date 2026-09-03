import { expect } from 'chai';
import { network, tasks } from 'hardhat';

import type {
  FHECounterPublicDecrypt,
  FHECounterPublicDecrypt__factory,
  FHECounterUserDecrypt,
  FHECounterUserDecrypt__factory,
} from '../../types/ethers-contracts/index.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

// `hardhat fhevm public-decrypt` / `user-decrypt`, run programmatically: the task actions open the
// same cached connection this suite uses (`getOrCreate()` with no network), so they see the counters.

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

describe('fhevm tasks', function () {
  let signers: Signers;

  before(async function () {
    signers = await getSigners(connection);
  });

  beforeEach(function () {
    if (!fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }
  });

  async function encryptOne32(contract: Hex, value: number): Promise<{ handle: Hex; inputProof: Hex }> {
    const encrypted = await fhevm
      .createEncryptedInput(contract, signers.alice.address as Hex)
      .add32(value)
      .encrypt();
    const [handle] = encrypted.handles;
    if (handle === undefined) throw new Error('encrypt() returned no handle');
    return { handle, inputProof: encrypted.inputProof };
  }

  it('fhevm public-decrypt prints a publicly decryptable count', async function () {
    const factory: FHECounterPublicDecrypt__factory = await ethers.getContractFactory('FHECounterPublicDecrypt');
    const counter: FHECounterPublicDecrypt = await factory.deploy();
    const counterAddress = (await counter.getAddress()) as Hex;
    const input = await encryptOne32(counterAddress, 9);
    await (await counter.connect(signers.alice).increment(input.handle, input.inputProof)).wait();
    const count = (await counter.getCount()) as Hex;

    const value: unknown = await tasks.getTask(['fhevm', 'public-decrypt']).run({ type: 'euint32', handle: count });
    expect(value).to.eq(9n);
  });

  it("fhevm user-decrypt prints a count for the network's account", async function () {
    const factory: FHECounterUserDecrypt__factory = await ethers.getContractFactory('FHECounterUserDecrypt');
    const counter: FHECounterUserDecrypt = await factory.deploy();
    const counterAddress = (await counter.getAddress()) as Hex;
    const input = await encryptOne32(counterAddress, 4);
    // alice is account #0 of the suite's mnemonic.
    await (await counter.connect(signers.alice).increment(input.handle, input.inputProof)).wait();
    const count = (await counter.getCount()) as Hex;

    const value: unknown = await tasks
      .getTask(['fhevm', 'user-decrypt'])
      .run({ type: 'euint32', handle: count, contract: counterAddress, user: 0 });
    expect(value).to.eq(4n);
  });
});
