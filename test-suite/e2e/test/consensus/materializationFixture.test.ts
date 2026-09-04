import { expect } from 'chai';
import type { Signer } from 'ethers';

import type { SdkInstance } from '../sdk/types';
import { decryptMaterializationFixture } from './materializationFixtureDecrypt';
import {
  FIXTURE_EXPECTED_PLAINTEXTS,
  FIXTURE_HANDLE_LABELS,
  type FixtureHandleLabel,
  type FixtureHandles,
} from './materializationFixtureModel';

function fixtureHandles(): FixtureHandles {
  return Object.fromEntries(
    FIXTURE_HANDLE_LABELS.map((label, index) => [label, `0x${index.toString(16).padStart(64, '0')}`]),
  ) as FixtureHandles;
}

describe('Materialization fixture plaintext oracle', () => {
  it('serializes every user-decryption request with terminal first', async () => {
    const handles = fixtureHandles();
    const labelByHandle = new Map(FIXTURE_HANDLE_LABELS.map((label) => [handles[label], label] as const));
    const calls: FixtureHandleLabel[] = [];
    let activeRequests = 0;
    let peakActiveRequests = 0;
    const instance = {
      async userDecryptSingleHandle({ handle }: { readonly handle: string }) {
        const label = labelByHandle.get(handle);
        if (!label) throw new Error(`unexpected fixture handle ${handle}`);
        calls.push(label);
        activeRequests += 1;
        peakActiveRequests = Math.max(peakActiveRequests, activeRequests);
        // Yield once so a Promise.all regression would start every remaining
        // request before any of them completes.
        await Promise.resolve();
        activeRequests -= 1;
        return FIXTURE_EXPECTED_PLAINTEXTS[label];
      },
    } as unknown as SdkInstance;
    const owner = { address: `0x${'11'.repeat(20)}` } as Signer & { readonly address: string };

    const plaintexts = await decryptMaterializationFixture(instance, owner, `0x${'22'.repeat(20)}`, handles);

    expect(calls).to.deep.equal(['terminal', ...FIXTURE_HANDLE_LABELS.filter((label) => label !== 'terminal')]);
    expect(peakActiveRequests).to.equal(1);
    expect(plaintexts).to.deep.equal(FIXTURE_EXPECTED_PLAINTEXTS);
  });
});
