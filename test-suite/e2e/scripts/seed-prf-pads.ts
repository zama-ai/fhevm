// Writes the complete `pads` row of SEEDED_OUTPUT_REF into one coprocessor's database, signed with
// that coprocessor's key, as its pad-worker will once it exists
// (https://github.com/zama-ai/fhevm-internal/issues/2193). prf-service then serves it to the
// Coprocessor proxy suite. Waits for the `pads` table of the migration first.
//
// Env: DATABASE_URL, a role that may write every `pads` column (not prf_service);
//      COPROCESSOR_SIGNER_PRIVATE_KEY, the key of that coprocessor's signer.
// Usage: npx ts-node --transpile-only scripts/seed-prf-pads.ts
import { getBytes } from 'ethers';
import { Client } from 'pg';

import {
  SEEDED_CIPHERTEXT128,
  SEEDED_CIPHERTEXT_DIGEST,
  SEEDED_OUTPUT_REF,
  prfOutputPath,
  signPrfOutput,
} from '../test/coprocessorProxy/prfOutput';

const MIGRATION_TIMEOUT_MS = 10 * 60_000;

const required = (name: string) => {
  const value = process.env[name];
  if (!value) throw new Error(`${name} is required`);
  return value;
};

const waitForPadsTable = async (client: Client) => {
  for (const deadline = Date.now() + MIGRATION_TIMEOUT_MS; ;) {
    const { rows } = await client.query("SELECT to_regclass('public.pads') IS NOT NULL AS ready");
    if (rows[0].ready) return;
    if (Date.now() > deadline) throw new Error('no pads table: the coprocessor migration did not run');
    await new Promise((resolve) => setTimeout(resolve, 2_000));
  }
};

const main = async () => {
  const ref = SEEDED_OUTPUT_REF;
  const attestation = signPrfOutput(
    required('COPROCESSOR_SIGNER_PRIVATE_KEY'),
    ref,
    SEEDED_CIPHERTEXT_DIGEST,
    SEEDED_CIPHERTEXT128,
  );
  const client = new Client({ connectionString: required('DATABASE_URL') });
  await client.connect();
  try {
    await waitForPadsTable(client);
    // No KMS connector paid for this row: it never was pending.
    await client.query(
      `INSERT INTO pads (prf_instance, label, pad_length, triggered_by, ciphertext128, ciphertext_digest,
                         ciphertext128_digest, signer, signature, attestation_version)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
       ON CONFLICT (prf_instance, label, pad_length) DO UPDATE
               SET is_error = FALSE, ciphertext128 = EXCLUDED.ciphertext128,
                   ciphertext_digest = EXCLUDED.ciphertext_digest,
                   ciphertext128_digest = EXCLUDED.ciphertext128_digest,
                   signer = EXCLUDED.signer, signature = EXCLUDED.signature,
                   attestation_version = EXCLUDED.attestation_version`,
      [
        ref.prfId,
        Buffer.from(ref.label, 'hex'),
        ref.padLength,
        Buffer.alloc(20),
        Buffer.from(SEEDED_CIPHERTEXT128),
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
  console.log(`[seed-prf-pads] ${prfOutputPath(ref)} complete, signed by ${attestation.signer}`);
};

main().catch((err) => {
  console.error(`[seed-prf-pads] ${err instanceof Error ? err.message : err}`);
  process.exit(1);
});
