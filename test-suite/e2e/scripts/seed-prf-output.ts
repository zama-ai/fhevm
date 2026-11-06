// Writes the complete `pads` row that the "Coprocessor proxy" suite requests, in place of the
// pad-worker. The signer and signature are placeholders: nothing checks them yet. Logs in as the
// database owner (DATABASE_URL). Safe to run again. Run: `node scripts/seed-prf-output.ts`.
import { keccak256 } from 'ethers';
import pg from 'pg';

// The suite's PRF_PATH: `/v1/prf/0/labels/0101…01/1`.
const PRF_INSTANCE = 0;
const LABEL = Buffer.alloc(32, 0x01);
const PAD_LENGTH = 1;
// All byte values: a client that decodes the body as text changes it.
const BODY = Buffer.from(Array.from({ length: 256 }, (_, i) => i));

const client = new pg.Client({ connectionString: process.env.DATABASE_URL });
await client.connect();
try {
  await client.query(
    `INSERT INTO pads (prf_instance, label, pad_length, triggered_by, ciphertext128, ciphertext_digest,
                       ciphertext128_digest, signer, signature, attestation_version)
          VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 1)
     ON CONFLICT (prf_instance, label, pad_length) DO UPDATE
             SET is_error = FALSE, ciphertext128 = EXCLUDED.ciphertext128,
                 ciphertext_digest = EXCLUDED.ciphertext_digest,
                 ciphertext128_digest = EXCLUDED.ciphertext128_digest, signer = EXCLUDED.signer,
                 signature = EXCLUDED.signature, attestation_version = EXCLUDED.attestation_version`,
    [
      PRF_INSTANCE,
      LABEL,
      PAD_LENGTH,
      Buffer.alloc(20),
      BODY,
      Buffer.alloc(32),
      Buffer.from(keccak256(BODY).slice(2), 'hex'),
      Buffer.alloc(20),
      Buffer.alloc(65),
    ],
  );
  console.log('[seed-prf-output] wrote the PRF output of the coprocessor proxy suite');
} finally {
  await client.end();
}
