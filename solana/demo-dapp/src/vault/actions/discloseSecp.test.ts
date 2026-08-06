import { describe, expect, it } from 'vitest';
import { address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import type { MmrProof } from '@sdk-src/solana/proof.js';
import type { SolanaPublicDecryptCertificateClaim } from '@sdk-src/solana/actions/publicDecryptCertificate.js';
import { buildDiscloseSecpInstruction } from './discloseSecp.js';
import { getDiscloseSecpInstructionDataDecoder } from '../internal/generated/confidentialToken/instructions/discloseSecp.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';
import { DisclosedValueKind } from '../internal/generated/confidentialToken/types/disclosedValueKind.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@sdk-src/solana/internal/generated/zamaHost/programAddress.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

const handleBytes = new Uint8Array(32).fill(0xab);
const cleartextBytes = new Uint8Array(32);
cleartextBytes[31] = 0x2a; // 42 as a uint256 low byte
const signatureBytes = new Uint8Array(65).fill(0x11);
const extraDataBytes = new Uint8Array([0x00]);
const sibling = new Uint8Array(32).fill(0x07);
const inclusionProof: MmrProof = { leafIndex: 3n, siblings: [sibling] };

function hex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join('');
}

function claim(overrides: Partial<SolanaPublicDecryptCertificateClaim> = {}): SolanaPublicDecryptCertificateClaim {
  return {
    handle: `0x${hex(handleBytes)}`,
    abiEncodedCleartext: hex(cleartextBytes),
    signatures: [hex(signatureBytes)],
    extraData: `0x${hex(extraDataBytes)}`,
    inclusionProof,
    ...overrides,
  };
}

describe('buildDiscloseSecpInstruction', () => {
  it('maps a claim onto the token disclose_secp instruction with the right accounts', async () => {
    const mint = addr(5);
    const tokenAccount = addr(8);
    const encryptedValue = addr(6);
    const kmsContext = addr(7);
    const hostConfig = addr(9);
    const instruction = await buildDiscloseSecpInstruction(
      { mint, tokenAccount, kind: DisclosedValueKind.BurnedAmount, encryptedValue, kmsContext, hostConfig },
      claim(),
    );

    expect(instruction.programAddress).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    const addresses = instruction.accounts?.map((a: { readonly address: Address }) => a.address) ?? [];
    // mint, tokenAccount, encryptedValue, hostConfig, kmsContext, zamaProgram, eventAuthority, program
    expect(addresses).toHaveLength(8);
    expect(addresses[0]).toBe(mint);
    expect(addresses[1]).toBe(tokenAccount);
    expect(addresses[2]).toBe(encryptedValue);
    expect(addresses[3]).toBe(hostConfig);
    expect(addresses[4]).toBe(kmsContext);
    expect(addresses[5]).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(addresses[7]).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);

    const decoded = getDiscloseSecpInstructionDataDecoder().decode(instruction.data!);
    expect(decoded.kind).toBe(DisclosedValueKind.BurnedAmount);
    expect(Array.from(decoded.handle)).toEqual(Array.from(handleBytes));
    expect(Array.from(decoded.cleartext)).toEqual(Array.from(cleartextBytes));
    expect(decoded.signatures.map((s) => Array.from(s))).toEqual([Array.from(signatureBytes)]);
    expect(Array.from(decoded.extraData)).toEqual(Array.from(extraDataBytes));
    expect(decoded.proof.leafIndex).toBe(3n);
    expect(decoded.proof.siblings.map((s) => Array.from(s))).toEqual([Array.from(sibling)]);
  });
});
