import { createFhevmEncryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../viem-common/clientEncrypt.encrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptEncryptTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmEncryptClient(params),
});
