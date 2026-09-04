import { createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext, protocolEraOf } from '../setupCommon.js';
import { defineClientDecryptStalePermitMigrationTests } from '../ethers-common/clientDecrypt.stalePermitMigration.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack_v12 npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.stalePermitMigration.test.ts
// CHAIN=localstack_v13 npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.stalePermitMigration.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.stalePermitMigration.test.ts
//
////////////////////////////////////////////////////////////////////////////////

const chainName = getEthersTestConfig().chainName;
const era = protocolEraOf(chainName);

defineClientDecryptStalePermitMigrationTests({
  // On protocol v0.11 the current context already encodes to extraData v0
  // (0x00), so there is no migration scenario to exercise.
  runIf: !isCleartext(chainName) && era >= 12,
  era,
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
