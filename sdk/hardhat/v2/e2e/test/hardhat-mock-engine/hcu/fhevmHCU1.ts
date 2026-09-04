import { getHCU } from '@fhevm/hardhat-plugin';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import * as hre from 'hardhat';

import type { FHEVMTestSuite1 } from '../../../typechain-types/contracts/operators/FHEVMTestSuite1';
import { getSigners, initSigners, type Signers } from '../signers';
import { requireReceipt } from '../utils';

async function deployFHEVMTestFixture1(): Promise<FHEVMTestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM HCU 1', function () {
  let contract1Address: string;
  let signers: Signers;
  let contract1: FHEVMTestSuite1;

  before(async function () {
    await initSigners();
    signers = await getSigners();

    const c1: FHEVMTestSuite1 = await deployFHEVMTestFixture1();
    contract1Address = await c1.getAddress();
    contract1 = c1;
  });

  it('test HCU FheAdd(euint8, euint8)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract1Address, signers.alice.address);
    input.add8(80n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract1.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    const receipt = requireReceipt(await tx.wait());

    const resEuint8 = (await contract1.resEuint8()) as `0x${string}`;

    const hcu = hre.fhevm.computeTransactionHCU(receipt);

    expect(hcu.globalHCU).to.eq(getHCU('FheAdd', 'Uint8'));
    expect(hcu.HCUDepthByHandle[resEuint8]).to.eq(getHCU('FheAdd', 'Uint8'));
    expect(hcu.maxHCUDepth).to.eq(getHCU('FheAdd', 'Uint8'));
  });

  it('test HCU FheSub(euint8, euint8)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract1Address, signers.alice.address);
    input.add8(133n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract1.sub_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    const receipt = requireReceipt(await tx.wait());

    const resEuint8 = (await contract1.resEuint8()) as `0x${string}`;

    const hcu = hre.fhevm.computeTransactionHCU(receipt);

    expect(hcu.globalHCU).to.eq(getHCU('FheSub', 'Uint8'));
    expect(hcu.HCUDepthByHandle[resEuint8]).to.eq(getHCU('FheSub', 'Uint8'));
    expect(hcu.maxHCUDepth).to.eq(getHCU('FheSub', 'Uint8'));
  });
});
