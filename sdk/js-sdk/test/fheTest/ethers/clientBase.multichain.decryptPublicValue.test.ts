import { createFhevmBaseClient } from '@fhevm/sdk/ethers';
import { areAllEthersTestConfigsCleartext, isMultichain } from '../setup-ethers.js';
import { defineClientBaseMultichainDecryptPublicValueTests } from '../ethers-common/clientBase.multichain.decryptPublicValue.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=devnet,polygon_devnet npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.multichain.decryptPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseMultichainDecryptPublicValueTests({
  runIf: !areAllEthersTestConfigsCleartext() && isMultichain(),
  createFhevmBaseClient: (params) => createFhevmBaseClient(params),
});
