import { readFileSync } from 'node:fs';
import { beforeAll, describe, expect, it } from 'vitest';
import init, {
  new_server_id_addr,
  new_solana_client,
  process_user_decryption_resp_solana_from_js,
  u8vec_to_ml_kem_pke_pk,
  u8vec_to_ml_kem_pke_sk,
} from '../../wasm/tkms/kms_lib.v0.15.0-0-solana.0ead5f74.js';
import { tkmsWasmBase64 } from '../../wasm/tkms/kms_lib_bg.v0.15.0-0-solana.0ead5f74.wasm.base64.js';
import { isomorphicCompileWasmFromBase64 } from '../../core/base/wasm.js';

// KMS core/service/src/client/solana_response.rs generates these deterministic test keys
// and transcripts in test_user_decryption_solana_and_write_transcript. That generator is
// unchanged between the vendored 0ead5f740 revision and source checkout fa749daab.
// Its handles are opaque KMS test inputs, not valid SDK handles: these tests pin the shipped
// WASM's plaintext recovery; SDK handle/type validation is exercised separately.
/* eslint-disable @typescript-eslint/naming-convention -- KMS transcript wire fields */
interface Transcript {
  fhe_parameter: string;
  server_addrs: { id: number; addr: string }[];
  solana_user_pubkey: string;
  host_chain_id: string;
  verifying_program_id: string;
  request: Record<string, unknown>;
  eip712_domain: Record<string, unknown>;
  responses: { signature: string; payload: string; extra_data: string }[];
  enc_pk: string;
  enc_sk: string;
  expected: { fhe_type: number; plaintext_hex: string }[];
}

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

  function recover(responses = transcript.responses) {
    const client = new_solana_client(
      transcript.server_addrs.map(({ id, addr }) => new_server_id_addr(id, addr)),
      transcript.fhe_parameter,
    );
    const publicKey = u8vec_to_ml_kem_pke_pk(Buffer.from(transcript.enc_pk, 'hex'));
    const secretKey = u8vec_to_ml_kem_pke_sk(Buffer.from(transcript.enc_sk, 'hex'));
    try {
      return process_user_decryption_resp_solana_from_js(
        client,
        transcript.request,
        {
          user_pubkey: transcript.solana_user_pubkey,
          host_chain_id: transcript.host_chain_id,
          verifying_program_id: transcript.verifying_program_id,
        },
        responses,
        publicKey,
        secretKey,
        transcript.eip712_domain,
      );
    } finally {
      client.free();
      publicKey.free();
      secretKey.free();
    }
  }

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
    expect(() => recover(transcript.responses.map((share) => ({ ...share, signature: '00'.repeat(65) })))).toThrow();
  });
});
