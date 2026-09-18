// timing — one line per protocol step so a live run shows where the time goes.
//
// The Solana scenarios wait on the coprocessor (SNS commit), the relayer (input proofs) and the
// KMS (decryptions). Printing each step's wall time turns a passing run into a latency record
// that can be compared across environments (local stack, preview namespace on devnet).

/** Runs `step` and prints `[timing] <label>: <ms>ms` when it settles, on success or failure. */
export const timed = async <T>(label: string, step: () => Promise<T>): Promise<T> => {
  const start = performance.now();
  try {
    return await step();
  } finally {
    console.log(`[timing] ${label}: ${Math.round(performance.now() - start)}ms`);
  }
};
