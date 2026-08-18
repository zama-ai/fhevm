// Resolving per-handle evidence, and the three things a resolver must not quietly do.
//
// It must not reorder, it must not drop an entry, and it must not let two occurrences of one handle
// disagree about that handle's access. The first two produce a request that is well formed, inside
// every limit, and not the request the caller made — and the only layer that would notice is the
// response linker, at which point the user has paid for an answer they cannot use. The third is why
// duplicates share one fetch: two lookups at different moments could straddle an update and answer
// current for one occurrence and historical for the other.
//
// Resolution is also concurrent — every unique fetch is in flight before the first is awaited — but
// failures still report deterministically: the first entry that cannot be resolved is the one
// named, however late its failure lands.

import type { SolanaAccessEvidence, SolanaAccessEvidenceSource, SolanaHandleRequest } from './index.js';
import { describe, expect, it, vi } from 'vitest';
import { resolveSolanaAccessEvidence } from './index.js';

const handle = (fill: number): Uint8Array => new Uint8Array(32).fill(fill);
const SUBJECT = handle(0xc1);

/** A source that answers from the handle it is given, recording what it was asked. */
function recordingSource(answer?: (request: SolanaHandleRequest) => SolanaAccessEvidence) {
  const resolve = vi.fn((request: SolanaHandleRequest) =>
    Promise.resolve(
      answer?.(request) ?? {
        handle: request.handle,
        subject: request.subject,
        encryptedValueId: new Uint8Array(32).fill(request.handle[0] ?? 0),
        encryptedValueAccount: new Uint8Array(32).fill(0xea),
        proofLeafCount: 0n,
        accessProof: new Uint8Array(0),
        peaks: [],
      },
    ),
  );
  return { resolve, source: { resolve } satisfies SolanaAccessEvidenceSource };
}

const requestsFor = (fills: readonly number[]): readonly SolanaHandleRequest[] =>
  fills.map((fill) => ({ handle: handle(fill), subject: SUBJECT }));

////////////////////////////////////////////////////////////////////////////////

describe('resolving evidence', () => {
  it('asks for every handle once, in the order given', async () => {
    const { resolve, source } = recordingSource();

    const resolved = await resolveSolanaAccessEvidence(source, requestsFor([0x01, 0x02, 0x03]));

    expect(resolve).toHaveBeenCalledTimes(3);
    expect(resolve.mock.calls.map((call) => call[0].handle[0])).toEqual([0x01, 0x02, 0x03]);
    expect(resolved.map((entry) => entry.handle[0])).toEqual([0x01, 0x02, 0x03]);
  });

  // A duplicate is two entries, not one entry named twice: each occupies a position the linker binds.
  // But it is one fetch — two lookups at different moments could straddle an update and give the two
  // occurrences contradictory evidence about one handle.
  it('fetches a repeated (handle, subject) once, keeping one entry per occurrence', async () => {
    const { resolve, source } = recordingSource();

    const resolved = await resolveSolanaAccessEvidence(source, requestsFor([0x01, 0x02, 0x01]));

    expect(resolve).toHaveBeenCalledTimes(2);
    expect(resolved).toHaveLength(3);
    expect(resolved.map((entry) => entry.handle[0])).toEqual([0x01, 0x02, 0x01]);
  });

  it('gives duplicate occurrences one answer, even when a second lookup would disagree', async () => {
    let lookups = 0;
    const { source } = recordingSource((request) => {
      lookups += 1;
      return {
        handle: request.handle,
        subject: request.subject,
        encryptedValueId: new Uint8Array(32).fill(lookups),
        encryptedValueAccount: new Uint8Array(32).fill(0xea),
        proofLeafCount: 0n,
        accessProof: new Uint8Array(0),
        peaks: [],
      };
    });

    const resolved = await resolveSolanaAccessEvidence(source, requestsFor([0x01, 0x01]));

    expect(resolved[0]).toBe(resolved[1]);
  });

  // The same handle under two subjects is two different questions — a direct entry and a delegated
  // one ask about different pubkeys' access — so sharing a fetch would answer the wrong one.
  it('does not share a fetch between the same handle under different subjects', async () => {
    const delegator = handle(0xd0);
    const { resolve, source } = recordingSource();

    const resolved = await resolveSolanaAccessEvidence(source, [
      { handle: handle(0x01), subject: SUBJECT },
      { handle: handle(0x01), subject: delegator },
    ]);

    expect(resolve).toHaveBeenCalledTimes(2);
    expect(resolved.map((entry) => entry.subject)).toEqual([SUBJECT, delegator]);
  });

  // Up to thirty-three handles, each a host read or a proof-service call: one at a time would make
  // the slowest chain the sum of every latency. Every unique fetch is in flight before any answer.
  it('has every unique fetch in flight before the first answer arrives', async () => {
    const gates = new Map<number, (evidence: SolanaAccessEvidence) => void>();
    const resolve = vi.fn(
      (request: SolanaHandleRequest) =>
        new Promise<SolanaAccessEvidence>((release) => {
          gates.set(request.handle[0] ?? 0, release);
        }),
    );

    const pending = resolveSolanaAccessEvidence({ resolve }, requestsFor([0x01, 0x02, 0x03]));
    expect(resolve).toHaveBeenCalledTimes(3);

    for (const [fill, release] of gates) {
      release({
        handle: handle(fill),
        subject: SUBJECT,
        encryptedValueId: handle(0xe1),
        encryptedValueAccount: handle(0xea),
        proofLeafCount: 0n,
        accessProof: new Uint8Array(0),
        peaks: [],
      });
    }
    const resolved = await pending;
    expect(resolved.map((entry) => entry.handle[0])).toEqual([0x01, 0x02, 0x03]);
  });

  // Deterministic reporting under concurrency: the first entry that cannot be resolved is the one
  // reported, not whichever of several in-flight failures happened to land first.
  it('reports the first unresolvable entry, not the first failure to land', async () => {
    const early = new Error('the first entry, failing late');
    const late = new Error('the second entry, failing first');
    const resolve = vi.fn((request: SolanaHandleRequest) =>
      request.handle[0] === 0x01
        ? Promise.resolve().then((): Promise<SolanaAccessEvidence> => Promise.reject(early))
        : Promise.reject<SolanaAccessEvidence>(late),
    );

    await expect(resolveSolanaAccessEvidence({ resolve }, requestsFor([0x01, 0x02]))).rejects.toBe(early);
  });

  it('passes each handle its own subject through untouched', async () => {
    const delegator = handle(0xd0);
    const { source } = recordingSource();

    const resolved = await resolveSolanaAccessEvidence(source, [
      { handle: handle(0x01), subject: SUBJECT },
      { handle: handle(0x02), subject: delegator },
    ]);

    expect(resolved.map((entry) => entry.subject)).toEqual([SUBJECT, delegator]);
  });

  it('carries both access modes as the source reports them', async () => {
    const proof = new Uint8Array([0x03, 0x00]);
    const { source } = recordingSource((request) => ({
      handle: request.handle,
      subject: request.subject,
      encryptedValueId: handle(0xe1),
      encryptedValueAccount: handle(0xea),
      proofLeafCount: request.handle[0] === 0x02 ? 8n : 0n,
      accessProof: request.handle[0] === 0x02 ? proof : new Uint8Array(0),
      peaks: [],
    }));

    const resolved = await resolveSolanaAccessEvidence(source, requestsFor([0x01, 0x02]));

    expect(resolved[0]?.proofLeafCount).toBe(0n);
    expect(resolved[0]?.accessProof).toHaveLength(0);
    expect(resolved[1]?.proofLeafCount).toBe(8n);
    expect(resolved[1]?.accessProof).toEqual(proof);
  });

  it('resolves nothing for an empty list, without asking the source', async () => {
    const { resolve, source } = recordingSource();

    await expect(resolveSolanaAccessEvidence(source, [])).resolves.toEqual([]);
    expect(resolve).not.toHaveBeenCalled();
  });

  // One entry that cannot be resolved fails the whole request. Assembling from what did resolve would
  // ask for less than the caller asked for, and nothing downstream can tell that was deliberate.
  it('fails the whole resolution when one handle cannot be resolved', async () => {
    const boom = new Error('the proof service is unavailable');
    const resolve = vi.fn((request: SolanaHandleRequest) =>
      request.handle[0] === 0x02
        ? Promise.reject(boom)
        : Promise.resolve<SolanaAccessEvidence>({
            handle: request.handle,
            subject: request.subject,
            encryptedValueId: handle(0xe1),
            encryptedValueAccount: handle(0xea),
            proofLeafCount: 0n,
            accessProof: new Uint8Array(0),
            peaks: [],
          }),
    );

    await expect(resolveSolanaAccessEvidence({ resolve }, requestsFor([0x01, 0x02, 0x03]))).rejects.toBe(boom);
  });
});
