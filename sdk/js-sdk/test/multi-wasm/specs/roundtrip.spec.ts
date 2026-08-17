import { expect, test } from '@playwright/test';
import { loadLocalstackChainDefaults } from '../support/chainDefaults.js';
import { loadMatrix, selectMatrixEntries } from '../support/matrix.js';

const matrix = loadMatrix();
const entries = selectMatrixEntries(matrix, {
  tfhe: process.env.MULTI_WASM_TFHE_VERSION,
  kms: process.env.MULTI_WASM_KMS_VERSION,
  mode: process.env.MULTI_WASM_MODE,
  cdn: process.env.MULTI_WASM_CDN,
});
const chainName = process.env.CHAIN ?? 'localstack';
const { rpcUrl, mnemonic, fheTestAddress } = loadLocalstackChainDefaults(chainName);

const ENTRY_PROGRESS_MILESTONES: readonly { readonly progress: number; readonly pattern: RegExp }[] = [
  { progress: 0.02, pattern: /^Matrix entry:/ },
  { progress: 0.06, pattern: /^TFHE / },
  { progress: 0.1, pattern: /^Creating FHEVM client/ },
  { progress: 0.18, pattern: /^Initializing FHEVM client/ },
  { progress: 0.3, pattern: /^Encrypting / },
  { progress: 0.48, pattern: /^Submitting encrypted value/ },
  { progress: 0.65, pattern: /^Private decrypting value/ },
  { progress: 0.82, pattern: /^\s*private decrypt ->/ },
  { progress: 0.9, pattern: /^Public decrypting value/ },
  { progress: 0.97, pattern: /^\s*public decrypt ->/ },
  { progress: 1, pattern: /^All checks passed/ },
  { progress: 1, pattern: /^\[FAIL\]/ },
];

for (const [entryIndex, entry] of entries.entries()) {
  test(`multi-WASM encrypt/decrypt round-trip: ${entry.label}`, async ({ page }) => {
    let printedLiveLogs = false;
    const printProgress = createSuiteProgressPrinter(entryIndex, entries.length);

    printProgress(`Starting ${entry.id}`);

    page.on('console', (message) => {
      const text = message.text();
      if (message.type() !== 'log' || !text.startsWith('[multi-wasm] ')) {
        return;
      }

      printedLiveLogs = true;
      printProgress(text.slice('[multi-wasm] '.length));
    });

    const query = new URLSearchParams({
      tfhe: entry.versionPair.tfhe,
      kms: entry.versionPair.kms,
      mode: entry.wasmAssetLoadMode,
      cdn: entry.cdn,
      chainName,
      rpcUrl,
      mnemonic,
      fheTestAddress,
    });

    await page.goto(`/test/multi-wasm/pages/roundtrip.html?${query.toString()}`);

    const result = page.locator('#result');
    await result.waitFor({ timeout: 600_000 });

    const status = await result.getAttribute('data-status');
    const logs = (await page.locator('#log').textContent()) ?? '';
    if (!printedLiveLogs) {
      console.log(`Multi-WASM logs (${entry.id}):\n`, logs);
    }

    expect(status).toBe('pass');
  });
}

function createSuiteProgressPrinter(entryIndex: number, totalEntries: number): (line: string) => void {
  let entryProgress = 0;

  return (line: string): void => {
    const message = line.replace(/^\[\d+ms\]\s*/, '');
    const milestone = ENTRY_PROGRESS_MILESTONES.find((candidate) => candidate.pattern.test(message));
    if (milestone !== undefined) {
      entryProgress = Math.max(entryProgress, milestone.progress);
    }

    const suiteProgress = ((entryIndex + entryProgress) / totalEntries) * 100;
    console.log(`[suite ${formatPercent(suiteProgress)} | entry ${entryIndex + 1}/${totalEntries}] ${line}`);
  };
}

function formatPercent(value: number): string {
  return `${value.toFixed(1).padStart(5)}%`;
}
