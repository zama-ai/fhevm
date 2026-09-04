import { createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptExtraDataV0Tests } from '../ethers-common/clientDecrypt.extraDataV0.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=testnet npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV0.test.ts
// CHAIN=devnet  npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV0.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptExtraDataV0Tests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
