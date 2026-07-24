import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';
import { sha256 } from '@noble/hashes/sha2.js';

import { buildDispatchBatchInstruction } from './dispatchBatch.js';
import {
  DISPATCH_DISCRIMINATOR,
  getDispatchInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/dispatch.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../internal/generated/zamaHost/programAddress.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

// Independent restatement of every seed / label the builder's derivations must reproduce, so the
// expected list below shares no code with the implementation (dispatch.rs is the common reference).
const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);
const pda = async (programAddress: Address, seeds: Uint8Array[]): Promise<Address> =>
  (await getProgramDerivedAddress({ programAddress, seeds }))[0];
const concat = (...parts: Uint8Array[]): Uint8Array => {
  const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
};
// sha256("zama-encrypted-value-key-v1" || aclDomain || appAccount || label), then
// PDA(zamaHost, ["encrypted-value", valueKey]) — zama_solana_acl::derive_value_key.
const valueAccountPda = (aclDomain: Address, appAccount: Address, label: Uint8Array): Promise<Address> =>
  pda(ZAMA_HOST_PROGRAM_ADDRESS, [
    utf8('encrypted-value'),
    sha256(concat(utf8('zama-encrypted-value-key-v1'), base58.decode(aclDomain), base58.decode(appAccount), label)),
  ]);

describe('buildDispatchBatchInstruction', () => {
  // Fixture aligned with derive.test.ts's consensus-critical golden: batcher = addr(2),
  // batch = the golden batch PDA for index 0, mint = addr(13) — so the token-account, balance and
  // total-supply expectations below can be pinned to the same golden base58 strings.
  const payer = signer(addr(1));
  const batcher = addr(2);
  const batch = address('Dm6gzuvv47gSSeMyV72nVs9N79AQA7sczD5GBw3XwXHX');
  const joinConfidentialMint = addr(13);
  const hostConfig = addr(8);

  it('derives every non-root account exactly as dispatch.rs validates them', async () => {
    const instruction = await buildDispatchBatchInstruction({
      payer,
      batcher,
      batch,
      joinConfidentialMint,
      hostConfig,
    });

    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);

    const batchAuthority = await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [
      utf8('batch-authority'),
      base58.decode(batch),
    ]);
    const batchJoinTokenAccount = await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
      utf8('token-account'),
      base58.decode(joinConfidentialMint),
      base58.decode(batchAuthority),
    ]);
    const totalSupplyAuthority = await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
      utf8('total-supply'),
      base58.decode(joinConfidentialMint),
    ]);
    const expected: Address[] = [
      payer.address,
      batcher,
      batch,
      batchAuthority,
      joinConfidentialMint,
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('fhe-compute'), base58.decode(joinConfidentialMint)]),
      totalSupplyAuthority,
      batchJoinTokenAccount,
      await valueAccountPda(joinConfidentialMint, batchJoinTokenAccount, utf8('balance_________________________')),
      await valueAccountPda(joinConfidentialMint, totalSupplyAuthority, utf8('total_supply____________________')),
      await valueAccountPda(joinConfidentialMint, batchJoinTokenAccount, utf8('burned_amount___________________')),
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      ZAMA_HOST_PROGRAM_ADDRESS,
      hostConfig,
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      address('11111111111111111111111111111111'),
    ];
    expect(instruction.accounts!.map((a) => a.address)).toEqual(expected);

    const decoded = getDispatchInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(DISPATCH_DISCRIMINATOR));
  });

  // Golden pins carried over from derive.test.ts (same fixture, byte-verified against the live
  // scenario's hand-built map) and from `solana find-program-derived-address <program>
  // string:__event_authority` for the two event authorities.
  it('matches the golden derived addresses for the fixed fixture', async () => {
    const instruction = await buildDispatchBatchInstruction({
      payer,
      batcher,
      batch,
      joinConfidentialMint,
      hostConfig,
    });
    const addresses = instruction.accounts!.map((a) => a.address);
    expect(addresses[5]).toBe('9Zex4Xc17gawiJNk1pEirBrTx2GsNb5HB6WYgHWWkemQ'); // joinComputeSigner
    expect(addresses[6]).toBe('W4dfnWqZVyik2iMYeP2jHGDfRJbZxzbXfgysxQS1VYK'); // totalSupplyAuthority
    expect(addresses[7]).toBe('8iRxqzbzVoCDyN5ruCrtDs3HEJXL6S5khbmijMta8j6z'); // batchJoinTokenAccount
    expect(addresses[8]).toBe('6L34CwYQLjs4e5sHTjCsoNk5UBZwDtTMkKegf7tRdoM7'); // batchBalanceValue
    expect(addresses[9]).toBe('D1kRDX4FNzfiFqnJCjX443t7ZgN3jCk2NLtNk93eH8pt'); // totalSupplyValue
    expect(addresses[11]).toBe('7usNGbH9WupMAsyDeqdUEoKrjisKcgusGjDiju4vNog'); // zamaEventAuthority
    expect(addresses[14]).toBe('2KQ5N8YEUTk8hQWXBnkGjsvKPzm2rh2nFH6PeoVt7q8U'); // tokenEventAuthority
  });
});
