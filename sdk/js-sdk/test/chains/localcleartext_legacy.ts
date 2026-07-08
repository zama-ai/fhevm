import { defineFhevmChain } from '@fhevm/sdk/chains';

export const localcleartext_legacy = /*#__PURE__*/ defineFhevmChain({
  id: 31_337,
  fhevm: {
    contracts: {
      acl: {
        address: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
      },
      inputVerifier: {
        address: '0x36772142b74871f255CbD7A3e89B401d3e45825f',
      },
      kmsVerifier: {
        address: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
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
