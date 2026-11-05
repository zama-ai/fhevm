import {
  createFhevmCleartextEncryptWithPadClient,
  createFhevmCleartextDecryptClient,
} from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isLocalCleartextChain } from '../setupCommon.js';
import { defineClientEncryptWithPadDecryptSlowTests } from '../ethers-common/clientEncryptWithPad.encryptDecrypt.slow.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v1_0 npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncryptWithPad.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptWithPadDecryptSlowTests({
  // RFC-030 pad-based encryption needs the v1.0 cleartext stack (PaddedInputter).
  runIf: isLocalCleartextChain(getEthersTestConfig().chainName),
  createFhevmEncryptWithPadClient: (params) => createFhevmCleartextEncryptWithPadClient(params),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
