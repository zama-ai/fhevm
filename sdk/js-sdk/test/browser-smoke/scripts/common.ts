import type { Logger } from '../../../src/core/types/logger.js';

export function createLogger(log: (msg: string) => void): Logger {
  return {
    debug: (message: string) => log(`[debug] ${message}`),
    warn: (message: string) => log(`[warn] ${message}`),
    error: (message: string, cause: unknown) => {
      log(`[error] ${message}`);
      if (cause !== undefined) {
        log(`[error] ${cause}`);
      }
    },
  };
}
