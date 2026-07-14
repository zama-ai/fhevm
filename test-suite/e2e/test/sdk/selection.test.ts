import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { LEGACY_SDK_SELECTION_WARNING, selectSdk } from './selection';

describe('selectSdk', () => {
  it('selects an exact published relayer SDK explicitly', () => {
    assert.deepEqual(
      selectSdk({
        E2E_SDK_FAMILY: 'relayer-sdk',
        E2E_SDK_SOURCE: 'npm',
        E2E_SDK_VERSION: '0.4.4',
      }),
      { family: 'relayer-sdk', source: 'npm', requestedVersion: '0.4.4', legacy: false },
    );
  });

  it('selects a published fhevm SDK independently of the workspace SDK', () => {
    assert.deepEqual(
      selectSdk({
        E2E_SDK_FAMILY: 'fhevm-sdk',
        E2E_SDK_SOURCE: 'npm',
        E2E_SDK_VERSION: '1.1.0-alpha.4',
      }),
      { family: 'fhevm-sdk', source: 'npm', requestedVersion: '1.1.0-alpha.4', legacy: false },
    );
  });

  it('requires a complete explicit selection', () => {
    assert.throws(() => selectSdk({ E2E_SDK_FAMILY: 'fhevm-sdk' }), /E2E_SDK_SOURCE/);
    assert.throws(
      () =>
        selectSdk({
          E2E_SDK_FAMILY: 'fhevm-sdk',
          E2E_SDK_SOURCE: 'registry',
          E2E_SDK_VERSION: '1.1.0-alpha.4',
        }),
      /E2E_SDK_SOURCE/,
    );
  });

  it('rejects npm ranges so an image identifies one package', () => {
    assert.throws(
      () =>
        selectSdk({
          E2E_SDK_FAMILY: 'relayer-sdk',
          E2E_SDK_SOURCE: 'npm',
          E2E_SDK_VERSION: '^0.4.4',
        }),
      /exact published version/,
    );
  });

  it('retains the old relayer version variable as a marked legacy path', () => {
    assert.deepEqual(selectSdk({ RELAYER_SDK_VERSION: '0.4.2' }), {
      family: 'relayer-sdk',
      source: 'npm',
      requestedVersion: '0.4.2',
      legacy: true,
    });
  });

  it('rejects ranges through the legacy relayer variable', () => {
    assert.throws(() => selectSdk({ RELAYER_SDK_VERSION: '^0.4.2' }), /exact published version/);
  });

  it('retains the old empty selection as a marked workspace fallback', () => {
    assert.deepEqual(selectSdk({}), {
      family: 'fhevm-sdk',
      source: 'workspace',
      requestedVersion: 'workspace',
      legacy: true,
    });
    assert.match(LEGACY_SDK_SELECTION_WARNING, /E2E_SDK_FAMILY/);
  });
});
