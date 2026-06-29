import { createFhevmCleartextEncryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../viem-common/clientEncrypt.encrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientEncrypt.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptEncryptTests({
  runIf: isCleartext(getViemTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmCleartextEncryptClient(params),
});
