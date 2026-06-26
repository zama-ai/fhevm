import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptTransportKeyPairTests } from '../viem-common/clientDecrypt.transportKeyPair.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.transportKeyPair.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.transportKeyPair.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.transportKeyPair.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptTransportKeyPairTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
