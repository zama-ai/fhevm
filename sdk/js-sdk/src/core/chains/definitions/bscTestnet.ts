import type { FhevmChain } from '../../types/fhevmChain.js';
import { defineFhevmChain } from '../utils.js';

export const bscTestnet: FhevmChain = /*#__PURE__*/ defineFhevmChain({
  id: 97,
  fhevm: {
    contracts: {
      acl: {
        address: '0x9Adac873Ae2f737AF011f0778305Cff0B220ac36',
      },
      inputVerifier: {
        address: '0xDd378841F0125656b97818d9d371e1027Df4c424',
      },
      kmsVerifier: {
        address: '0x1F7CAD0c05fDED5F988683AdfDB4F242b310cdcB',
      },
      protocolConfig: {
        address: '0xC320dd3d4e15D8E4c20dff42fE09Aa936A88b3ED',
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
