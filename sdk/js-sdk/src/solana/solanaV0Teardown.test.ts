// The v0 Solana user-decrypt surface is gone from the SDK, provably.
//
// The SDK used to build a bespoke binary preimage, sign it as a raw message, and post it under an
// attestation type the relayer no longer accepts. All of that is replaced by the sRFC-38 permit: the
// canonical text, the offchain-message envelope, and the host-generic v3 envelope.
//
// A surviving reference keeps the dead surface alive silently — worse here than in most places,
// because a v0 code path that still compiles is a second way to sign, and the exclusivity of the
// signing channel is the property that makes a permit signature unusable as anything else. So this gate
// scans the SDK sources for the symbols that named v0 and fails while any of them exists.
//
// It is red until the code stage deletes them. That is deliberate: the gate is the definition of done
// for the removal, written before the removal.

import { readFileSync, readdirSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

/** The symbols that named the v0 Solana user-decrypt surface, each with the reason it must die. */
const FORBIDDEN: ReadonlyArray<readonly [string, string]> = [
  ['solanaNonce', 'the v0 per-request nonce; replay is bounded by the permit validity window'],
  ['solanaUserIdentity', 'a v0 typed Solana field on the EVM payload; the permit carries the identity'],
  ['solanaAllowedAclDomainKeys', 'a v0 typed Solana field; the ACL scope rides inside the permit'],
  ['solanaUserDecryptSigningMessage', 'the v0 binary preimage renderer; the canonical text replaced it'],
  ['buildSolanaUserDecryptMmrProofExtraData', 'the v0 extraData carrier for user decrypt; routing is 0x02 now'],
  ['solana-ed25519-user-decrypt', 'the retired v0 attestation type, in any of its versions'],
  ['deSigncryptSolanaUserDecrypt', 'the v0 de-signcryption entry point; response verification replaced it'],
  ['68ba21ba', 'the retired vendored TKMS blob; the host-generic path uses the newer one'],
  ['signing_message_v1.json', 'the v0 signing-message fixture; the permit and envelope sets replaced it'],
];

/** Files that may mention the symbols because their job is to name them. */
const EXEMPT = ['solanaV0Teardown.test.ts'];

const SRC = new URL('../', import.meta.url);

function typescriptSources(directory: URL): readonly URL[] {
  const found: URL[] = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.name === 'wasm' || entry.name === '_cjs' || entry.name === '_esm' || entry.name === '_types') {
      continue;
    }
    const child = new URL(entry.isDirectory() ? `${entry.name}/` : entry.name, directory);
    if (entry.isDirectory()) {
      found.push(...typescriptSources(child));
    } else if (entry.name.endsWith('.ts') && !EXEMPT.includes(entry.name)) {
      found.push(child);
    }
  }
  return found;
}

describe('the v0 Solana user-decrypt surface', () => {
  it('is gone from the SDK sources', () => {
    const sources = typescriptSources(SRC);
    expect(sources.length, 'the gate found suspiciously few sources — did the layout move?').toBeGreaterThan(20);

    const survivors: string[] = [];
    for (const source of sources) {
      const text = readFileSync(source, 'utf8');
      for (const [token, reason] of FORBIDDEN) {
        text.split('\n').forEach((line, index) => {
          if (line.includes(token)) {
            survivors.push(`${source.pathname.split('/js-sdk/')[1] ?? source.pathname}:${index + 1} — ${reason}`);
          }
        });
      }
    }

    expect(survivors, `the v0 Solana user-decrypt surface is still alive:\n${survivors.join('\n')}`).toEqual([]);
  });
});
