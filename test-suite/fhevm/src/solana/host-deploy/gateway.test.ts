import { afterEach, describe, expect, test } from 'bun:test';

import { evmAddressBytes, readGatewayBootstrapInputsFromEnv } from './gateway';

const ADDRESS_A = '0x000000000000000000000000000000000000aaaa';
const ADDRESS_B = '0x1111111111111111111111111111111111111111';
const GATEWAY_CONFIG = '0x2222222222222222222222222222222222222222';

const word = (hex: string): string => hex.replace(/^0x/, '').padStart(64, '0');
const addressArrayReturnData = (addresses: readonly string[]): string =>
  `0x${word('0x20')}${word(`0x${addresses.length.toString(16)}`)}${addresses.map(word).join('')}`;

describe('readGatewayBootstrapInputsFromEnv', () => {
  const originalFetch = globalThis.fetch;
  const originalEnv = { ...process.env };
  afterEach(() => {
    globalThis.fetch = originalFetch;
    for (const name of [
      'GATEWAY_RPC_URL',
      'GATEWAY_CONFIG_ADDRESS',
      'INPUT_VERIFICATION_ADDRESS',
      'DECRYPTION_ADDRESS',
    ] as const) {
      if (originalEnv[name] === undefined) delete process.env[name];
      else process.env[name] = originalEnv[name];
    }
  });

  test('reads contract addresses from env and signers live from the gateway RPC', async () => {
    process.env.GATEWAY_RPC_URL = 'http://127.0.0.1:8546';
    process.env.GATEWAY_CONFIG_ADDRESS = GATEWAY_CONFIG;
    process.env.INPUT_VERIFICATION_ADDRESS = ADDRESS_A;
    process.env.DECRYPTION_ADDRESS = ADDRESS_B;
    globalThis.fetch = (async (_url: string | URL | Request, options?: RequestInit) => {
      const request = JSON.parse(String(options?.body)) as {
        id: number;
        method: string;
        params: [{ data?: string }?];
      };
      const result =
        request.method === 'eth_chainId'
          ? '0xd903'
          : addressArrayReturnData(request.params?.[0]?.data === '0x9164d0ae' ? [ADDRESS_A] : [ADDRESS_A, ADDRESS_B]);
      return new Response(JSON.stringify({ jsonrpc: '2.0', id: request.id, result }));
    }) as typeof fetch;

    const inputs = await readGatewayBootstrapInputsFromEnv();
    expect(inputs.gatewayChainId).toBe(55555n);
    expect(Buffer.from(inputs.inputVerificationContract).toString('hex')).toBe(ADDRESS_A.slice(2));
    expect(inputs.coprocessorSigners).toHaveLength(1);
    expect(inputs.kmsSigners).toHaveLength(2);
  });

  test('fails when a required env is missing', async () => {
    delete process.env.GATEWAY_RPC_URL;
    delete process.env.GATEWAY_CONFIG_ADDRESS;
    delete process.env.INPUT_VERIFICATION_ADDRESS;
    delete process.env.DECRYPTION_ADDRESS;
    await expect(readGatewayBootstrapInputsFromEnv()).rejects.toThrow('missing required env GATEWAY_RPC_URL');
  });
});

describe('evmAddressBytes', () => {
  test('is re-exported for the host-deploy CLI', () => {
    expect(evmAddressBytes(ADDRESS_A)).toHaveLength(20);
  });
});
