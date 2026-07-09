import { defineFhevmChain } from '@fhevm/sdk/chains';

export const ingen_trex_cleartext = /*#__PURE__*/ defineFhevmChain({
  id: 364_301,
  fhevm: {
    contracts: {
      acl: {
        address: '0x09a4710BfBe7B557cD5CFE88BB31e9b5b85C419b',
      },
      inputVerifier: {
        address: '0x90f05B10db153365D8cB143EA17f5E5714D0bCD5',
      },
      kmsVerifier: {
        address: '0xd885DEa6a924785fCcdf9CE993FEe27EA11832e6',
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
