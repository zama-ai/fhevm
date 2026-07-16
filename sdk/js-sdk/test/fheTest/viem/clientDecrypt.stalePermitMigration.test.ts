import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext, protocolEraOf } from '../setupCommon.js';
import { defineClientDecryptStalePermitMigrationTests } from '../viem-common/clientDecrypt.stalePermitMigration.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack_v12 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.stalePermitMigration.test.ts
// CHAIN=localstack_v13 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.stalePermitMigration.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.stalePermitMigration.test.ts
//
////////////////////////////////////////////////////////////////////////////////

const chainName = getViemTestConfig().chainName;
const era = protocolEraOf(chainName);

defineClientDecryptStalePermitMigrationTests({
  // On protocol v0.11 the current context already encodes to extraData v0
  // (0x00), so there is no migration scenario to exercise.
  runIf: !isCleartext(chainName) && era >= 12,
  era,
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
