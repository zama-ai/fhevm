import { FhevmType } from '@fhevm/hardhat-plugin';
import { expect } from 'chai';
import { network } from 'hardhat';

import type { FHECounterPublicDecrypt, FHECounterPublicDecrypt__factory } from '../../types/ethers-contracts/index.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

// The debugger is the test-only read that ignores the ACL: v2's operator suites lean on it ~2,500
// times (E2E-0b). Here the counter is incremented WITHOUT `makePubliclyDecryptable`, so the
// permissioned path is refused while the debugger still reads the count.

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

describe('FHECounter debugger', function () {
  let signers: Signers;
  let counter: FHECounterPublicDecrypt;
  let counterAddress: Hex;

  before(async function () {
    signers = await getSigners(connection);
  });

  beforeEach(async () => {
    if (!fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }
    const factory: FHECounterPublicDecrypt__factory = await ethers.getContractFactory('FHECounterPublicDecrypt');
    counter = await factory.deploy();
    counterAddress = (await counter.getAddress()) as Hex;
  });

  it('reads a count nobody is allowed to decrypt', async function () {
    const encrypted = await fhevm
      .createEncryptedInput(counterAddress, signers.alice.address as Hex)
      .add32(5)
      .encrypt();
    const [handle] = encrypted.handles;
    if (handle === undefined) throw new Error('encrypt() returned no handle');
    const tx = await counter.connect(signers.alice).incrementNotPubliclyDecryptable(handle, encrypted.inputProof);
    await tx.wait();
    const count = (await counter.getCount()) as Hex;

    let refused = false;
    try {
      await fhevm.publicDecryptEuint(FhevmType.euint32, count);
    } catch {
      refused = true;
    }
    expect(refused).to.eq(true);

    expect(await fhevm.debugger.decryptEuint(FhevmType.euint32, count)).to.eq(5n);
    expect(fhevm.typeof(count)).to.eq('euint32');
  });
});
