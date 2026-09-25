import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { TkmsPrivateKey } from '../types/tkms-p.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import { describe, expect, it, vi } from 'vitest';
import { generateTransportKeyPair } from './TransportKeyPair-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { CANONICAL_WASM_VERSIONS } from '../runtime/WasmVersions-p.js';

describe('generateTransportKeyPair', () => {
  it('generates a key pair from the decrypt runtime and the shipped TKMS version', async () => {
    const free = vi.fn();
    const tkmsPrivateKey = { free } as unknown as TkmsPrivateKey;
    const generateTkmsPrivateKey = vi.fn().mockResolvedValue(tkmsPrivateKey);
    const serializeTkmsPrivateKey = vi.fn().mockResolvedValue(new Uint8Array([1, 2, 3]));
    const getTkmsPublicKeyHex = vi.fn().mockResolvedValue('0x010203');
    const runtime = {
      decrypt: {
        generateTkmsPrivateKey,
        serializeTkmsPrivateKey,
        getTkmsPublicKeyHex,
      },
    } as unknown as WithDecrypt;

    const keyPair = await generateTransportKeyPair(
      { runtime, chain: {} as FhevmChain, client: {} },
      { fhevmContext: createFhevmClientFrozenContext({}) },
    );

    expect(keyPair.publicKey).toBe('0x010203');
    expect(keyPair.tkmsVersion).toBe(CANONICAL_WASM_VERSIONS.kms);
    expect(generateTkmsPrivateKey).toHaveBeenCalledWith();
    expect(serializeTkmsPrivateKey).toHaveBeenCalledWith({ tkmsPrivateKey });
    expect(getTkmsPublicKeyHex).toHaveBeenCalledWith({ tkmsPrivateKey });
    expect(free).toHaveBeenCalledOnce();
  });
});
