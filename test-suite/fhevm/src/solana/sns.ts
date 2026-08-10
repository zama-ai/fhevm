// SNS-commit wait: the one poll every live Solana assertion path shares.
//
// A handle is decryptable only after the SNS worker materializes its ciphertexts into the
// coprocessor database (`ciphertext_digest.ciphertext` and `.ciphertext128` both non-NULL).
// Every consumer of a freshly rotated handle — public decrypt, user decrypt, burn certificates —
// has to wait for that commit first. This helper replaces the identical psql polling loops that
// lived in `full-vertical.sh`, `adversarial-l4.sh`, and `two-holder-transfer.ts`.

import { COPROCESSOR_DB_CONTAINER } from "../layout";
import { run } from "../utils/process";

const BYTES32 = /^0x[0-9a-f]{64}$/i;

export type SnsCommitOptions = {
  /** Overall deadline. Default 2 minutes, matching the 40 x 3s loops it replaces. */
  readonly timeoutMs?: number;
  /** Delay between polls. Default 3s. */
  readonly intervalMs?: number;
  /** Database container name. Default the compose stack's coprocessor DB. */
  readonly container?: string;
};

/** Waits until the SNS worker has committed both ciphertext forms for `handle` (0x-hex, 32 bytes). */
export const waitForSnsCommit = async (handle: string, options: SnsCommitOptions = {}): Promise<void> => {
  if (!BYTES32.test(handle)) throw new Error(`invalid handle before ciphertext wait: ${handle}`);
  const timeoutMs = options.timeoutMs ?? 120_000;
  const intervalMs = options.intervalMs ?? 3_000;
  const container = options.container ?? COPROCESSOR_DB_CONTAINER;
  const hex = handle.slice(2);
  const deadline = Date.now() + timeoutMs;
  for (;;) {
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
    if (result.code === 0 && result.stdout.trim() === "t") return;
    if (Date.now() >= deadline) throw new Error(`ciphertext materialization timed out for ${handle}`);
    await Bun.sleep(intervalMs);
  }
};
