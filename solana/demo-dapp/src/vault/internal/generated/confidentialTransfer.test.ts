import { describe, expect, it } from 'vitest';

import { AccountRole, address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { getConfidentialTransferInstructionAsync } from './confidentialToken/instructions/confidentialTransfer.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from './confidentialToken/programAddress.js';

function key(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function signer(value: Address): TransactionSigner {
  return { address: value, signTransactions: async () => [] } as unknown as TransactionSigner;
}

describe('generated confidentialTransfer instruction', () => {
  it('preserves IDL order, signer privileges, omitted optionals, and self-transfer aliases', async () => {
    const owner = signer(key(1));
    const payer = signer(key(2));
    const mint = key(3);
    const aliasedToken = key(4);
    const aliasedBalance = key(5);
    const transferred = key(6);
    const zamaEvent = key(7);
    const hostConfig = key(8);
    const tokenEvent = key(10);
    const instruction = await getConfidentialTransferInstructionAsync({
      owner,
      payer,
      mint,
      fromAccount: aliasedToken,
      toAccount: aliasedToken,
      fromBalanceValue: aliasedBalance,
      toBalanceValue: aliasedBalance,
      transferredAmountValue: transferred,
      zamaEventAuthority: zamaEvent,
      hostConfig,
      eventAuthority: tokenEvent,
      program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      amountAttestation: {
        inputHandle: new Uint8Array(32).fill(11),
        ctHandles: [new Uint8Array(32).fill(11)],
        handleIndex: 0,
        userAddress: base58.decode(owner.address),
        contractAddress: new Uint8Array(32).fill(12),
        contractChainId: (1n << 63n) | 12345n,
        extraData: new Uint8Array([0]),
        signatures: [new Uint8Array(65).fill(13)],
      },
    });

    expect(instruction.accounts.map(({ address, role }) => [address, role])).toEqual([
      [owner.address, AccountRole.READONLY_SIGNER],
      [payer.address, AccountRole.WRITABLE_SIGNER],
      [mint, AccountRole.READONLY],
      [aliasedToken, AccountRole.WRITABLE],
      [aliasedToken, AccountRole.WRITABLE],
      [expect.any(String), AccountRole.READONLY],
      [aliasedBalance, AccountRole.WRITABLE],
      [aliasedBalance, AccountRole.WRITABLE],
      [transferred, AccountRole.WRITABLE],
      [zamaEvent, AccountRole.READONLY],
      ['6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu', AccountRole.READONLY],
      [hostConfig, AccountRole.READONLY],
      ['11111111111111111111111111111111', AccountRole.READONLY],
      [CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, AccountRole.READONLY],
      [CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, AccountRole.READONLY],
      [tokenEvent, AccountRole.READONLY],
      [CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, AccountRole.READONLY],
    ]);
  });
});
