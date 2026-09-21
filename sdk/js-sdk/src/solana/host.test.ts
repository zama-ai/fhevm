import { address, getProgramDerivedAddress, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM, SolanaError } from '@solana/kit';
import { describe, expect, it } from 'vitest';
import {
  getVerifyPublicDecryptInstructionAsync,
  isZamaHostError,
  ZAMA_HOST_ERROR__TRANSIENT_STORE_NOT_OPENED,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from './host.js';

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

describe('generated host error helpers', () => {
  const missingJournal = new SolanaError(SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM, {
    code: ZAMA_HOST_ERROR__TRANSIENT_STORE_NOT_OPENED,
    index: 1,
  });

  it('identifies TransientStoreNotOpened when the failed instruction is the host', () => {
    expect(
      isZamaHostError(
        missingJournal,
        { instructions: { 1: { programAddress: ZAMA_HOST_PROGRAM_ADDRESS } } },
        ZAMA_HOST_ERROR__TRANSIENT_STORE_NOT_OPENED,
      ),
    ).toBe(true);
  });

  it('does not treat a custom error from another program as a host error', () => {
    expect(
      isZamaHostError(missingJournal, {
        instructions: { 1: { programAddress: address('pS2gMMq6PNZKpjxiANeoN5XxJgwaFsUR6xaJkpUHcDg') } },
      }),
    ).toBe(false);
  });

  it('does not classify a custom 6000 without a program-address match', () => {
    expect(
      isZamaHostError(new SolanaError(SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM, { code: 6000, index: 0 }), {
        instructions: {},
      }),
    ).toBe(false);
  });
});
