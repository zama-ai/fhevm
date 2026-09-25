import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { parseManifest } from '../src/manifest/manifest.js';
import { ManifestError } from '../src/manifest/types.js';

const VALID = `
apiVersion: fhevm.zama.ai/scenario/v1
kind: Scenario
metadata:
  id: erc20.transfer
  name: Transfer
  owner: qa
  description: Transfer tokens.
  tags: [erc20]
spec:
  runtime: cucumber
  entrypoint: ./transfer.feature
  timeoutSeconds: 300
`;

function problemsOf(source: string): string[] {
  try {
    parseManifest('scenario.yaml', source);
  } catch (error) {
    assert.ok(error instanceof ManifestError);
    return error.problems;
  }
  assert.fail('expected a ManifestError');
}

describe('parseManifest', () => {
  it('accepts a valid manifest and applies defaults', () => {
    const manifest = parseManifest('scenario.yaml', VALID);
    assert.equal(manifest.metadata.id, 'erc20.transfer');
    assert.deepEqual(manifest.spec.supportCode, []);
    assert.equal(manifest.spec.enabled, true);
  });

  it('keeps optional fields', () => {
    const manifest = parseManifest(
      'scenario.yaml',
      VALID.replace('tags: [erc20]', 'tags: [erc20]\n  origin: EncryptedERC20.ts#transfer').replace(
        'timeoutSeconds: 300',
        'timeoutSeconds: 300\n  enabled: false\n  supportCode: [./steps.ts]',
      ),
    );
    assert.equal(manifest.metadata.origin, 'EncryptedERC20.ts#transfer');
    assert.equal(manifest.spec.enabled, false);
    assert.deepEqual(manifest.spec.supportCode, ['./steps.ts']);
  });

  it('rejects unknown fields so that no field is decorative', () => {
    const problems = problemsOf(VALID.replace('timeoutSeconds: 300', 'timeoutSeconds: 300\n  requires: {}'));
    assert.deepEqual(problems, ["spec: unknown field 'requires'"]);
  });

  it('reports every missing required field', () => {
    const problems = problemsOf(VALID.replace('  owner: qa\n', '').replace('  entrypoint: ./transfer.feature\n', ''));
    assert.ok(problems.includes("metadata: missing required field 'owner'"));
    assert.ok(problems.includes("spec: missing required field 'entrypoint'"));
  });

  it('rejects a wrong apiVersion', () => {
    assert.deepEqual(problemsOf(VALID.replace('scenario/v1', 'scenario/v9')), [
      "apiVersion: must be 'fhevm.zama.ai/scenario/v1'",
    ]);
  });

  it('validates id format, timeout and file extensions', () => {
    assert.equal(problemsOf(VALID.replace('erc20.transfer', 'ERC20 Transfer')).length, 1);
    assert.equal(problemsOf(VALID.replace('timeoutSeconds: 300', 'timeoutSeconds: 0')).length, 1);
    assert.equal(problemsOf(VALID.replace('timeoutSeconds: 300', 'timeoutSeconds: 1.5')).length, 1);
    assert.equal(problemsOf(VALID.replace('./transfer.feature', './transfer.txt')).length, 1);
    assert.equal(
      problemsOf(VALID.replace('timeoutSeconds: 300', 'timeoutSeconds: 300\n  supportCode: [./steps.js]')).length,
      1,
    );
  });

  it('reports invalid YAML on one line', () => {
    const [problem] = problemsOf('metadata: [unclosed\n');
    assert.match(problem!, /^invalid YAML: /);
    assert.ok(!problem!.includes('\n'));
  });
});
