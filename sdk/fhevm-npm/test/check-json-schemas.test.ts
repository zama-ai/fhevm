import assert from 'node:assert/strict';
import { cpSync, mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import test from 'node:test';

import { checkJsonSchemas } from '../commands/check-json-schemas.ts';
import { loadNpmManifest } from '../manifest.ts';

const SOURCE_WORKSPACE = join(import.meta.dirname, '..', '..');
const MANIFEST = loadNpmManifest(join(SOURCE_WORKSPACE, 'npm-manifest.json'));
const ROOT_JSON_FILES = [
  'npm-manifest.json',
  'cleartext-config.json',
  'fhevm-chains.config.json',
  'fhevm-network-groups.config.json',
] as const;
const EXPORT_MANIFESTS = [
  join('host-contracts-cleartext', 'v12', 'export.manifest.json'),
  join('host-contracts-cleartext', 'v13', 'export.manifest.json'),
] as const;

function makeWorkspace(): string {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-json-schemas-'));
  for (const file of ROOT_JSON_FILES) cpSync(join(SOURCE_WORKSPACE, file), join(workspace, file));
  cpSync(join(SOURCE_WORKSPACE, 'fhevm-npm', 'schemas'), join(workspace, 'fhevm-npm', 'schemas'), {
    recursive: true,
  });
  for (const file of EXPORT_MANIFESTS) {
    mkdirSync(dirname(join(workspace, file)), { recursive: true });
    cpSync(join(SOURCE_WORKSPACE, file), join(workspace, file));
  }
  return workspace;
}

function report(workspaceRoot: string) {
  return checkJsonSchemas({ workspaceRoot, manifest: MANIFEST });
}

test('validates every real central schema and JSON configuration file', () => {
  const workspace = makeWorkspace();
  try {
    const result = report(workspace);
    assert.deepEqual(result.violations, []);
    assert.equal(result.checkedPackageKeys.length, 11);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('reports the JSON file and precise instance path for a schema violation', () => {
  const workspace = makeWorkspace();
  try {
    const path = join(workspace, 'fhevm-network-groups.config.json');
    const config = JSON.parse(readFileSync(path, 'utf8')) as {
      groups: { devnet: { relayerUrl: string } };
    };
    config.groups.devnet.relayerUrl = 'not-a-url';
    writeFileSync(path, JSON.stringify(config));

    assert.ok(
      report(workspace).violations.some(
        (violation) =>
          violation.rule === 'json-schema-validation' &&
          violation.packageKey === './fhevm-network-groups.config.json' &&
          violation.message.includes('/groups/devnet/relayerUrl'),
      ),
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('requires local schema references and rejects orphan schemas', () => {
  const workspace = makeWorkspace();
  try {
    const path = join(workspace, 'cleartext-config.json');
    const config = JSON.parse(readFileSync(path, 'utf8')) as Record<string, unknown>;
    delete config.$schema;
    writeFileSync(path, JSON.stringify(config));

    const violations = report(workspace).violations;
    assert.ok(
      violations.some(
        (violation) => violation.rule === 'json-schema-reference' && violation.packageKey === './cleartext-config.json',
      ),
    );
    assert.ok(
      violations.some(
        (violation) =>
          violation.rule === 'json-schema-coverage' &&
          violation.packageKey === './fhevm-npm/schemas/cleartext-config.schema.json',
      ),
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('validates every discovered export manifest', () => {
  const workspace = makeWorkspace();
  try {
    const relPath = join('host-contracts-cleartext', 'v13', 'export.manifest.json');
    const path = join(workspace, relPath);
    const manifest = JSON.parse(readFileSync(path, 'utf8')) as Record<string, unknown>;
    manifest.packageSpecifier = 42;
    writeFileSync(path, JSON.stringify(manifest));

    assert.ok(
      report(workspace).violations.some(
        (violation) =>
          violation.rule === 'json-schema-validation' &&
          violation.packageKey === `./${relPath}` &&
          violation.message.includes('/packageSpecifier'),
      ),
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('rejects a malformed schema document', () => {
  const workspace = makeWorkspace();
  try {
    const path = join(workspace, 'fhevm-npm', 'schemas', 'fhevm-chains.config.schema.json');
    const schema = JSON.parse(readFileSync(path, 'utf8')) as Record<string, unknown>;
    schema.unknownSchemaKeyword = true;
    writeFileSync(path, JSON.stringify(schema));

    assert.ok(
      report(workspace).violations.some(
        (violation) =>
          violation.rule === 'json-schema-document' &&
          violation.packageKey === './fhevm-npm/schemas/fhevm-chains.config.schema.json' &&
          violation.message.includes('unknown keyword'),
      ),
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});
