import { SEPOLIA_CHAIN_ID } from '#constants.js'
import { ChainId } from '#src/domain/entities/value-objects.js'
import { registerAs } from '@nestjs/config'

export type EtherProvider = 'Etherscan'

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
  static getEtherConfig(chainId: string): EtherConfig | null {
    const config = configs[chainId]
    return config
      ? {
          ...config,
          // Note: if I found a config it should be safe to unwrap
          chainId: ChainId.fromString(chainId).unwrap(),
          apiKey: config.apiKey(),
        }
      : null
  }
}
