import type { FhevmChain } from '../../types/fhevmChain.js';
import { defineFhevmChain } from '../utils.js';

export const polygon: FhevmChain = /*#__PURE__*/ defineFhevmChain({
  id: 137,
  fhevm: {
    contracts: {
      acl: {
        address: '0x6737F17e31cf26a1b62fb0362acC5a16CB156F49',
      },
      inputVerifier: {
        address: '0xf40BD204B035522EaAc8E5afAdc55113Acac96ca',
      },
      kmsVerifier: {
        address: '0x14e609595474874Dd6b6128376E336EfADfdBE37',
      },
      protocolConfig: {
        address: '0x17f62Ab3A1Ea519703cD597410147A30Fa1a7f1e',
      },
    },
    relayerUrl: 'https://relayer.mainnet.zama.org',
    gateway: {
      id: 261_131,
      contracts: {
        decryption: {
          address: '0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24',
        },
        inputVerification: {
          address: '0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287',
        },
      },
    },
  },
});
