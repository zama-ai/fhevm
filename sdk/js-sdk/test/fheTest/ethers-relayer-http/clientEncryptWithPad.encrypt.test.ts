import { getEthersTestConfig } from '../setup-ethers.js';
import { defineClientEncryptWithPadEncryptTests } from '../ethers-common/clientEncryptWithPad.encrypt.tests.js';
import { RELAYER_HTTP_URL, createRelayerHttpEncryptWithPadClient } from './relayerHttpClient.js';

////////////////////////////////////////////////////////////////////////////////
//
// The pad shares through relayer-http (POST /v4/pad-decrypt) and its mock KMS connectors, on the v1.0.0 cleartext
// stack. Start them first (relayer-http/dev/README.md, "Pad decryption"), then:
//
// RELAYER_HTTP_URL=http://127.0.0.1:8080 CHAIN=localcleartext_v1_0 npx vitest run --config test/fheTest/vitest.config.ts ethers-relayer-http/clientEncryptWithPad.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptWithPadEncryptTests({
  runIf: RELAYER_HTTP_URL !== undefined && getEthersTestConfig().chainName === 'localcleartext_v1_0',
  createFhevmEncryptWithPadClient: (params) => createRelayerHttpEncryptWithPadClient(params),
});
