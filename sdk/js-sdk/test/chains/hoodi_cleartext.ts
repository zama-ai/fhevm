import { defineFhevmChain } from '@fhevm/sdk/chains';

export const hoodi_cleartext = /*#__PURE__*/ defineFhevmChain({
  id: 560_048,
  fhevm: {
    contracts: {
      acl: {
        address: '0x6D3FAf6f86e1fF9F3B0831Dda920AbA1cBd5bd68',
      },
      inputVerifier: {
        address: '0xf3D9A51f32D9bC23E1eECb0fAbF1f1DA4d9Bba26',
      },
      kmsVerifier: {
        address: '0x822BE20679CfAfdc352F05dEdfe12a07E912212e',
      },
      protocolConfig: {
        address: '0x0000000000000000000000000000000000000000',
      },
    },
    relayerUrl: 'http://localhost:8545',
    gateway: {
      id: 10_901,
      contracts: {
        decryption: {
          address: '0x5ffdaAB0373E62E2ea2944776209aEf29E631A64',
        },
        inputVerification: {
          address: '0x812b06e1CDCE800494b79fFE4f925A504a9A9810',
        },
      },
    },
  },
});
