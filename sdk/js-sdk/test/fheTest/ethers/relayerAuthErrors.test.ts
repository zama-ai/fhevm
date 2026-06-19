import type { Auth } from '../../../src/core/types/auth.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { getEthersTestConfig, type FheTestEthersConfig } from '../setup-ethers.js';
import { fetchFheEncryptionKeySource } from '../../../src/core/modules/relayer/module/fetchFheEncryptionKeySource.js';

////////////////////////////////////////////////////////////////////////////////
//
// End-to-end coverage for relayer/edge auth-error surfacing (401 / 403).
//
// When the relayer (Kong/origin) returns a 401, or an edge proxy (Cloudflare)
// blocks the request with a 403, the SDK must surface the message the server
// actually sent instead of a generic "HTTP error! status: <code>".
//
// These hit the real hosted relayer `v2/keyurl` endpoint with deliberately
// invalid credentials, so they only run against Zama-hosted endpoints
// (*.zama.org) that enforce auth at the edge. Local/self-hosted relayers that
// don't enforce auth are skipped.
//
// CHAIN=testnet npx vitest run --config test/fheTest/vitest.config.ts ethers/relayerAuthErrors.test.ts
// CHAIN=mainnet npx vitest run --config test/fheTest/vitest.config.ts ethers/relayerAuthErrors.test.ts
//
////////////////////////////////////////////////////////////////////////////////

function isZamaHostedRelayer(relayerUrl: string): boolean {
  try {
    return new URL(relayerUrl).hostname.endsWith('.zama.org');
  } catch {
    return false;
  }
}

const config: FheTestEthersConfig = getEthersTestConfig();
const relayerUrl: string = config.fhevmChain.fhevm.relayerUrl;

describe.runIf(isZamaHostedRelayer(relayerUrl))('Relayer auth-error surfacing (keyurl)', () => {
  let relayerClient: { relayerUrl: string; chainId: number };

  beforeAll(() => {
    relayerClient = { relayerUrl, chainId: config.fhevmChain.id };
  });

  // Helper: call the keyurl fetch with the given (invalid) auth and return the
  // thrown error. Fails the test if the call unexpectedly succeeds.
  async function fetchKeyUrlExpectingError(auth?: Auth): Promise<{ name: string; message: string }> {
    let caught: { name?: string; message?: string } | undefined;
    try {
      await fetchFheEncryptionKeySource(relayerClient, {
        options: auth !== undefined ? { auth } : {},
      });
    } catch (e) {
      caught = e as { name?: string; message?: string };
    }
    expect(caught, 'Expected fetchFheEncryptionKeySource to reject with an auth error').toBeDefined();
    return { name: caught?.name ?? '', message: caught?.message ?? '' };
  }

  it('surfaces the origin 401 message for an invalid x-api-key', async () => {
    // An x-api-key header reaches the origin (Kong/relayer), which replies 401
    // with a JSON `{ message, label }` body.
    const { name, message } = await fetchKeyUrlExpectingError({
      type: 'ApiKeyHeader',
      value: 'invalid-zama-api-key-for-testing',
    });

    console.log(`  401 surfaced message: ${message}`);
    expect(name).toBe('RelayerFetchError');
    expect(message).toMatch(/HTTP error! status: 40[13]/);
    // The fix appends the server message after the URL — proving the body was
    // surfaced rather than discarded behind the bare "HTTP error! status: X".
    expect(message).toMatch(/on https:\/\/\S+: \S/);
  });

  it('surfaces the edge 403 message when no x-api-key reaches the origin', async () => {
    // A Bearer token (or no key) never reaches the origin: Cloudflare blocks it
    // at the edge with a 403 and a JSON `{ message, label }` body.
    const { name, message } = await fetchKeyUrlExpectingError({
      type: 'BearerToken',
      token: 'invalid-bearer-token-for-testing',
    });

    console.log(`  403 surfaced message: ${message}`);
    expect(name).toBe('RelayerFetchError');
    expect(message).toMatch(/HTTP error! status: 40[13]/);
    expect(message).toMatch(/on https:\/\/\S+: \S/);
  });
});
