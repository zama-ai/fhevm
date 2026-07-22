import type { FhevmChain } from '../../types/fhevmChain.js';
import { defineFhevmChain } from '../utils.js';

export const polygonAmoy: FhevmChain = /*#__PURE__*/ defineFhevmChain({
  id: 80_002,
  fhevm: {
    contracts: {
      acl: {
        address: '0xD99Cb9Fc3c42c87f2A4A12e8Fd60318d6bDdf985',
      },
      inputVerifier: {
        address: '0x6e5A7D8b0c645467Cba7e62D6624917085118631',
      },
      kmsVerifier: {
        address: '0xCD1D89E311bce4C8DEa9a0857a0c9A4E153D4041',
      },
      protocolConfig: {
        address: '0x4CcF009Aba90D04f52b31fc7aDdE240578aFe10F',
      },
    },
    relayerUrl: 'https://relayer.testnet.zama.org',
    gateway: {
      id: 10_901,
      contracts: {
        decryption: {
          address: '0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478',
        },
        inputVerification: {
          address: '0x483b9dE06E4E4C7D35CCf5837A1668487406D955',
        },
      },
    },
  },
});
