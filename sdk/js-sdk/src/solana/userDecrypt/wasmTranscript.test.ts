// The committed KMS transcripts, through the shipped blob: real signcrypted shares, and the rules
// that refuse them once one bound input of the request changes.
//
// KMS core/service/src/client/solana_response.rs generates these deterministic test keys and
// transcripts in test_user_decryption_solana_and_write_transcript; the regeneration script vendors
// them beside the blob they were produced for. Their handles are opaque KMS test inputs, not valid
// SDK handles: these tests pin the shipped WASM's plaintext recovery and its refusals, while SDK
// handle/type validation is exercised separately.

import { readFileSync } from 'node:fs';
import { beforeAll, describe, expect, it } from 'vitest';
import init, {
  new_server_id_addr,
  new_solana_client,
  process_user_decryption_resp_solana_from_js,
  u8vec_to_ml_kem_pke_pk,
  u8vec_to_ml_kem_pke_sk,
} from '../../wasm/tkms/kms_lib.v0.15.0-0-solana.294802eb.js';
import { tkmsWasmBase64 } from '../../wasm/tkms/kms_lib_bg.v0.15.0-0-solana.294802eb.wasm.base64.js';
import { isomorphicCompileWasmFromBase64 } from '../../core/base/wasm.js';

/* eslint-disable @typescript-eslint/naming-convention -- KMS transcript wire fields */
interface TranscriptDomain {
  name: string;
  version: string;
  /** The gateway chain id as 32 big-endian bytes. */
  chain_id: number[];
  verifying_contract: string;
  salt: null;
}

interface TranscriptRequest {
  signature: null;
  client_address: string;
  enc_key: string;
  ciphertext_handles: string[];
  eip712_verifying_contract: string;
  extra_data: string;
}

interface TranscriptShare {
  signature: string;
  payload: string;
  extra_data: string;
}

interface Transcript {
  fhe_parameter: string;
  server_addrs: { id: number; addr: string }[];
  solana_user_pubkey: string;
  host_chain_id: string;
  verifying_program_id: string;
  request: TranscriptRequest;
  eip712_domain: TranscriptDomain;
  responses: TranscriptShare[];
  enc_pk: string;
  enc_sk: string;
  expected: { fhe_type: number; plaintext_hex: string }[];
}
/* eslint-enable @typescript-eslint/naming-convention */

beforeAll(async () => {
  await init({ module_or_path: await isomorphicCompileWasmFromBase64(tkmsWasmBase64) });
});

describe.each(['central', 'threshold'])('%s KMS plaintext transcript', (mode) => {
  const transcript = JSON.parse(
    readFileSync(
      new URL(`../../../../../solana/test-fixtures/user-decrypt/${mode}-wasm-transcript.json`, import.meta.url),
      'utf8',
    ),
  ) as Transcript;

  /** One verification with the transcript's inputs, each overridable to stand in for a changed request. */
  function recover(
    overrides: {
      responses?: TranscriptShare[];
      request?: Partial<TranscriptRequest>;
      solanaRequest?: Partial<{ user_pubkey: string; host_chain_id: string; verifying_program_id: string }>;
      eip712Domain?: TranscriptDomain | undefined;
    } = {},
  ) {
    const client = new_solana_client(
      transcript.server_addrs.map(({ id, addr }) => new_server_id_addr(id, addr)),
      transcript.fhe_parameter,
    );
    const publicKey = u8vec_to_ml_kem_pke_pk(Buffer.from(transcript.enc_pk, 'hex'));
    const secretKey = u8vec_to_ml_kem_pke_sk(Buffer.from(transcript.enc_sk, 'hex'));
    try {
      return process_user_decryption_resp_solana_from_js(
        client,
        { ...transcript.request, ...overrides.request },
        {
          user_pubkey: transcript.solana_user_pubkey,
          host_chain_id: transcript.host_chain_id,
          verifying_program_id: transcript.verifying_program_id,
          ...overrides.solanaRequest,
        },
        overrides.responses ?? transcript.responses,
        publicKey,
        secretKey,
        'eip712Domain' in overrides ? overrides.eip712Domain : transcript.eip712_domain,
      );
    } finally {
      client.free();
      publicKey.free();
      secretKey.free();
    }
  }

  /** The same bytes with one flipped, at `index`. */
  const flippedAt = (hex: string, index: number): string => {
    const bytes = Buffer.from(hex, 'hex');
    bytes[index] = (bytes[index] ?? 0) ^ 0x01;
    return bytes.toString('hex');
  };

  /**
   * How the blob reports a refusal by one rule: the centralized client names the single share and
   * the rule; the threshold client counts the rejected shares per rule and reports that none of the
   * required number survived. Either way the rule that did the refusing is in the text.
   */
  const refusedBy = (rule: 'node_signature' | 'link_mismatch'): RegExp => {
    if (mode === 'threshold') {
      return new RegExp(
        `only 0 of the \\d+ required user decryption shares carry the recomputed link .*${rule}: ${transcript.responses.length}`,
      );
    }
    return rule === 'node_signature'
      ? /node signature on the response from party 1 is not valid/
      : /not the link recomputed from the request/;
  };

  it('recovers the expected plaintext with the shipped WASM', () => {
    const plaintexts = recover();
    try {
      expect(
        plaintexts.map((plaintext) => ({
          fhe_type: plaintext.fhe_type,
          // Rust transcripts store little endian; the JS API returns big endian.
          plaintext_hex: Buffer.from(plaintext.bytes).reverse().toString('hex'),
        })),
      ).toEqual(transcript.expected);
    } finally {
      plaintexts.forEach((plaintext) => plaintext.free());
    }
  });

  it('rejects shares with invalid authentication', () => {
    expect(() =>
      recover({ responses: transcript.responses.map((share) => ({ ...share, signature: '00'.repeat(65) })) }),
    ).toThrow(refusedBy('node_signature'));
  });

  // The same real shares, once the client's request differs in one bound input. Each is refused by
  // the link rule: the shares were minted for the transcript's request, and the recomputed link is
  // another request's.
  it('refuses the shares once the request names another recipient', () => {
    expect(() => recover({ solanaRequest: { user_pubkey: flippedAt(transcript.solana_user_pubkey, 0) } })).toThrow(
      refusedBy('link_mismatch'),
    );
  });

  it('refuses the shares once the request names another program', () => {
    expect(() =>
      recover({ solanaRequest: { verifying_program_id: flippedAt(transcript.verifying_program_id, 31) } }),
    ).toThrow(refusedBy('link_mismatch'));
  });

  it('refuses the shares once the request declares another host chain than its handles embed', () => {
    expect(() =>
      recover({ solanaRequest: { host_chain_id: (BigInt(transcript.host_chain_id) + 1n).toString() } }),
    ).toThrow(/does not match handle chain ID/);
  });

  // The gateway domain is both the domain the link is hashed under and the domain the node signed
  // under, so a client configured with another gateway's domain refuses real shares — by the
  // signature rule, which runs first — rather than accepting them under a link of its own.
  it('refuses the shares under a foreign gateway domain', () => {
    const chainId = [...transcript.eip712_domain.chain_id];
    chainId[31] = (chainId[31] ?? 0) ^ 0x01;
    expect(() => recover({ eip712Domain: { ...transcript.eip712_domain, chain_id: chainId } })).toThrow(
      refusedBy('node_signature'),
    );
  });

  it('refuses to verify anything without a gateway domain', () => {
    expect(() => recover({ eip712Domain: undefined })).toThrow(/eip712_domain is required/);
  });

  // The route is not in the link, so this is the node-signature rule's doing: the signed message
  // carries the route, and a share whose route is not the request's does not verify.
  it('refuses shares that carry another KMS route than the request', () => {
    expect(() =>
      recover({
        responses: transcript.responses.map((share) => ({ ...share, extra_data: flippedAt(share.extra_data, 0) })),
      }),
    ).toThrow(refusedBy('node_signature'));
  });
});
