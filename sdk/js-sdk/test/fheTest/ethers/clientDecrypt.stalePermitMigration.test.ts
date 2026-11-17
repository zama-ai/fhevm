import { createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext, isLocalstackChain } from '../setupCommon.js';
import { defineClientDecryptStalePermitMigrationTests } from '../ethers-common/clientDecrypt.stalePermitMigration.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.stalePermitMigration.test.ts
//
////////////////////////////////////////////////////////////////////////////////

const chainName = getEthersTestConfig().chainName;

defineClientDecryptStalePermitMigrationTests({
  runIf: !isCleartext(chainName),
  isProtocolV14OrHigher: isLocalstackChain(chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
