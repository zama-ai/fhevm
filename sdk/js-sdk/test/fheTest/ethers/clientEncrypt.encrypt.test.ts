import { createFhevmEncryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../ethers-common/clientEncrypt.encrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encrypt.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encrypt.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptEncryptTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmEncryptClient(params),
});
