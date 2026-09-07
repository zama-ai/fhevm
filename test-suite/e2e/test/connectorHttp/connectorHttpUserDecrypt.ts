import { expect } from 'chai';
import { ethers } from 'hardhat';

import { UserDecrypt } from '../../types';
import { createInstances, verifyingContractAddressDecryption } from '../instance';
import {
  USER_DECRYPT_ROUTE,
  type UserDecryptionResponse,
  buildUserRequest,
  fanOut,
  fanOutUntilSettled,
} from '../sdk/connector/connectorHttp';
import { verifyUserDecryptResponses } from '../sdk/connector/verify';
import { type Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';
import { connectorSetup } from './setup';

const POSITIVE_TIMEOUT_MS = 4 * 60 * 1000;

const TYPED_HANDLES = [
  ['ebool', 'xBool'],
  ['euint8', 'xUint8'],
  ['euint16', 'xUint16'],
  ['euint32', 'xUint32'],
  ['euint64', 'xUint64'],
  ['euint128', 'xUint128'],
  ['eaddress', 'xAddress'],
  ['euint256', 'xUint256'],
] as const;

describe('Connector HTTP user decrypt', function () {
  let signers: Signers;
  let instances: FhevmInstances;
  let contract: UserDecrypt;
  let contractAddress: string;
  let publicKey: string;

  before(async function () {
    this.timeout(180_000);
    await connectorSetup(this);
    await initSigners(2);
    signers = await getSigners();
    instances = await createInstances(signers);
    const factory = await ethers.getContractFactory('UserDecrypt');
    contract = await factory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
    contractAddress = await contract.getAddress();
    publicKey = (await instances.alice.generateKeypair()).publicKey;
  });

  const aliceRequest = async (handles: string[], allowedContracts: string[] = [contractAddress]) =>
    buildUserRequest(
      {
        handles: handles.map((handle) => ({ handle, contractAddress, ownerAddress: signers.alice.address })),
        userAddress: signers.alice.address,
        publicKey,
        allowedContracts,
        decryptionContractAddress: verifyingContractAddressDecryption,
      },
      signers.alice,
    );

  it('test connector endpoint user decrypt euint64 owned by alice', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    const body = await aliceRequest([await contract.xUint64()]);
    const out = await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, body);
    verifyUserDecryptResponses(out.results);
  });

  for (const [type, getter] of TYPED_HANDLES) {
    it(`test connector endpoint user decrypt ${type}`, async function () {
      this.timeout(POSITIVE_TIMEOUT_MS);
      const handle = (await contract.getFunction(getter)()) as string;
      const out = await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, await aliceRequest([handle]));
      verifyUserDecryptResponses(out.results);
    });
  }

  it('test connector endpoint user decrypt permissive mode (empty allowedContracts)', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    const out = await fanOutUntilSettled<UserDecryptionResponse>(
      USER_DECRYPT_ROUTE,
      await aliceRequest([await contract.xUint8()], []),
    );
    verifyUserDecryptResponses(out.results);
  });

  it('test connector endpoint user decrypt multi-handle batch under one permit', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    const handles = [await contract.xBool(), await contract.xUint32(), await contract.xAddress()];
    const out = await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, await aliceRequest(handles));
    verifyUserDecryptResponses(out.results);
  });

  it('test connector endpoint user decrypt identical permit is served from cache', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    const body = await aliceRequest([await contract.xUint16()]);
    const first = await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, body);
    const { decryptionId } = verifyUserDecryptResponses(first.results);

    const cached = await fanOut<UserDecryptionResponse>(USER_DECRYPT_ROUTE, body);
    expect(cached.failures.map((r) => r.url)).to.deep.equal([]);
    for (const [i, r] of cached.results.entries()) {
      expect(r.body).to.deep.equal(first.results[i].body);
      expect((r.body as UserDecryptionResponse).decryptionId).to.equal(decryptionId);
      expect(r.elapsedMs, `${r.url} took ${r.elapsedMs}ms for a cached answer`).to.be.lessThan(5_000);
    }
  });

  it('test connector endpoint user decrypt leaves the SDK path intact on the same handle', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    const handle = await contract.xUint64();
    await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, await aliceRequest([handle]));
    const viaSdk = await instances.alice.userDecryptSingleHandle({ handle, contractAddress, signer: signers.alice });
    expect(viaSdk).to.equal(18446744073709551600n);
  });
});
