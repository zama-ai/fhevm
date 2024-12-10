import { registerAs } from '@nestjs/config'

export type EtherProvider = 'Etherscan'
export const SEPOLIA_CHAIN_ID = '11155111'
export type ChainId = typeof SEPOLIA_CHAIN_ID

export interface EtherConfig {
  chainId: ChainId
  provider: EtherProvider
  apiEndpoint: string
  rpcEndpoint: string
  apiKey?: string
}

const configs: Record<
  ChainId,
  Omit<EtherConfig, 'chainId' | 'apiKey'> & { apiKey: () => string | undefined }
> = {
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

export function isChainId(chainId: string): chainId is ChainId {
  return [SEPOLIA_CHAIN_ID].includes(chainId)
}

export class EtherConfigFactory {
  static getEtherConfig(chainId: ChainId): EtherConfig {
    const config = configs[chainId]
    return {
      ...config,
      chainId,
      apiKey: config.apiKey(),
    }
  }
}
