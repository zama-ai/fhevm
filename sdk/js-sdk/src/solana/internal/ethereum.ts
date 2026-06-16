import type { EthereumModuleFactory } from '../../core/modules/ethereum/types.js';

////////////////////////////////////////////////////////////////////////////////
// solanaEthereumModule
//
// The Solana host has no EVM ABI/RPC surface. The runtime requires an EthereumModule slot,
// but the Solana user-decrypt path never touches it (no on-chain ACL/KMSVerifier reads, no
// EIP-712 signing — the request is ed25519 and the KMS context is static). Every method
// therefore throws: reaching one means a non-Solana code path leaked onto a Solana client.
////////////////////////////////////////////////////////////////////////////////

function notSupported(name: string): never {
  throw new Error(`EthereumModule.${name} is not available on a Solana client`);
}

export const solanaEthereumModule: EthereumModuleFactory = () => {
  return Object.freeze({
    ethereum: Object.freeze({
      recoverTypedDataAddress: () => notSupported('recoverTypedDataAddress'),
      signTypedData: () => notSupported('signTypedData'),
      readContract: () => notSupported('readContract'),
      encode: () => notSupported('encode'),
      encodePacked: () => notSupported('encodePacked'),
      decode: () => notSupported('decode'),
      getChainId: () => notSupported('getChainId'),
    }),
  });
};
