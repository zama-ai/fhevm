import { FhevmType } from '@fhevm/hardhat-plugin-v3';
import { expect } from 'chai';
import { network } from 'hardhat';

import type { FHEVMManualTestSuiteWithAllowSender } from '../../types/ethers-contracts/index.ts';
import { type Signers, accountFor, getSigners } from '../utils/signers.ts';

// The v2 file, mechanically adapted: `hre.fhevm` → the connection's `fhevm`, handles narrowed with `at`,
// result handles typed as Hex, decrypting users as viem accounts.
const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

// `handles` is `Hex[]`: the i-th handle, or a loud failure.
function at(handles: readonly Hex[], i: number): Hex {
  const handle = handles[i];
  if (handle === undefined) throw new Error(`encrypt() returned no handle #${String(i)}`);
  return handle;
}

async function deployFHEVMManualTestFixture(): Promise<FHEVMManualTestSuiteWithAllowSender> {
  const signers = await getSigners(connection);
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMManualTestSuiteWithAllowSender');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM manual operations using userDecrypt', function () {
  let signers: Signers;
  let contractAddress: Hex;
  let contract: FHEVMManualTestSuiteWithAllowSender;

  beforeEach(async function () {
    signers = await getSigners(connection);

    contract = await deployFHEVMManualTestFixture();
    contractAddress = (await contract.getAddress()) as Hex;
  });

  it('Select works returning if false', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addBool(false);
    input.add32(3);
    input.add32(4);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_select(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      at(encryptedAmount.handles, 2),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      (await contract.resEuint32()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(4);
  });

  it('Select works returning if true', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addBool(true);
    input.add32(3);
    input.add32(4);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_select(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      at(encryptedAmount.handles, 2),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      (await contract.resEuint32()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(3);
  });

  it('Select ebool', async function () {
    const tx = await contract.test_select_ebool(true, false, true);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
    const tx2 = await contract.test_select_ebool(false, false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);
  });

  it('Select works for eaddress returning if false', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addBool(false);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x8881f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_select_eaddress(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      at(encryptedAmount.handles, 2),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.userDecryptEaddress(
      (await contract.resAdd()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(ethers.getAddress('0x8881f109551bd432803012645ac136ddd64dba72'));
  });

  it('Select works for eaddress returning if true', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addBool(true);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x8881f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_select_eaddress(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      at(encryptedAmount.handles, 2),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.userDecryptEaddress(
      (await contract.resAdd()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(ethers.getAddress('0x8ba1f109551bd432803012645ac136ddd64dba72'));
  });

  it('ebool eq ebool', async function () {
    const tx = await contract.eqEbool(true, true);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
    const tx2 = await contract.eqEbool(false, false);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);
    const tx3 = await contract.eqEbool(false, true);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(false);
    const tx4 = await contract.eqEbool(true, false);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(false);
  });

  it('ebool eq ebool - ScalarL', async function () {
    const tx = await contract.eqEboolScalarL(true, true);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
    const tx2 = await contract.eqEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(false);
    const tx3 = await contract.eqEboolScalarL(false, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);
    const tx4 = await contract.eqEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(false);
  });

  it('ebool eq ebool - ScalarR', async function () {
    const tx = await contract.eqEboolScalarL(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
    const tx2 = await contract.eqEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(false);
    const tx3 = await contract.eqEboolScalarL(true, true);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);
    const tx4 = await contract.eqEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(false);
  });

  it('ebool ne ebool', async function () {
    const tx = await contract.neEbool(true, true);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
    const tx2 = await contract.neEbool(false, false);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(false);
    const tx3 = await contract.neEbool(false, true);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);
    const tx4 = await contract.neEbool(true, false);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool ne ebool - ScalarL', async function () {
    const tx = await contract.neEboolScalarL(true, true);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
    const tx2 = await contract.neEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);
    const tx3 = await contract.neEboolScalarL(false, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(false);
    const tx4 = await contract.neEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool ne ebool - ScalarR', async function () {
    const tx = await contract.neEboolScalarL(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
    const tx2 = await contract.neEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);
    const tx3 = await contract.neEboolScalarL(true, true);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(false);
    const tx4 = await contract.neEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('eaddress eq eaddress,eaddress true', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_eq_eaddress_eaddress(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
  });

  it('eaddress eq eaddress,eaddress false', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x9ba1f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_eq_eaddress_eaddress(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
  });

  it('eaddress eq scalar eaddress,address true', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_eq_eaddress_address(at(encryptedAmount.handles, 0), b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
  });

  it('eaddress eq scalar eaddress,address false', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_eq_eaddress_address(at(encryptedAmount.handles, 0), b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
  });

  it('eaddress eq scalar address,eaddress true', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_eq_address_eaddress(at(encryptedAmount.handles, 0), b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
  });

  it('eaddress eq scalar address,eaddress false', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_eq_address_eaddress(at(encryptedAmount.handles, 0), b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
  });

  it('eaddress ne eaddress,eaddress false', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_ne_eaddress_eaddress(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
  });

  it('eaddress ne eaddress,eaddress true', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    input.addAddress('0x9ba1f109551bd432803012645ac136ddd64dba72');
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_ne_eaddress_eaddress(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
  });

  it('eaddress ne scalar eaddress,address false', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_ne_eaddress_address(at(encryptedAmount.handles, 0), b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
  });

  it('eaddress ne scalar eaddress,address true', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_ne_eaddress_address(at(encryptedAmount.handles, 0), b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
  });

  it('eaddress ne scalar address,eaddress false', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x8ba1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_ne_address_eaddress(at(encryptedAmount.handles, 0), b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
  });

  it('eaddress ne scalar address,eaddress true', async function () {
    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address as Hex);
    input.addAddress('0x8ba1f109551bd432803012645ac136ddd64dba72');
    const b = '0x9aa1f109551bd432803012645ac136ddd64dba72';
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_ne_address_eaddress(at(encryptedAmount.handles, 0), b, encryptedAmount.inputProof);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
  });

  it('ebool to euint8 casting works with true', async function () {
    const tx = await contract.test_ebool_to_euint8_cast(true);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint8,
      (await contract.resEuint8()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(1);
  });

  it('ebool to euint8 casting works with false', async function () {
    const tx = await contract.test_ebool_to_euint8_cast(false);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint8,
      (await contract.resEuint8()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(0);
  });

  it('ebool to euint16 casting works with true', async function () {
    const tx = await contract.test_ebool_to_euint16_cast(true);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint16,
      (await contract.resEuint16()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(1);
  });

  it('ebool to euint16 casting works with false', async function () {
    const tx = await contract.test_ebool_to_euint16_cast(false);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint16,
      (await contract.resEuint16()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(0);
  });

  it('ebool to euint32 casting works with true', async function () {
    const tx = await contract.test_ebool_to_euint32_cast(true);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      (await contract.resEuint32()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(1);
  });

  it('ebool to euint32 casting works with false', async function () {
    const tx = await contract.test_ebool_to_euint32_cast(false);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      (await contract.resEuint32()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(0);
  });

  it('ebool to euint64 casting works with true', async function () {
    const tx = await contract.test_ebool_to_euint64_cast(true);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint64,
      (await contract.resEuint64()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(1);
  });

  it('ebool to euint64 casting works with false', async function () {
    const tx = await contract.test_ebool_to_euint64_cast(false);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint64,
      (await contract.resEuint64()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(0);
  });

  it('ebool to euint128 casting works with true', async function () {
    const tx = await contract.test_ebool_to_euint128_cast(true);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint128,
      (await contract.resEuint128()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(1);
  });

  it('ebool to euint128 casting works with false', async function () {
    const tx = await contract.test_ebool_to_euint128_cast(false);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint128,
      (await contract.resEuint128()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(0);
  });

  it('ebool to euint256 casting works with true', async function () {
    const tx = await contract.test_ebool_to_euint256_cast(true);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint256,
      (await contract.resEuint256()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(1);
  });

  it('ebool to euint256 casting works with false', async function () {
    const tx = await contract.test_ebool_to_euint256_cast(false);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint256,
      (await contract.resEuint256()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(0);
  });

  it('euint128 to euint8 casting works', async function () {
    const tx = await contract.test_euint128_to_euint8_cast(7668756464674969496544n);
    await tx.wait();
    const res = await fhevm.userDecryptEuint(
      FhevmType.euint8,
      (await contract.resEuint8()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(224n);
  });

  it('ebool not for false is true', async function () {
    const tx = await contract.test_ebool_not(false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(true);
  });

  it('ebool not for true is false', async function () {
    const tx = await contract.test_ebool_not(true);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
  });

  it('ebool and', async function () {
    const tx = await contract.test_ebool_and(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);

    const tx2 = await contract.test_ebool_and(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(false);

    const tx3 = await contract.test_ebool_and(true, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(false);

    const tx4 = await contract.test_ebool_and(true, true);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool or', async function () {
    const tx = await contract.test_ebool_or(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);

    const tx2 = await contract.test_ebool_or(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);

    const tx3 = await contract.test_ebool_or(true, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);

    const tx4 = await contract.test_ebool_or(true, true);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool xor', async function () {
    const tx = await contract.test_ebool_xor(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);

    const tx2 = await contract.test_ebool_xor(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);

    const tx3 = await contract.test_ebool_xor(true, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);

    const tx4 = await contract.test_ebool_xor(true, true);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(false);
  });

  it('ebool xor scalarL', async function () {
    const tx = await contract.test_ebool_xor_scalarL(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);

    const tx2 = await contract.test_ebool_xor_scalarL(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);

    const tx3 = await contract.test_ebool_xor_scalarL(true, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);

    const tx4 = await contract.test_ebool_xor_scalarL(true, true);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(false);
  });

  it('ebool xor scalarR', async function () {
    const tx = await contract.test_ebool_xor_scalarR(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);

    const tx2 = await contract.test_ebool_xor_scalarR(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);

    const tx3 = await contract.test_ebool_xor_scalarR(true, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);

    const tx4 = await contract.test_ebool_xor_scalarR(true, true);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(false);
  });

  it('ebool or scalarL', async function () {
    const tx = await contract.test_ebool_or_scalarL(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);

    const tx2 = await contract.test_ebool_or_scalarL(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);

    const tx3 = await contract.test_ebool_or_scalarL(true, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);

    const tx4 = await contract.test_ebool_or_scalarL(true, true);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool or scalarR', async function () {
    const tx = await contract.test_ebool_or_scalarR(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);

    const tx2 = await contract.test_ebool_or_scalarR(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);

    const tx3 = await contract.test_ebool_or_scalarR(true, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);

    const tx4 = await contract.test_ebool_or_scalarR(true, true);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool and scalarL', async function () {
    const tx = await contract.test_ebool_and_scalarL(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);

    const tx2 = await contract.test_ebool_and_scalarL(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(false);

    const tx3 = await contract.test_ebool_and_scalarL(true, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(false);

    const tx4 = await contract.test_ebool_and_scalarL(true, true);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool and scalarR', async function () {
    const tx = await contract.test_ebool_and_scalarR(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);

    const tx2 = await contract.test_ebool_and_scalarR(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(false);

    const tx3 = await contract.test_ebool_and_scalarR(true, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(false);

    const tx4 = await contract.test_ebool_and_scalarR(true, true);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool ne ebool', async function () {
    const tx = await contract.neEbool(true, true);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
    const tx2 = await contract.neEbool(false, false);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(false);
    const tx3 = await contract.neEbool(false, true);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(true);
    const tx4 = await contract.neEbool(true, false);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool ne ebool - ScalarL', async function () {
    const tx = await contract.neEboolScalarL(true, true);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
    const tx2 = await contract.neEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);
    const tx3 = await contract.neEboolScalarL(false, false);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(false);
    const tx4 = await contract.neEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });

  it('ebool ne ebool - ScalarR', async function () {
    const tx = await contract.neEboolScalarL(false, false);
    await tx.wait();
    const res = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res).to.equal(false);
    const tx2 = await contract.neEboolScalarL(false, true);
    await tx2.wait();
    const res2 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res2).to.equal(true);
    const tx3 = await contract.neEboolScalarL(true, true);
    await tx3.wait();
    const res3 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res3).to.equal(false);
    const tx4 = await contract.neEboolScalarL(true, false);
    await tx4.wait();
    const res4 = await fhevm.userDecryptEbool(
      (await contract.resEbool()) as Hex,
      contractAddress,
      accountFor(signers.alice),
    );
    expect(res4).to.equal(true);
  });
});
