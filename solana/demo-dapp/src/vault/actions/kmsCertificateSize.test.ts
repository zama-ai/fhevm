import { afterEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));
import { address, generateKeyPairSigner, type Address, type Instruction, type Transaction, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildVerifyPublicDecryptInstruction, type SolanaPublicDecryptCertificateClaim } from '@fhevm/sdk/solana';
import { getRedeemBurnedAmountInstructionAsync, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { buildDiscloseSecpInstruction } from './discloseSecp.js';
import { encodedSize, testDemoClient } from '../../testDemoClient';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

// A certificate at the host's maximum KMS threshold (MAX_KMS_SIGNERS = 16) with the SDK's version 2
// extra data (a version byte, then the 32-byte KMS context and epoch ids).
const claim: SolanaPublicDecryptCertificateClaim = {
  handle: `0x${'ab'.repeat(32)}`,
  abiEncodedCleartext: `${'00'.repeat(31)}2a`,
  signatures: Array.from({ length: 16 }, () => '11'.repeat(65)),
  extraData: `0x02${'09'.repeat(32)}${'0a'.repeat(32)}`,
};
const kmsContext = addr(9);

const consumers: [string, (payer: TransactionSigner) => Promise<Instruction>][] = [
  ['disclose_secp', () => buildDiscloseSecpInstruction({ kmsContext }, claim)],
  [
    'redeem_burned_amount',
    (owner) =>
      getRedeemBurnedAmountInstructionAsync({
        owner,
        mint: addr(1),
        tokenAccount: addr(2),
        underlyingMint: addr(3),
        vaultUsdc: addr(4),
        destinationUsdc: addr(5),
        burnedAmountStore: addr(6),
        kmsContext,
        program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
        burnedHandle: new Uint8Array(32).fill(0xab),
        cleartextAmount: 42n,
        signatures: Array.from({ length: 16 }, () => new Uint8Array(65).fill(0x11)),
        extraData: new Uint8Array([0x02, ...new Uint8Array(32).fill(0x09), ...new Uint8Array(32).fill(0x0a)]),
      }),
  ],
  ['verify_public_decrypt', () => buildVerifyPublicDecryptInstruction({ programAddress: ZAMA_HOST_PROGRAM_ADDRESS, kmsContext }, claim)],
];

describe('KMS certificate consumers', () => {
  afterEach(() => vi.restoreAllMocks());

  // A v1 transaction is at most 4,096 bytes and 64 account keys (solana-message v1::MAX_TRANSACTION_SIZE
  // and MAX_ADDRESSES). The test suite sends these through the demo client.
  it.each(consumers)('%s fits one v1 transaction at the maximum KMS threshold', async (_name, build) => {
    const payer = await generateKeyPairSigner();
    const { client } = testDemoClient(payer);
    await client.sendTransaction([await build(payer)]);
    const size = encodedSize(sendAndConfirm.mock.lastCall![0] as Transaction);
    expect(size.version).toBe(1);
    expect(size.bytes).toBeLessThanOrEqual(4096);
    expect(size.addresses).toBeLessThanOrEqual(64);
  });
});
