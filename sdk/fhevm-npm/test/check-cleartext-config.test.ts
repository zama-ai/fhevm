import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { generateCleartextConfig } from '../base/generate-cleartext-config.ts';
import { checkCleartextConfig } from '../commands/check-cleartext-config.ts';
import type { NpmManifest } from '../manifest.ts';

// The command reads only workspaceRoot; the manifest is untouched by design.
const UNUSED_MANIFEST = {} as NpmManifest;

function makeWorkspace(): string {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-check-cleartext-config-'));
  writeFileSync(
    join(workspace, 'cleartext-config.json'),
    JSON.stringify({
      appliesTo: { generations: ['v13'] },
      constants: { A_VALUE: { value: '1', ts: 'number', solidity: 'uint256' } },
      localhost: {
        MNEMONIC: { value: 'adapt mosquito move limb' },
        DEPLOYER_ADDRESS_INDEX: { value: '5' },
        DEPLOYER_ADDRESS: { value: '0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4' },
        DEPLOYER_START_NONCE: { value: '0' },
        zamaConfigLocal: {
          ACLAddress: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
          CoprocessorAddress: '0xe3a9105a3a932253A70F126eb1E3b589C643dD24',
          KMSVerifierAddress: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
        },
      },
    }),
  );
  return workspace;
}

test('reports missing faces, then a clean report once generated, then the one drifted face', () => {
  const workspace = makeWorkspace();
  try {
    const missing = checkCleartextConfig({ workspaceRoot: workspace, manifest: UNUSED_MANIFEST });
    assert.equal(missing.command, 'check-cleartext-config');
    assert.equal(missing.checkedPackageKeys.length, 3);
    assert.deepEqual(
      missing.violations.map((violation) => violation.rule),
      ['cleartext-config-face', 'cleartext-config-face', 'cleartext-config-face'],
    );
    assert.match(missing.violations[0]?.message ?? '', /missing/);

    generateCleartextConfig({ workspaceRoot: workspace, check: false });
    assert.deepEqual(checkCleartextConfig({ workspaceRoot: workspace, manifest: UNUSED_MANIFEST }).violations, []);

    writeFileSync(
      join(workspace, 'host-contracts-cleartext', 'v13', 'scripts', 'cleartext-config.sh'),
      'A_VALUE="2"\n',
    );
    const drifted = checkCleartextConfig({ workspaceRoot: workspace, manifest: UNUSED_MANIFEST });
    assert.deepEqual(
      drifted.violations.map((violation) => violation.packageKey),
      ['./host-contracts-cleartext/v13/scripts/cleartext-config.sh'],
    );
    assert.match(drifted.violations[0]?.message ?? '', /differs from sdk\/cleartext-config\.json/);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});
