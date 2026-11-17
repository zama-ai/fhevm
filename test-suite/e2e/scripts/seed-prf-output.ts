// Writes the complete `pads` row that the "Coprocessor proxy" suite requests, signed with this
// coprocessor's key, in place of the pad-worker
// (https://github.com/zama-ai/fhevm-internal/issues/2193). Safe to run again.
//
// Env: DATABASE_URL, the database owner (prf_service cannot write outputs);
//      TX_SENDER_PRIVATE_KEY, the key this coprocessor signs attestations with.
// Run: npx ts-node --transpile-only scripts/seed-prf-output.ts
import { getBytes } from 'ethers';
import pg from 'pg';

import {
  SEEDED_BODY,
  SEEDED_CIPHERTEXT_DIGEST,
  SEEDED_REF,
  prfPath,
  signPrfOutput,
} from '../test/coprocessorProxy/prfOutput';

async function main() {
  const attestation = signPrfOutput(
    process.env.TX_SENDER_PRIVATE_KEY!,
    SEEDED_REF,
    SEEDED_CIPHERTEXT_DIGEST,
    SEEDED_BODY,
  );
  const client = new pg.Client({ connectionString: process.env.DATABASE_URL });
  await client.connect();
  try {
    await client.query(
      `INSERT INTO pads (prf_instance, label, pad_length, triggered_by, ciphertext128, ciphertext_digest,
                         ciphertext128_digest, signer, signature, attestation_version)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
       ON CONFLICT (prf_instance, label, pad_length) DO UPDATE
               SET is_error = FALSE, ciphertext128 = EXCLUDED.ciphertext128,
                   ciphertext_digest = EXCLUDED.ciphertext_digest,
                   ciphertext128_digest = EXCLUDED.ciphertext128_digest, signer = EXCLUDED.signer,
                   signature = EXCLUDED.signature, attestation_version = EXCLUDED.attestation_version`,
      [
        SEEDED_REF.prfId,
        Buffer.from(getBytes(SEEDED_REF.label)),
        SEEDED_REF.padLength,
        Buffer.alloc(20),
        Buffer.from(SEEDED_BODY),
        Buffer.from(getBytes(attestation.ciphertext_digest)),
        Buffer.from(getBytes(attestation.sns_ciphertext_digest)),
        Buffer.from(getBytes(attestation.signer)),
        Buffer.from(getBytes(attestation.signature)),
        attestation.version,
      ],
    );
  } finally {
    await client.end();
  }
  console.log(`[seed-prf-output] wrote ${prfPath(SEEDED_REF)}, signed by ${attestation.signer}`);
}

main().catch((err) => {
  console.error('[seed-prf-output]', err);
  process.exit(1);
});
