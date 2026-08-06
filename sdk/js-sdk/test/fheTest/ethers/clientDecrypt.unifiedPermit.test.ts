import { createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext, isRealDeployedChain, protocolEraOf } from '../setupCommon.js';
import { defineClientDecryptUnifiedPermitTests } from '../ethers-common/clientDecrypt.unifiedPermit.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.unifiedPermit.test.ts
//
////////////////////////////////////////////////////////////////////////////////

const chainName = getEthersTestConfig().chainName;

defineClientDecryptUnifiedPermitTests({
  // Unified (V2) permits require protocol v14+ (KMSVerifier >= 0.4.0 + ProtocolConfig)
  runIf: !isCleartext(chainName) && (protocolEraOf(chainName) >= 14 || isRealDeployedChain(chainName)),
  checkRelayerSupportsUnifiedPermit: isRealDeployedChain(chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
