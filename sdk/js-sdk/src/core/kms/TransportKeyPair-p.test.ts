import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { TkmsPrivateKey } from '../types/tkms-p.js';
import { describe, expect, it, vi } from 'vitest';
import { generateTransportKeyPair } from './TransportKeyPair-p.js';

describe('generateTransportKeyPair', () => {
  it('generates a key pair from only the decrypt runtime and TKMS version', async () => {
    const tkmsPrivateKey = {} as TkmsPrivateKey;
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

    const keyPair = await generateTransportKeyPair({ runtime, tkmsVersion: '0.13.20-0' });

    expect(keyPair.publicKey).toBe('0x010203');
    expect(keyPair.tkmsVersion).toBe('0.13.20-0');
    expect(generateTkmsPrivateKey).toHaveBeenCalledWith({ tkmsVersion: '0.13.20-0' });
    expect(serializeTkmsPrivateKey).toHaveBeenCalledWith({ tkmsPrivateKey, tkmsVersion: '0.13.20-0' });
    expect(getTkmsPublicKeyHex).toHaveBeenCalledWith({ tkmsPrivateKey, tkmsVersion: '0.13.20-0' });
  });
});
