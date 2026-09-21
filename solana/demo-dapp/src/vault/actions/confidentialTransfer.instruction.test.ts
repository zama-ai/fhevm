import { INSTRUCTIONS_SYSVAR_ADDRESS, prepareTransientStore } from '@fhevm/sdk/solana';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/sdk/solana/host';
import { describe, expect, it } from 'vitest';

import { AccountRole, address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { getConfidentialTransferInstruction, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';

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
    const zamaEvent = key(7);
    const hostConfig = key(8);
    const tokenEvent = key(10);
    const underlyingMint = key(14);
    const fromAta = key(15);
    const toAta = key(15);
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    const instruction = getConfidentialTransferInstruction({
      transientStore: transientStore.address,
      instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
      owner,
      payer,
      mint,
      underlyingMint,
      fromAta,
      toAta,
      fromAccount: aliasedToken,
      toAccount: aliasedToken,
      fromStore: aliasedBalance,
      toStore: aliasedBalance,
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
        contractChainId: 72057594037940281n,
        extraData: new Uint8Array([0]),
        signatures: [new Uint8Array(65).fill(13)],
      },
    });

    expect(instruction.accounts.map(({ address, role }) => [address, role])).toEqual([
      [owner.address, AccountRole.READONLY_SIGNER],
      [payer.address, AccountRole.WRITABLE_SIGNER],
      [mint, AccountRole.READONLY],
      [underlyingMint, AccountRole.READONLY],
      [fromAta, AccountRole.READONLY],
      [toAta, AccountRole.READONLY],
      [aliasedToken, AccountRole.WRITABLE],
      [aliasedToken, AccountRole.WRITABLE],
      [aliasedBalance, AccountRole.WRITABLE],
      [aliasedBalance, AccountRole.WRITABLE],
      [zamaEvent, AccountRole.READONLY],
      [transientStore.address, AccountRole.WRITABLE],
      [INSTRUCTIONS_SYSVAR_ADDRESS, AccountRole.READONLY],
      ['6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu', AccountRole.READONLY],
      [hostConfig, AccountRole.READONLY],
      ['11111111111111111111111111111111', AccountRole.READONLY],
      // HCU witnesses and the optional result State resolve to the program id.
      [CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, AccountRole.READONLY],
      [CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, AccountRole.READONLY],
      [CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, AccountRole.READONLY],
      [tokenEvent, AccountRole.READONLY],
      [CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, AccountRole.READONLY],
    ]);
  });
});
