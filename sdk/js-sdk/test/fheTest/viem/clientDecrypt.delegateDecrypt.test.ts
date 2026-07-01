import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDelegateDecryptTests } from '../viem-common/clientDecrypt.delegateDecrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.delegateDecrypt.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.delegateDecrypt.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.delegateDecrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptDelegateDecryptTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
