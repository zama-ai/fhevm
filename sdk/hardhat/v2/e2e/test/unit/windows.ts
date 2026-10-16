import { expect } from 'chai';

import { toUnixRelPath } from '../../../plugin/pkg/src/internal/utils/path';

describe('Windows', function () {
  // eslint-disable-next-line @typescript-eslint/require-await
  it('path', async function () {
    // Invalid import fhevmTemp\@fhevm\solidity\config/ZamaConfig.sol from contracts/FHECounterPublicDecrypt.sol. Imports must use / instead of \, even in Windows
    const str = 'fhevmTemp\\@fhevm\\solidity\\config/ZamaConfig.sol';
    expect(toUnixRelPath(str) === 'fhevmTemp/@fhevm/solidity/config/ZamaConfig.sol');
  });
});
