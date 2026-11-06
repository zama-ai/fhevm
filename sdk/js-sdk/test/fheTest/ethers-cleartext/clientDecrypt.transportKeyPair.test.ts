import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptTransportKeyPairTests } from '../ethers-common/clientDecrypt.transportKeyPair.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientDecrypt.transportKeyPair.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptTransportKeyPairTests({
  runIf: isCleartext(getEthersTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
