import { createFhevmBaseClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseCanUseUnifiedDecryptionPermitTests } from '../ethers-common/clientBase.canUseUnifiedDecryptionPermit.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.canUseUnifiedDecryptionPermit.test.ts
// CHAIN=testnet    npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.canUseUnifiedDecryptionPermit.test.ts
// CHAIN=devnet     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.canUseUnifiedDecryptionPermit.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseCanUseUnifiedDecryptionPermitTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmBaseClient(params),
});
