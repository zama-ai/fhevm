import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { generateCleartextConfig, renderCleartextConfigFaces } from '../base/generate-cleartext-config.ts';

const LOCALHOST = {
  MNEMONIC: { value: 'adapt mosquito move limb' },
  DEPLOYER_ADDRESS_INDEX: { value: '5' },
  DEPLOYER_ADDRESS: { value: '0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4' },
  DEPLOYER_START_NONCE: { value: '0' },
  zamaConfigLocal: {
    ACLAddress: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
    CoprocessorAddress: '0xe3a9105a3a932253A70F126eb1E3b589C643dD24',
    KMSVerifierAddress: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
  },
} as const;

function makeWorkspace(constants: Record<string, unknown>, overrides?: Record<string, unknown>): string {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-cleartext-config-'));
  const config = { appliesTo: { generations: ['v13'] }, constants, localhost: LOCALHOST, ...overrides };
  writeFileSync(join(workspace, 'cleartext-config.json'), JSON.stringify(config));
  return workspace;
}

const CONSTANTS = {
  CHAIN_ID: {
    value: '100733346448153',
    ts: 'number',
    tsEmit: 'bigint',
    solidity: 'uint256',
    formula: 'uint48(uint256(keccak256("fhevm.cheat.chainId cleartext gateway")))',
  },
  PLAIN_COUNT: { value: '4', ts: 'number', solidity: 'uint256' },
  URL: { value: 'https://relayer.cleartext.foo', ts: 'string', solidity: 'string' },
  HD_PATH: { value: "m/44'/60'/0'/2/", ts: 'string', solidity: 'string' },
  URL_ALIAS: { alias: 'URL', ts: 'string', solidity: 'string' },
} as const;

test('renders the TypeScript face: order, formula comments, literal shapes, quoting, aliases', () => {
  const workspace = makeWorkspace(CONSTANTS);
  try {
    const outputs = renderCleartextConfigFaces(workspace);
    assert.deepEqual(
      outputs.map((output) => output.path),
      [
        join(workspace, 'common-vendored', 'src', 'cleartext-config.ts'),
        join(workspace, 'host-contracts-cleartext', 'v13', 'create2-deploy', 'script', 'FhevmCleartextConfig.sol'),
        join(workspace, 'host-contracts-cleartext', 'v13', 'scripts', 'cleartext-config.sh'),
      ],
    );

    const body = (outputs[0]?.content ?? '').split('\n\n').slice(1).join('\n\n');
    assert.equal(
      body,
      [
        '// uint48(uint256(keccak256("fhevm.cheat.chainId cleartext gateway")))',
        'export const CHAIN_ID = 100733346448153n;',
        '',
        'export const PLAIN_COUNT = 4;',
        '',
        "export const URL = 'https://relayer.cleartext.foo';",
        '',
        // Double quotes exactly where prettier would put them: the value holds a single quote.
        "export const HD_PATH = \"m/44'/60'/0'/2/\";",
        '',
        'export const URL_ALIAS = URL;',
        '',
      ].join('\n'),
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('renders the Solidity face: declared types, bare addresses, quoted strings, aliases', () => {
  const constants = {
    ...CONSTANTS,
    AN_ADDRESS: { value: '0x6189F6c0c3E40B4a3c72ec86262295D78d845297', ts: 'string', solidity: 'address' },
    AN_INDEX: { value: '0', ts: 'number', solidity: 'uint32' },
  };
  const workspace = makeWorkspace(constants);
  try {
    const sol = renderCleartextConfigFaces(workspace)[1]?.content ?? '';
    assert.match(sol, /^\/\/ SPDX-License-Identifier: BSD-3-Clause-Clear\npragma solidity \^0\.8\.24;\n/);
    const expected = [
      'library FhevmCleartextConfig {',
      '    // uint48(uint256(keccak256("fhevm.cheat.chainId cleartext gateway")))',
      '    uint256 internal constant CHAIN_ID = 100733346448153;',
      '',
      '    uint256 internal constant PLAIN_COUNT = 4;',
      '',
      '    string internal constant URL = "https://relayer.cleartext.foo";',
      '',
      "    string internal constant HD_PATH = \"m/44'/60'/0'/2/\";",
      '',
      '    string internal constant URL_ALIAS = URL;',
      '',
      '    address internal constant AN_ADDRESS = 0x6189F6c0c3E40B4a3c72ec86262295D78d845297;',
      '',
      '    uint32 internal constant AN_INDEX = 0;',
      '}',
      '',
    ].join('\n');
    assert.equal(sol.slice(sol.indexOf('library FhevmCleartextConfig {')), expected);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('renders the shell face: verbatim constants, alias references, deploy recipe, ZamaConfig trio', () => {
  const workspace = makeWorkspace(CONSTANTS);
  try {
    const sh = renderCleartextConfigFaces(workspace)[2]?.content ?? '';
    assert.match(sh, /^#!\/usr\/bin\/env bash\n# AUTO-GENERATED/);
    assert.match(sh, /^# uint48\(uint256\(keccak256\("fhevm\.cheat\.chainId cleartext gateway"\)\)\)$/m);
    assert.match(sh, /^CHAIN_ID="100733346448153"$/m);
    assert.match(sh, /^HD_PATH="m\/44'\/60'\/0'\/2\/"$/m);
    assert.match(sh, /^URL_ALIAS="\$URL"$/m);
    assert.match(sh, /^MNEMONIC="adapt mosquito move limb"$/m);
    assert.match(sh, /^DEPLOYER_ADDRESS_INDEX="5"$/m);
    assert.match(sh, /^ZAMA_LOCAL_ACL="0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D"$/m);
    assert.match(sh, /^ZAMA_LOCAL_COPROCESSOR="0xe3a9105a3a932253A70F126eb1E3b589C643dD24"$/m);
    assert.match(sh, /^ZAMA_LOCAL_KMS_VERIFIER="0x901F8942346f7AB3a01F6D7613119Bca447Bb030"$/m);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('write mode creates every face; check mode reports identical, different and missing', () => {
  const workspace = makeWorkspace(CONSTANTS);
  const shFace = join(workspace, 'host-contracts-cleartext', 'v13', 'scripts', 'cleartext-config.sh');
  try {
    const missing = generateCleartextConfig({ workspaceRoot: workspace, check: true });
    assert.deepEqual(
      missing.map((output) => output.status),
      ['missing', 'missing', 'missing'],
    );

    generateCleartextConfig({ workspaceRoot: workspace, check: false });
    assert.match(readFileSync(shFace, 'utf8'), /AUTO-GENERATED by `fhevm-npm generate-cleartext-config`/);
    const identical = generateCleartextConfig({ workspaceRoot: workspace, check: true });
    assert.deepEqual(
      identical.map((output) => output.status),
      ['identical', 'identical', 'identical'],
    );

    writeFileSync(shFace, 'CHAIN_ID="1"\n');
    const drifted = generateCleartextConfig({ workspaceRoot: workspace, check: true });
    assert.deepEqual(
      drifted.map((output) => output.status),
      ['identical', 'identical', 'different'],
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('rejects a malformed source of truth instead of emitting a wrong face', () => {
  const entry = { value: '1', ts: 'number', solidity: 'uint256' };
  const cases: readonly [Record<string, unknown>, Record<string, unknown> | undefined, RegExp][] = [
    [{ BOTH: { ...entry, alias: 'X' } }, undefined, /exactly one of "value" or "alias"/],
    [{ DANGLING: { alias: 'MISSING', ts: 'string', solidity: 'string' } }, undefined, /not declared/],
    [{ WIDE_STRING: { value: 'x', ts: 'string', tsEmit: 'bigint', solidity: 'string' } }, undefined, /only widen/],
    [{ HEXY: { ...entry, value: '0x10' } }, undefined, /decimal digits/],
    [{ lower_case: entry }, undefined, /CONSTANT_CASE/],
    [{ BAD_SOL: { ...entry, solidity: 'function' } }, undefined, /unknown "solidity" type/],
    [{ EXPANDS: { value: 'has a $dollar', ts: 'string', solidity: 'string' } }, undefined, /cannot emit a shell/],
    [{}, undefined, /declares no "constants"/],
    [{ OK: entry }, { appliesTo: { generations: [] } }, /no "appliesTo.generations"/],
    [{ OK: entry }, { appliesTo: { generations: ['0.13.0'] } }, /not a generation key/],
    [{ OK: entry }, { localhost: undefined }, /no "localhost" block/],
  ];
  for (const [constants, overrides, message] of cases) {
    const workspace = makeWorkspace(constants, overrides);
    try {
      assert.throws(() => renderCleartextConfigFaces(workspace), message);
    } finally {
      rmSync(workspace, { recursive: true, force: true });
    }
  }
});
