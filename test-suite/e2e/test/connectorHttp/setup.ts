// Shared setup for the kms-connector endpoint suites.
import { expect } from 'chai';
import { ethers } from 'hardhat';

import { connectorHttpConfigured, endpointUrls, getVersion } from '../sdk/connector/connectorHttp';
import { type KmsSignerSet, readKmsSignerSet } from '../sdk/connector/verify';

/** Skips the whole suite when the stack has no kms-connector endpoint (older bundles). */
export function skipUnlessConfigured(ctx: Mocha.Context): void {
  if (!connectorHttpConfigured()) {
    console.log('[connector-http] KMS_CONNECTOR_ENDPOINT_URLS is empty; skipping suite');
    ctx.skip();
  }
}

/** Fails fast, listing every party whose `/v1/version` is unreachable. */
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
