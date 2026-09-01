import assert from 'node:assert/strict';
import test from 'node:test';

import { patchHardhatTemplateV2Manifest } from '../base/mirrors/hardhat-template-v2.ts';

test('applies the complete Hardhat v2 workspace mirror transformation', () => {
  const patched = patchHardhatTemplateV2Manifest({
    name: 'fhevm-hardhat-template',
    description: 'upstream',
    version: '0.4.1',
    dependencies: {
      '@fhevm/mock-utils': '^0.4.2',
      '@fhevm/solidity': '^0.11.1',
    },
    devDependencies: {
      '@fhevm/hardhat-plugin': '^0.4.2',
      '@zama-fhe/relayer-sdk': '^0.4.1',
    },
    scripts: { test: 'hardhat test' },
  });

  assert.equal(patched.name, 'fhevm-hardhat-template-v2');
  assert.equal((patched.dependencies as Record<string, string>)['@fhevm/mock-utils'], undefined);
  assert.equal((patched.dependencies as Record<string, string>)['@fhevm/solidity'], '^0.13.3');
  assert.equal((patched.devDependencies as Record<string, string>)['@zama-fhe/relayer-sdk'], undefined);
  assert.equal(
    (patched.scripts as Record<string, string>)['check:mirror'],
    'node ../../../fhevm-npm/fhevm-npm.ts check-mirror ./hardhat/v2/fhevm-hardhat-template',
  );
});
