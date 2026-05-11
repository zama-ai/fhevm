import { defineFhevmChain } from '../utils.js';

export const mainnet = /*#__PURE__*/ defineFhevmChain({
  id: 1,
  fhevm: {
    contracts: {
      acl: {
        address: '0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6',
      },
      inputVerifier: {
        address: '0xCe0FC2e05CFff1B719EFF7169f7D80Af770c8EA2',
      },
      kmsVerifier: {
        address: '0x77627828a55156b04Ac0DC0eb30467f1a552BB03',
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
