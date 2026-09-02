// The RPC evidence source: one account read decides current-versus-historical, and stays one.
//
// What is pinned: the id the caller hands in is resolved to the account it names — the PDA under
// the host program, read at `confirmed` — and both travel in the evidence; a handle that is the
// account's current one resolves with no proof and no peaks; a handle an update has replaced
// resolves with the service's proof and — load-bearing — the peaks and leaf count of the SAME
// account read that showed it replaced, so the builder verifies the proof against the snapshot it
// belongs to. A service answering for a different leaf count than that read is an incoherent
// snapshot and fails the resolution rather than assembling evidence that contradicts itself.

import type { SolanaRpc } from '../encryptedValueAccount.js';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { getAddressDecoder, getAddressEncoder } from '@solana/kit';
import { solanaEncryptedValueAccountAddress } from '../encryptedValueAccount.js';
import { createSolanaRpcAccessEvidenceSource } from './index.js';

////////////////////////////////////////////////////////////////////////////////
// An account, and the service beside it
////////////////////////////////////////////////////////////////////////////////

const bytes32 = (fill: number): Uint8Array => new Uint8Array(32).fill(fill);
const HOST_PROGRAM_ID = bytes32(0x22);
const ENCRYPTED_VALUE_ID = bytes32(0xe1);
const CURRENT_HANDLE = bytes32(0x44);
const REPLACED_HANDLE = bytes32(0x45);
const SUBJECT = bytes32(0xc1);

// The one address the id names under the host program; the source must read exactly this account.
const ACCOUNT_ADDRESS = await solanaEncryptedValueAccountAddress(HOST_PROGRAM_ID, ENCRYPTED_VALUE_ID);
const ACCOUNT_BYTES = new Uint8Array(getAddressEncoder().encode(ACCOUNT_ADDRESS));

const u32LE = (value: number): Uint8Array => {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setUint32(0, value, true);
  return out;
};
const u64LE = (value: bigint): Uint8Array => {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, true);
  return out;
};
const concat = (...parts: readonly Uint8Array[]): Uint8Array => {
  const out = new Uint8Array(parts.reduce((length, part) => length + part.length, 0));
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
};

/** The account body: current handle 0x44, leaf count 3, its two peaks. */
const accountData = (): Uint8Array =>
  concat(
    // `sha256("account:EncryptedValue")[..8]` — matched by the decoder before anything else.
    new Uint8Array([0x9b, 0x03, 0x95, 0x3a, 0x84, 0x67, 0xc8, 0xa1]),
    bytes32(0x11),
    bytes32(0x22),
    bytes32(0x33),
    CURRENT_HANDLE,
    u32LE(1),
    SUBJECT,
    u64LE(3n),
    u32LE(2),
    bytes32(0x71),
    bytes32(0x72),
    new Uint8Array([0xfe]),
  );

/** An RPC whose one account is the fixture, recording the address and config of each read. */
function fixtureRpc() {
  const reads: { address: string; config: unknown }[] = [];
  const rpc = {
    getAccountInfo: (address: string, config: unknown) => ({
      send: () => {
        reads.push({ address, config });
        return Promise.resolve({
          context: { slot: 100n },
          value: {
            data: [Buffer.from(accountData()).toString('base64'), 'base64'],
            executable: false,
            lamports: 1n,
            owner: getAddressDecoder().decode(HOST_PROGRAM_ID),
            rentEpoch: 0n,
            space: BigInt(accountData().length),
          },
        });
      },
    }),
  } as unknown as SolanaRpc;
  return { rpc, reads };
}

/** A proof service answering with a verified proof built against the given leaf count. */
function proofServiceAnswering(leafCount: number) {
  const urls: string[] = [];
  vi.stubGlobal(
    'fetch',
    vi.fn((input: string | URL | Request) => {
      urls.push(String(input));
      return Promise.resolve(
        new Response(
          JSON.stringify({
            mmr_proof: { leaf_index: 1, siblings: ['11'.repeat(32), '22'.repeat(32)] },
            leaf_count: leafCount,
            verified: true,
            status: 'verified',
          }),
          { status: 200 },
        ),
      );
    }),
  );
  return { urls };
}

function sourceOver(rpc: SolanaRpc) {
  return createSolanaRpcAccessEvidenceSource({
    rpc,
    proofService: { proofServiceUrl: 'http://proofs.local' },
    hostProgramId: HOST_PROGRAM_ID,
  });
}

afterEach(() => {
  vi.unstubAllGlobals();
});

////////////////////////////////////////////////////////////////////////////////

describe('the RPC evidence source', () => {
  it('resolves the account-current handle with no proof, no peaks, and a zero leaf count', async () => {
    const { rpc, reads } = fixtureRpc();
    proofServiceAnswering(3);

    const evidence = await sourceOver(rpc).resolve({
      handle: CURRENT_HANDLE,
      subject: SUBJECT,
      encryptedValueId: ENCRYPTED_VALUE_ID,
    });

    expect(evidence).toEqual({
      handle: CURRENT_HANDLE,
      subject: SUBJECT,
      encryptedValueId: ENCRYPTED_VALUE_ID,
      encryptedValueAccount: ACCOUNT_BYTES,
      proofLeafCount: 0n,
      accessProof: new Uint8Array(0),
      peaks: [],
    });
    // The read goes to the PDA the id names — never to the id itself — and observes the chain at
    // confirmed; finalized would lag the very update whose evidence is being resolved.
    expect(reads[0]?.address).toBe(ACCOUNT_ADDRESS);
    expect(reads[0]?.config).toMatchObject({ commitment: 'confirmed' });
  });

  it('resolves a replaced handle with the proof, under the peaks of the same account read', async () => {
    const { rpc } = fixtureRpc();
    const { urls } = proofServiceAnswering(3);

    const evidence = await sourceOver(rpc).resolve({
      handle: REPLACED_HANDLE,
      subject: SUBJECT,
      encryptedValueId: ENCRYPTED_VALUE_ID,
    });

    // The service is asked by the account — the PDA — not by the wire identity.
    expect(urls[0]).toContain(`encrypted_value=${ACCOUNT_ADDRESS}`);
    expect(urls[0]).toContain(`handle=${'45'.repeat(32)}`);
    expect(evidence.encryptedValueAccount).toEqual(ACCOUNT_BYTES);
    expect(evidence.proofLeafCount).toBe(3n);
    expect(evidence.peaks).toEqual([bytes32(0x71), bytes32(0x72)]);
    expect(evidence.accessProof.length).toBeGreaterThan(0);
  });

  // The proof and the peaks must describe one MMR. A service answer built against another leaf
  // count belongs to another snapshot, and evidence assembled across two snapshots contradicts
  // itself in a way only the fee would discover.
  it('fails the resolution when the service and the account disagree about the leaf count', async () => {
    const { rpc } = fixtureRpc();
    proofServiceAnswering(4);

    await expect(
      sourceOver(rpc).resolve({ handle: REPLACED_HANDLE, subject: SUBJECT, encryptedValueId: ENCRYPTED_VALUE_ID }),
    ).rejects.toThrow('leaf count');
  });
});
