import { createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptExtraDataV2Tests } from '../ethers-common/clientDecrypt.extraDataV2.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=testnet npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV2.test.ts
// CHAIN=devnet  npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV2.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptExtraDataV2Tests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
