import { getHCU } from '@fhevm/hardhat-plugin-v3';
import { expect } from 'chai';
import { network } from 'hardhat';

import type { FHEVMTestSuite1, FHEVMTestSuite1__factory } from '../../types/ethers-contracts/index.ts';
import { requireReceipt } from '../utils/receipts.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

async function deployFHEVMTestFixture1(signers: Signers): Promise<FHEVMTestSuite1> {
  const admin = signers.alice;

  const contractFactory: FHEVMTestSuite1__factory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

// Encrypts two euint8 for alice under one proof; `handles` is `Hex[]`, so both are narrowed here once.
async function encryptTwo8(
  contractAddress: Hex,
  user: Hex,
  a: bigint,
  b: bigint,
): Promise<{ handleA: Hex; handleB: Hex; inputProof: Hex }> {
  const encrypted = await fhevm.createEncryptedInput(contractAddress, user).add8(a).add8(b).encrypt();
  const [handleA, handleB] = encrypted.handles;
  if (handleA === undefined || handleB === undefined) throw new Error('encrypt() returned fewer than two handles');
  return { handleA, handleB, inputProof: encrypted.inputProof };
}

describe('FHEVM HCU 1', function () {
  let contract1Address: Hex;
  let signers: Signers;
  let contract1: FHEVMTestSuite1;

  before(async function () {
    signers = await getSigners(connection);

    const c1: FHEVMTestSuite1 = await deployFHEVMTestFixture1(signers);
    contract1Address = (await c1.getAddress()) as Hex;
    contract1 = c1;
  });

  it('test HCU FheAdd(euint8, euint8)', async function () {
    const encryptedAmount = await encryptTwo8(contract1Address, signers.alice.address as Hex, 80n, 133n);
    const tx = await contract1.add_euint8_euint8(
      encryptedAmount.handleA,
      encryptedAmount.handleB,
      encryptedAmount.inputProof,
    );
    const receipt = requireReceipt(await tx.wait());

    const resEuint8 = (await contract1.resEuint8()) as Hex;

    const hcu = fhevm.computeTransactionHCU(receipt);

    expect(hcu.globalHCU).to.eq(getHCU('FheAdd', 'Uint8'));
    expect(hcu.HCUDepthByHandle[resEuint8]).to.eq(getHCU('FheAdd', 'Uint8'));
    expect(hcu.maxHCUDepth).to.eq(getHCU('FheAdd', 'Uint8'));
  });

  it('test HCU FheSub(euint8, euint8)', async function () {
    const encryptedAmount = await encryptTwo8(contract1Address, signers.alice.address as Hex, 133n, 80n);
    const tx = await contract1.sub_euint8_euint8(
      encryptedAmount.handleA,
      encryptedAmount.handleB,
      encryptedAmount.inputProof,
    );
    const receipt = requireReceipt(await tx.wait());

    const resEuint8 = (await contract1.resEuint8()) as Hex;

    const hcu = fhevm.computeTransactionHCU(receipt);

    expect(hcu.globalHCU).to.eq(getHCU('FheSub', 'Uint8'));
    expect(hcu.HCUDepthByHandle[resEuint8]).to.eq(getHCU('FheSub', 'Uint8'));
    expect(hcu.maxHCUDepth).to.eq(getHCU('FheSub', 'Uint8'));
  });
});
