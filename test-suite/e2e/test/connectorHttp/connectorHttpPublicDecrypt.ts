import { expect } from 'chai';
import { ethers } from 'hardhat';

import { HTTPPublicDecrypt } from '../../types';
import { createInstances } from '../instance';
import { HOST_CHAINS, deployContract, getSigners as getChainSigners } from '../multiChain/multiChainHelper';
import {
  PUBLIC_DECRYPT_ROUTE,
  type PublicDecryptionResponse,
  buildPublicRequest,
  fanOut,
  fanOutUntilSettled,
  getVersion,
  post,
  quorum,
} from '../sdk/connector/connectorHttp';
import { type KmsSignerSet, verifyPublicDecryptQuorum } from '../sdk/connector/verify';
import { type Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';
import { connectorSetup } from './setup';

const POSITIVE_TIMEOUT_MS = 4 * 60 * 1000;

describe('Connector HTTP public decrypt', function () {
  let signers: Signers;
  let instances: FhevmInstances;
  let urls: string[];
  let kms: KmsSignerSet;
  let contract: HTTPPublicDecrypt;

  before(async function () {
    this.timeout(180_000);
    ({ urls, kms } = await connectorSetup(this));
    await initSigners(2);
    signers = await getSigners();
    instances = await createInstances(signers);
    const factory = await ethers.getContractFactory('HTTPPublicDecrypt');
    contract = await factory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
  });

  it('test connector endpoint version route answers on every party', async function () {
    const versions = await Promise.all(urls.map((url) => getVersion(url)));
    for (const [i, v] of versions.entries()) {
      expect(v.httpStatus, urls[i]).to.equal(200);
      expect(v.body, urls[i]).to.deep.equal(versions[0].body);
      expect(Object.keys(v.body).length, `${urls[i]} body ${JSON.stringify(v.body)}`).to.be.greaterThan(0);
    }
  });

  it('test connector endpoint public decrypt ebool', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    const handles = [await contract.xBool()];
    const out = await fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, buildPublicRequest(handles));
    const verified = verifyPublicDecryptQuorum(handles, out.results, kms, { minSigners: quorum() });
    expect(verified.clearValues).to.deep.equal([true]);
  });

  it('test connector endpoint public decrypt mixed types in one request', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    const handles = [await contract.xBool(), await contract.xUint32(), await contract.xAddress()];
    const out = await fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, buildPublicRequest(handles));
    const verified = verifyPublicDecryptQuorum(handles, out.results, kms, { minSigners: quorum() });
    expect(verified.clearValues).to.deep.equal([true, 242n, '0xfC4382C084fCA3f4fB07c3BCDA906C01797595a8']);
  });

  it('test connector endpoint public decrypt same body is served from cache and concurrent duplicates attach', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    const handles = [await contract.xUint32()];
    const body = buildPublicRequest(handles);

    // Two concurrent posts of the same body: the second attaches to the first's request row.
    const [first, second] = await Promise.all([
      fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, body),
      fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, body),
    ]);
    const a = verifyPublicDecryptQuorum(handles, first.results, kms, { minSigners: quorum() });
    const b = verifyPublicDecryptQuorum(handles, second.results, kms, { minSigners: quorum() });
    expect(b.decryptionId).to.equal(a.decryptionId);

    // A later re-submission is answered from the stored response row: same id, same bytes, fast.
    const cached = await fanOut<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, body);
    expect(cached.failures.map((r) => r.url)).to.deep.equal([]);
    for (const [i, r] of cached.results.entries()) {
      const fresh = first.results[i].body as PublicDecryptionResponse;
      expect(r.body).to.deep.equal(fresh);
      expect(r.elapsedMs, `${r.url} took ${r.elapsedMs}ms for a cached answer`).to.be.lessThan(5_000);
    }
  });

  it('test connector endpoint public decrypt agrees with the relayer path', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    const handles = [await contract.xBool(), await contract.xUint32(), await contract.xAddress()];
    const out = await fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, buildPublicRequest(handles));
    const verified = verifyPublicDecryptQuorum(handles, out.results, kms, { minSigners: quorum() });

    const viaRelayer = await instances.alice.publicDecrypt(handles);
    expect(verified.clearValues).to.deep.equal(handles.map((h) => viaRelayer.clearValues[h as `0x${string}`]));
    // Same ABI encoding of the same plaintexts as what the Gateway published.
    expect((out.successes[0].body as PublicDecryptionResponse).decryptedResult.toLowerCase()).to.equal(
      viaRelayer.abiEncodedClearValues.toLowerCase(),
    );
  });

  it('test connector endpoint public decrypt accepts handles from a second host chain', async function () {
    this.timeout(POSITIVE_TIMEOUT_MS);
    if (HOST_CHAINS.length < 2) {
      console.log('[connector-http] single host chain; skipping second-chain check');
      this.skip();
    }
    const chain = HOST_CHAINS[1];
    const deployed = await deployContract('HTTPPublicDecrypt', getChainSigners(chain).alice);
    const handles = [(await deployed.getFunction('xUint32')()) as string];
    const out = await fanOutUntilSettled<PublicDecryptionResponse>(PUBLIC_DECRYPT_ROUTE, buildPublicRequest(handles));
    // The KMS signer set is shared across host chains (same committee); the primary chain's
    // KMSVerifier is the reference.
    const verified = verifyPublicDecryptQuorum(handles, out.results, kms, { minSigners: quorum() });
    expect(verified.clearValues).to.deep.equal([242n]);
    // And the id is content-derived: a single party re-post answers the same id from cache.
    const again = await post<PublicDecryptionResponse>(urls[0], PUBLIC_DECRYPT_ROUTE, buildPublicRequest(handles));
    expect(again.httpStatus).to.equal(200);
    expect((again.body as PublicDecryptionResponse).decryptionId).to.equal(verified.decryptionId);
  });
});
