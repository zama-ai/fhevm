import { expect } from 'chai';
import type { Contract } from 'ethers';
import { ethers } from 'hardhat';

import { UserDecrypt } from '../../types';
import { createInstances, verifyingContractAddressDecryption } from '../instance';
import {
  PUBLIC_DECRYPT_ROUTE,
  type PostResult,
  type PublicDecryptionResponse,
  RETRYABLE_BY_CODE,
  USER_DECRYPT_ROUTE,
  type UserDecryptionResponse,
  buildPublicRequest,
  buildUserRequest,
  describeResult,
  fanOut,
  fanOutUntilSettled,
  isError,
  quorum,
} from '../sdk/connector/connectorHttp';
import { type KmsSignerSet, verifyPublicDecryptQuorum } from '../sdk/connector/verify';
import { type Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';
import { connectorSetup, waitBlocks } from './setup';

const ASYNC_TIMEOUT_MS = 4 * 60 * 1000;
// Blocks to wait after an on-chain ACL change before the kms-workers' host reads observe it.
const PROPAGATION_BLOCKS = 3;

/** Asserts the connector `ErrorResponse` contract on one result. */
function expectError<T>(result: PostResult<T>, expected: { status: number; code: string }) {
  const where = describeResult(result);
  expect(result.httpStatus, where).to.equal(expected.status);
  expect(isError(result.body), where).to.equal(true);
  if (!isError(result.body)) return;
  expect(result.body.code, where).to.equal(expected.code);
  expect(result.body.retryable, where).to.equal(RETRYABLE_BY_CODE[expected.code]);
  // Worker-decided errors are stored under the request's content-derived id.
  expect(result.body.decryptionId, where).to.match(/^0x[0-9a-f]{64}$/i);
}

function expectErrorOnEveryParty<T>(results: PostResult<T>[], expected: { status: number; code: string }) {
  expect(results.length).to.be.greaterThan(0);
  for (const r of results) expectError(r, expected);
}

describe('Connector HTTP negative', function () {
  let signers: Signers;
  let instances: FhevmInstances;
  let kms: KmsSignerSet;
  let aliceContract: UserDecrypt;
  let aliceContractAddress: string;
  let bobContractAddress: string;
  let fixture: Contract;
  let publicKey: string;

  before(async function () {
    this.timeout(180_000);
    ({ kms } = await connectorSetup(this));
    await initSigners(2);
    signers = await getSigners();
    instances = await createInstances(signers);

    const userFactory = await ethers.getContractFactory('UserDecrypt');
    aliceContract = await userFactory.connect(signers.alice).deploy();
    await aliceContract.waitForDeployment();
    aliceContractAddress = await aliceContract.getAddress();
    const bobContract = await userFactory.connect(signers.bob).deploy();
    await bobContract.waitForDeployment();
    bobContractAddress = await bobContract.getAddress();

    const fixtureFactory = await ethers.getContractFactory('ConnectorHttpFixture');
    fixture = (await fixtureFactory.connect(signers.alice).deploy()) as unknown as Contract;
    await fixture.waitForDeployment();

    publicKey = (await instances.alice.generateKeypair()).publicKey;
  });

  const userRequest = (
    handle: string,
    overrides: Partial<Parameters<typeof buildUserRequest>[0]> = {},
    signer = signers.alice,
    mode?: Parameters<typeof buildUserRequest>[2],
  ) =>
    buildUserRequest(
      {
        handles: [{ handle, contractAddress: aliceContractAddress, ownerAddress: signers.alice.address }],
        userAddress: signers.alice.address,
        publicKey,
        allowedContracts: [aliceContractAddress],
        decryptionContractAddress: verifyingContractAddressDecryption,
        ...overrides,
      },
      signer,
      mode,
    );

  describe('negative-acl', function () {
    it('test connector endpoint negative public decrypt of a non-publicly-decryptable handle is acl_denied', async function () {
      this.timeout(ASYNC_TIMEOUT_MS);
      const body = buildPublicRequest([(await fixture.getFunction('xPrivate')()) as string]);
      const out = await fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, body);
      expectErrorOnEveryParty(out.results, { status: 403, code: 'acl_denied' });

      // `acl_denied` is retryable: a re-submission re-arms the row and the worker re-evaluates.
      const again = await fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, body);
      expectErrorOnEveryParty(again.results, { status: 403, code: 'acl_denied' });
    });

    it('test connector endpoint negative user decrypt by a user the ACL does not allow is acl_denied', async function () {
      this.timeout(ASYNC_TIMEOUT_MS);
      const handle = await aliceContract.xUint8();
      const body = await userRequest(
        handle,
        {
          handles: [{ handle, contractAddress: aliceContractAddress, ownerAddress: signers.bob.address }],
          userAddress: signers.bob.address,
        },
        signers.bob,
      );
      const out = await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, body);
      expectErrorOnEveryParty(out.results, { status: 403, code: 'acl_denied' });
    });

    it('test connector endpoint negative user decrypt with an allowedContracts list excluding the handle contract is acl_denied', async function () {
      this.timeout(ASYNC_TIMEOUT_MS);
      const body = await userRequest(await aliceContract.xUint8(), { allowedContracts: [bobContractAddress] });
      const out = await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, body);
      expectErrorOnEveryParty(out.results, { status: 403, code: 'acl_denied' });
    });
  });

  describe('worker signature checks', function () {
    it('test connector endpoint negative user decrypt signed by someone else than userAddress is user_signature_rejected', async function () {
      this.timeout(ASYNC_TIMEOUT_MS);
      // bob signs a permit that claims alice's identity (an EOA, so no ERC-1271 fallback can save it).
      const body = await userRequest(await aliceContract.xUint8(), {}, signers.alice, {
        kind: 'erc1271',
        ownerSigner: signers.bob,
      });
      const out = await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, body);
      expectErrorOnEveryParty(out.results, { status: 403, code: 'user_signature_rejected' });
    });

    it('test connector endpoint negative user decrypt with an expired validity window is user_signature_rejected', async function () {
      this.timeout(ASYNC_TIMEOUT_MS);
      const body = await userRequest(await aliceContract.xUint8(), {
        startTimestamp: Math.floor(Date.now() / 1000) - 3600,
        durationSeconds: 60,
      });
      const out = await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, body);
      expectErrorOnEveryParty(out.results, { status: 403, code: 'user_signature_rejected' });
    });

    it('test connector endpoint negative user decrypt with userAddress inside allowedContracts is unprocessable', async function () {
      this.timeout(ASYNC_TIMEOUT_MS);
      const body = await userRequest(await aliceContract.xUint8(), {
        allowedContracts: [aliceContractAddress, signers.alice.address],
      });
      const out = await fanOutUntilSettled<UserDecryptionResponse>(USER_DECRYPT_ROUTE, body);
      expectErrorOnEveryParty(out.results, { status: 422, code: 'unprocessable' });

      // Non-retryable: the stored error row is served as is on re-submission.
      const again = await fanOut<UserDecryptionResponse>(USER_DECRYPT_ROUTE, body);
      expectErrorOnEveryParty(again.results, { status: 422, code: 'unprocessable' });
    });
  });

  describe('re-arm', function () {
    it('test connector endpoint negative a retryable acl_denied becomes 200 once the ACL allows the handle', async function () {
      this.timeout(ASYNC_TIMEOUT_MS);
      const handles = [(await fixture.getFunction('xLater')()) as string];
      const body = buildPublicRequest(handles);

      const denied = await fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, body);
      expectErrorOnEveryParty(denied.results, { status: 403, code: 'acl_denied' });
      const deniedId = (denied.results[0].body as { decryptionId: string }).decryptionId;

      const tx = await fixture.connect(signers.alice).getFunction('makeLaterPubliclyDecryptable')();
      await tx.wait();
      await waitBlocks(PROPAGATION_BLOCKS);

      // Same body, same id: the endpoint re-arms the failed row and the worker now succeeds.
      const allowed = await fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, body);
      const verified = verifyPublicDecryptQuorum(handles, allowed.results, kms, { minSigners: quorum() });
      expect(verified.decryptionId).to.equal(deniedId);
      expect(verified.clearValues).to.deep.equal([1337n]);
    });
  });
});
