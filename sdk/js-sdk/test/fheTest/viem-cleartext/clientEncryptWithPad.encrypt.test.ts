import { createFhevmCleartextEncryptWithPadClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isLocalCleartextChain } from '../setupCommon.js';
import { defineClientEncryptWithPadEncryptTests } from '../viem-common/clientEncryptWithPad.encrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientEncryptWithPad.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptWithPadEncryptTests({
  // RFC-030 pad-based encryption needs the v1.0 cleartext stack (PaddedInputter).
  runIf: isLocalCleartextChain(getViemTestConfig().chainName),
  createFhevmEncryptWithPadClient: (params) => createFhevmCleartextEncryptWithPadClient(params),
});
