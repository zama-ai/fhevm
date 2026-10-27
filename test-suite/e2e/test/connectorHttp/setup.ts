// Shared setup for the kms-connector endpoint suites.
import { expect } from 'chai';
import type { Signer } from 'ethers';
import { ethers } from 'hardhat';

import { aclAddress } from '../instance';
import { connectorHttpConfigured, endpointUrls, getVersion } from '../sdk/connector/connectorHttp';
import { type KmsSignerSet, readKmsSignerSet } from '../sdk/connector/verify';

/** Skips the whole suite when the stack has no kms-connector endpoint (older bundles). */
export function skipUnlessConfigured(ctx: Mocha.Context): void {
  if (!connectorHttpConfigured()) {
    console.log('[connector-http] KMS_CONNECTOR_ENDPOINT_URLS is empty; skipping suite');
    ctx.skip();
  }
}

/** Fails fast, listing every party whose `/version` is unreachable. */
export async function probeEndpoints(): Promise<void> {
  const unreachable: string[] = [];
  for (const url of endpointUrls()) {
    try {
      const { httpStatus } = await getVersion(url);
      if (httpStatus !== 200) unreachable.push(`${url} (${httpStatus})`);
    } catch (error) {
      unreachable.push(`${url} (${(error as Error).message})`);
    }
  }
  expect(unreachable, `unreachable connector endpoints: ${unreachable.join(', ')}`).to.deep.equal([]);
}

export async function connectorSetup(ctx: Mocha.Context): Promise<{ urls: string[]; kms: KmsSignerSet }> {
  skipUnlessConfigured(ctx);
  await probeEndpoints();
  return { urls: endpointUrls(), kms: await readKmsSignerSet() };
}

/** Waits for `n` more blocks so the kms-workers' host-chain reads observe a fresh ACL change. */
export async function waitBlocks(n: number): Promise<void> {
  const target = (await ethers.provider.getBlockNumber()) + n;
  for (;;) {
    if ((await ethers.provider.getBlockNumber()) >= target) return;
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
}

const DELEGATION_LIFETIME_SECONDS = 24 * 60 * 60;

/** `delegator` delegates the user decryption of its `contractAddress` handles to `delegate`, for a day. */
export async function delegateForUserDecryption(
  delegator: Signer,
  delegate: string,
  contractAddress: string,
): Promise<void> {
  const acl = new ethers.Contract(
    aclAddress,
    ['function delegateForUserDecryption(address delegate, address contractAddress, uint64 expirationDate)'],
    delegator,
  );
  const latestBlock = await ethers.provider.getBlock('latest');
  const expirationDate = latestBlock!.timestamp + DELEGATION_LIFETIME_SECONDS;
  const tx = await acl.getFunction('delegateForUserDecryption')(delegate, contractAddress, expirationDate);
  await tx.wait();
}
