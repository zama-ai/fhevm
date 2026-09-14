import { address, getProgramDerivedAddress } from '@solana/kit';
import { describe, expect, it } from 'vitest';
import { getVerifyPublicDecryptInstructionAsync, ZAMA_HOST_PROGRAM_ADDRESS } from './host.js';

describe('generated host instruction defaults', () => {
  it.each([ZAMA_HOST_PROGRAM_ADDRESS, address('11111111111111111111111111111111')])(
    'derives hostConfig under the selected program %s',
    async (programAddress) => {
      const instruction = await getVerifyPublicDecryptInstructionAsync(
        {
          kmsContext: programAddress,
          encryptedStore: programAddress,
          handle: new Uint8Array(32),
          cleartext: new Uint8Array(32),
          signatures: [],
          extraData: new Uint8Array(),
          leafIndex: 0n,
          siblings: [],
        },
        { programAddress },
      );
      const [hostConfig] = await getProgramDerivedAddress({
        programAddress,
        seeds: [new TextEncoder().encode('host-config')],
      });
      expect(instruction.programAddress).toBe(programAddress);
      expect(instruction.accounts[0].address).toBe(hostConfig);
    },
  );
});
