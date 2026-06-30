import { createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptTransportKeyPairTests } from '../ethers-common/clientDecrypt.transportKeyPair.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.transportKeyPair.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.transportKeyPair.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.transportKeyPair.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptTransportKeyPairTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
