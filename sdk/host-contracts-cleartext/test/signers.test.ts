import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

// The js-sdk relayer duplicates this package's cleartext signer config (mnemonic, HD paths, pool size)
// deep in its internals — see js-sdk .../relayer/cleartext/signers.ts. It is not exported, so we read
// the relevant files as text and assert they agree. A silent drift here breaks cleartext decrypt.
const PACKAGE_ROOT = join(dirname(fileURLToPath(import.meta.url)), '..');
const JS_SDK_SIGNERS_PATH = join(
  PACKAGE_ROOT,
  '..',
  'js-sdk',
  'src',
  'core',
  'modules',
  'relayer',
  'cleartext',
  'signers.ts',
);
const HOST_CONSTANTS_PATH = join(PACKAGE_ROOT, 'ts', 'constants.ts');
const HOST_COPROCESSOR_SIGNERS_PATH = join(PACKAGE_ROOT, 'ts', 'signers', 'defaultCoprocessorSigners.ts');
const HOST_KMS_SIGNERS_PATH = join(PACKAGE_ROOT, 'ts', 'signers', 'defaultKmsSigners.ts');

function read(path: string): string {
  return readFileSync(path, 'utf8');
}

function capture(source: string, pattern: RegExp, label: string): string {
  const value = pattern.exec(source)?.[1];
  if (value === undefined) {
    throw new Error(`Could not find ${label}`);
  }
  return value;
}

// Count the quoted addresses in an `export const <name> = [ ... ]` array.
function countAddresses(source: string, constName: string): number {
  const body = capture(source, new RegExp(`const ${constName}\\s*=\\s*\\[([\\s\\S]*?)\\]`), constName);
  return [...body.matchAll(/'0x[0-9a-fA-F]{40}'/g)].length;
}

void test('js-sdk cleartext signer config matches host-contracts-cleartext constants', () => {
  const sdk = read(JS_SDK_SIGNERS_PATH);
  const host = read(HOST_CONSTANTS_PATH);

  // Mnemonic.
  assert.equal(
    capture(sdk, /const FHEVM_TEST_MNEMONIC\s*=\s*'([^']+)'/, 'js-sdk FHEVM_TEST_MNEMONIC'),
    capture(host, /const FHEVM_MNEMONIC\s*=\s*'([^']+)'/, 'host FHEVM_MNEMONIC'),
    'mnemonic mismatch between js-sdk and host-contracts-cleartext',
  );

  // HD paths. The js-sdk stores only the suffix after the shared `m/44'/60'/` prefix (it prepends the
  // prefix in _fillSignersPrivateKey), whereas this package stores the full path.
  assert.equal(
    `m/44'/60'/${capture(sdk, /const COPROCESSOR_PATH\s*=\s*"([^"]+)"/, 'js-sdk COPROCESSOR_PATH')}`,
    capture(
      host,
      /const DEFAULT_COPROCESSORS_MNEMONIC_PATH\s*=\s*"([^"]+)"/,
      'host DEFAULT_COPROCESSORS_MNEMONIC_PATH',
    ),
    'coprocessor derivation path mismatch',
  );
  assert.equal(
    `m/44'/60'/${capture(sdk, /const KMS_PATH\s*=\s*"([^"]+)"/, 'js-sdk KMS_PATH')}`,
    capture(host, /const DEFAULT_KMS_NODES_MNEMONIC_PATH\s*=\s*"([^"]+)"/, 'host DEFAULT_KMS_NODES_MNEMONIC_PATH'),
    'kms derivation path mismatch',
  );

  // Number of signers. The js-sdk hardcodes the pool size in both `_fillSignersPrivateKey(...)` calls;
  // this package's pool size is the number of entries in each generated signer module.
  const sdkCounts = [...sdk.matchAll(/_fillSignersPrivateKey\([^)]*?,\s*(\d+),/g)].map((m) => Number(m[1]));
  assert.ok(sdkCounts.length >= 2, 'expected the js-sdk to populate both the coprocessor and kms maps');
  assert.ok(
    sdkCounts.every((count) => count === sdkCounts[0]),
    'js-sdk uses different signer counts for coprocessor vs kms',
  );
  assert.equal(
    sdkCounts[0],
    countAddresses(read(HOST_COPROCESSOR_SIGNERS_PATH), 'DEFAULT_COPROCESSOR_ADDRESSES'),
    'coprocessor signer count mismatch',
  );
  assert.equal(
    sdkCounts[0],
    countAddresses(read(HOST_KMS_SIGNERS_PATH), 'DEFAULT_KMS_NODE_ADDRESSES'),
    'kms signer count mismatch',
  );
});
