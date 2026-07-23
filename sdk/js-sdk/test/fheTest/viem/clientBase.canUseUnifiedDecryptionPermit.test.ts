import { createFhevmBaseClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseCanUseUnifiedDecryptionPermitTests } from '../viem-common/clientBase.canUseUnifiedDecryptionPermit.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.canUseUnifiedDecryptionPermit.test.ts
// CHAIN=testnet    npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.canUseUnifiedDecryptionPermit.test.ts
// CHAIN=devnet     npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.canUseUnifiedDecryptionPermit.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseCanUseUnifiedDecryptionPermitTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmBaseClient(params),
});
