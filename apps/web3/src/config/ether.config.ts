import { ChainId } from '#domain/entities/value-objects.js'
import { registerAs } from '@nestjs/config'
import { LOCAL_FHEVM_CHAIN_ID, SEPOLIA_CHAIN_ID } from 'utils'

export type EtherProvider = 'Etherscan' | 'Ethers'

export interface EtherConfig {
  chainId: ChainId
  provider: EtherProvider
  apiEndpoint: string
  rpcEndpoint: string
  apiKey?: string
}

const configs: Record<
  string,
  Omit<EtherConfig, 'chainId' | 'apiKey'> & { apiKey: () => string | undefined }
> = {
  [LOCAL_FHEVM_CHAIN_ID]: {
    provider: 'Ethers',
    apiEndpoint: 'ws://localhost:8545',
    rpcEndpoint: 'ws://localhost:8545',
    apiKey: () => undefined,
  },
  [SEPOLIA_CHAIN_ID]: {
    provider: 'Etherscan',
    apiEndpoint: 'https://api-sepolia.etherscan.io/api',
    rpcEndpoint: 'https://rpc.sepolia.org',
    apiKey: () => process.env.ETHERSCAN_SEPOLIA_APIKEY,
  },
}

export default registerAs('ether', () => {
  return {
    chainIds:
      process.env.ETHER_CHAIN_IDS?.split(',').map(id => id.trim()) ?? [],
  }
})

export class EtherConfigFactory {
  static getEtherConfig(chainId: number): EtherConfig | null {
    const config = configs[chainId]
    return config
      ? {
          ...config,
          // Note: if I found a config it should be safe to unwrap
          chainId: ChainId.from(chainId).unwrap(),
          apiKey: config.apiKey(),
        }
      : null
  }
}
