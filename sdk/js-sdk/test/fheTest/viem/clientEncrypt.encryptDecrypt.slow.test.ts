import { createFhevmEncryptClient, createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptDecryptSlowTests } from '../viem-common/clientEncrypt.encryptDecrypt.slow.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encryptDecrypt.slow.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptDecryptSlowTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmEncryptClient(params),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
