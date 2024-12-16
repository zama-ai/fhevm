import { expect } from 'chai';
import { randomBytes } from 'crypto';
import { ethers } from 'hardhat';

import { getSigners, initSigners } from '../signers';

describe('TFHE revert paths', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contractFactory = await ethers.getContractFactory('TFHERevertTest');
    const contract = await contractFactory.connect(this.signers.alice).deploy();
    await contract.waitForDeployment();

    this.contract = contract;
  });

  it('padToBytes64 reverts if input > 64 bytes', async function () {
    const numberBytes = 65;
    const input = randomBytes(numberBytes);

    await expect(this.contract.padToBytes64(input))
      .to.be.revertedWithCustomError(this.contract, 'InputLengthAbove64Bytes')
      .withArgs(numberBytes);
  });

  it('padToBytes128 reverts if input > 128 bytes', async function () {
    const numberBytes = 129;
    const input = randomBytes(numberBytes);

    await expect(this.contract.padToBytes128(input))
      .to.be.revertedWithCustomError(this.contract, 'InputLengthAbove128Bytes')
      .withArgs(numberBytes);
  });

  it('padToBytes256 reverts if input > 256 bytes', async function () {
    const numberBytes = 257;
    const input = randomBytes(numberBytes);

    await expect(this.contract.padToBytes256(input))
      .to.be.revertedWithCustomError(this.contract, 'InputLengthAbove256Bytes')
      .withArgs(numberBytes);
  });
});
