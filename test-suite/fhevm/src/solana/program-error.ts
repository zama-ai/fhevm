// The custom program error a refused Solana transaction carries, for scenarios and profiles that
// must pin the exact refusal rather than any failure.
import { isSolanaError, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM } from '@solana/kit';

/** Anchor `ErrorCode::AccountNotInitialized`: an `Account<T>` names an address that holds no account. */
export const ANCHOR_ACCOUNT_NOT_INITIALIZED = 3012;

/**
 * Walks a kit SolanaError cause chain (preflight failure -> transaction error -> instruction
 * error) to the custom program error code, if one is there.
 */
const customProgramErrorCode = (error: unknown): number | undefined => {
  for (let current = error, depth = 0; current && depth < 8; depth++) {
    if (isSolanaError(current, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM)) return Number(current.context.code);
    current = (current as { cause?: unknown }).cause;
  }
  return undefined;
};

/** Runs `send` and throws unless it fails with the custom program error `code`. */
export const expectProgramError = async (description: string, code: number, send: () => Promise<unknown>): Promise<void> => {
  const rejection = await send().then(
    () => undefined,
    (error: unknown) => error,
  );
  if (rejection === undefined) throw new Error(`${description}: the transaction succeeded`);
  const actual = customProgramErrorCode(rejection);
  if (actual !== code) {
    throw new Error(`${description}: expected custom program error ${code}, got ${actual ?? 'no custom error'}`, {
      cause: rejection,
    });
  }
};
