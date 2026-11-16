import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext, isLocalstackChain } from '../setupCommon.js';
import { defineClientDecryptStalePermitMigrationTests } from '../viem-common/clientDecrypt.stalePermitMigration.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.stalePermitMigration.test.ts
//
////////////////////////////////////////////////////////////////////////////////

const chainName = getViemTestConfig().chainName;

defineClientDecryptStalePermitMigrationTests({
  runIf: !isCleartext(chainName),
  isProtocolV14OrHigher: isLocalstackChain(chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
