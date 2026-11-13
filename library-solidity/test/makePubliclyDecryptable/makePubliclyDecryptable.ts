import { expect } from 'chai';

import type { MakePubliclyDecryptable } from '../../typechain-types/index.js';
import { MakePubliclyDecryptable__factory } from '../../typechain-types/index.js';
import { getSigners, initSigners } from '../signers.js';

describe('MakePubliclyDecryptable', function () {
  let contract: MakePubliclyDecryptable;

  before(async function () {
    await initSigners(2);
    const signers = await getSigners();
    contract = await new MakePubliclyDecryptable__factory(signers.alice).deploy();
    await contract.waitForDeployment();
  });

  it('test MakePubliclyDecryptable ebool', async function () {
    const isPubliclyDecryptableBefore = await contract.isPubliclyDecryptableBool();
    expect(isPubliclyDecryptableBefore).to.equal(false);
    const tx = await contract.makePubliclyDecryptableBool();
    await tx.wait();
    const isPubliclyDecryptableAfter = await contract.isPubliclyDecryptableBool();
    expect(isPubliclyDecryptableAfter).to.equal(true);
  });

  it('test MakePubliclyDecryptable euint8', async function () {
    const isPubliclyDecryptableBefore = await contract.isPubliclyDecryptableUint8();
    expect(isPubliclyDecryptableBefore).to.equal(false);
    const tx = await contract.makePubliclyDecryptableUint8();
    await tx.wait();
    const isPubliclyDecryptableAfter = await contract.isPubliclyDecryptableUint8();
    expect(isPubliclyDecryptableAfter).to.equal(true);
  });
});
