import type { Logger } from '../../../src/core/types/logger.js';

// Delay after the page reports its result, so BrowserStack's video/screenshot
// capture shows the completed page state before the test tears down.
export const RESULT_SETTLE_DELAY_MS = 3_000;

/** Formats a `Date` as `YYYY-MM-DD HH:mm:ss.SSS` in local time. */
function formatLogTimestamp(date: Date): string {
  const pad = (n: number, len = 2): string => String(n).padStart(len, '0');
  const datePart = `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
  const timePart = `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}.${pad(date.getMilliseconds(), 3)}`;
  return `${datePart} ${timePart}`;
}

export function createLogger(log: (msg: string) => void, chainName?: string): Logger {
  // Prefix every line with the chain under test so interleaved multi-chain /
  // multi-suite output stays attributable. Defaults to the CHAIN env var
  // (e.g. `[testnet]`); pass `config.chainName` for exact per-config tagging.
  const chain = chainName ?? (typeof process !== 'undefined' ? process.env?.CHAIN : undefined) ?? 'sepolia';
  return {
    debug: (message: string) => log(`${formatLogTimestamp(new Date())} DBG [${chain}] ${message}`),
    warn: (message: string) => log(`${formatLogTimestamp(new Date())} WRN [${chain}] ${message}`),
    error: (message: string, cause: unknown) => {
      const timestamp = formatLogTimestamp(new Date());
      log(`${timestamp} ERR [${chain}] ${message}`);
      if (cause !== undefined) {
        log(`${timestamp} ERR [${chain}] ${String(cause)}`);
      }
    },
  };
}

/** Reads `fheTestAddress` for `chainName` from test/chains/chain-defaults.json. */
export async function fetchFheTestAddress(chainName: string): Promise<string> {
  const res = await fetch('/test/chains/chain-defaults.json');
  const defaults = await res.json();
  const address = defaults?.[chainName]?.fheTestAddress;
  if (!address) {
    throw new Error(`Missing fheTestAddress for "${chainName}" in test/chains/chain-defaults.json.`);
  }
  return address as string;
}

export function createPageHarness() {
  const logEl = document.getElementById('log')!;
  logEl.style.maxHeight = '80vh';
  logEl.style.overflowY = 'auto';

  const t0 = performance.now();

  function elapsed(): string {
    return (performance.now() - t0).toFixed(0);
  }

  function log(msg: string) {
    const line = `[${elapsed()}ms] ${msg}`;
    logEl.textContent += `${line}\n`;
    logEl.scrollTop = logEl.scrollHeight;
    console.log(line);
  }

  function done(status: 'pass' | 'fail') {
    const el = document.createElement('div');
    el.id = 'result';
    el.dataset.status = status;
    el.className = status;
    el.textContent = status.toUpperCase();
    document.body.appendChild(el);
  }

  return { log, done, elapsed };
}
