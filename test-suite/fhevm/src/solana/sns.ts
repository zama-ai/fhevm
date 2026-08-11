// SNS-commit wait: the one poll every live Solana assertion path shares.
//
// A handle is decryptable only after the SNS worker materializes its ciphertexts into the
// coprocessor database (`ciphertext_digest.ciphertext` and `.ciphertext128` both non-NULL).
// Every consumer of a freshly rotated handle — public decrypt, user decrypt, burn certificates —
// has to wait for that commit first. This helper replaces the identical psql polling loops that
// lived in `full-vertical.sh`, `adversarial-l4.sh`, and `two-holder-transfer.ts`.

import { COPROCESSOR_DB_CONTAINER } from "../layout";
import { run } from "../utils/process";
import { until } from "../utils/until";

const BYTES32 = /^0x[0-9a-f]{64}$/i;

// The most generous budget the retired bash gave any commit: 40 x 6s for the burned handle, the
// slowest case (five FHE steps in one instruction). The ordinary loops were 30 x 6s = 3 minutes;
// one shared ceiling is simpler than per-call budgets and a fast commit returns early anyway.
const SNS_COMMIT_TIMEOUT_MS = 240_000;
// A commit usually lands well inside one interval, and the probe is a local `docker exec` — poll
// at 1s so a fast commit is noticed promptly rather than sitting out the rest of a 3s tick.
const SNS_COMMIT_POLL_INTERVAL_MS = 1_000;

/**
 * Waits until the SNS worker has committed both ciphertext forms for `handle` (0x-hex, 32 bytes).
 * `container` defaults to the layout constant; the scenarios pass `env.coprocessorDbContainer` so
 * the documented `COPROCESSOR_DB_CONTAINER` override actually reaches this probe.
 */
export const waitForSnsCommit = async (
  handle: string,
  container: string = COPROCESSOR_DB_CONTAINER,
): Promise<void> => {
  if (!BYTES32.test(handle)) throw new Error(`invalid handle before ciphertext wait: ${handle}`);
  const hex = handle.slice(2);
  await until(
    async () => {
      const result = await run(
        [
          "docker",
          "exec",
          container,
          "psql",
          "-U",
          "postgres",
          "-d",
          "coprocessor",
          "-tAc",
          `SELECT ciphertext IS NOT NULL AND ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('${hex}','hex')`,
        ],
        { allowFailure: true },
      );
      return result.code === 0 && result.stdout.trim() === "t";
    },
    {
      timeoutMs: SNS_COMMIT_TIMEOUT_MS,
      intervalMs: SNS_COMMIT_POLL_INTERVAL_MS,
      description: `ciphertext materialization for ${handle}`,
    },
  );
};
