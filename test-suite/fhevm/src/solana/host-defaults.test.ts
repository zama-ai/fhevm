import { expect, test } from 'bun:test';
import { address, createNoopSigner, getProgramDerivedAddress } from '@solana/kit';
import { getInitializeHostConfigInstructionAsync } from './internal/generated/zamaHost/instructions/initializeHostConfig';
import { getDefineKmsContextInstructionAsync } from './internal/generated/zamaHost/instructions/defineKmsContext';

const programAddress = address('11111111111111111111111111111111');
const admin = createNoopSigner(programAddress);
const seed = (value: string) => new TextEncoder().encode(value);

test('generated initialization uses the selected program for hostConfig and randNonce', async () => {
  const instruction = await getInitializeHostConfigInstructionAsync(
    {
      payer: admin,
      admin,
      programData: programAddress,
      eventAuthority: programAddress,
      program: programAddress,
      chainId: 1n,
      gatewayChainId: 1n,
      inputVerificationContract: new Uint8Array(20),
      coprocessorSigners: [new Uint8Array(20)],
      coprocessorThreshold: 1,
      decryptionContract: new Uint8Array(20),
      grantDenyListEnabled: false,
    },
    { programAddress },
  );
  const [config] = await getProgramDerivedAddress({ programAddress, seeds: [seed('host-config')] });
  const [nonce] = await getProgramDerivedAddress({ programAddress, seeds: [seed('rand-nonce')] });
  expect(instruction.accounts[3].address).toBe(config);
  expect(instruction.accounts[4].address).toBe(nonce);
});

test('generated KMS context preserves argument seeds and selected program', async () => {
  for (const fill of [1, 2]) {
    const contextId = new Uint8Array(32).fill(fill);
    const instruction = await getDefineKmsContextInstructionAsync(
      {
        admin,
        eventAuthority: programAddress,
        program: programAddress,
        contextId,
        signers: [new Uint8Array(20)],
        thresholds: { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 },
      },
      { programAddress },
    );
    const [context] = await getProgramDerivedAddress({ programAddress, seeds: [seed('kms-context'), contextId] });
    expect(instruction.accounts[2].address).toBe(context);
  }
});
