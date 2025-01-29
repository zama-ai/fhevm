import { ChainId, Web3Address } from '#domain/entities/value-objects.js'
import { LOCAL_CHAIN_ID } from '#src/constants.js'
import { registerAs } from '@nestjs/config'

export default registerAs('fhe', () => {
  return {
    chainIds: process.env.FHE_CHAIN_IDS?.split(',').map(id => id.trim()) ?? [],
  }
})

export interface FheConfig {
  chainId: ChainId
  contractAddress: Web3Address
  providerUrl: string
}

const configs: Record<string, () => FheConfig> = {
  [LOCAL_CHAIN_ID]: () => ({
    chainId: ChainId.fromString(LOCAL_CHAIN_ID).unwrap(),
    contractAddress: Web3Address.fromString(
      '0x596E6682c72946AF006B27C131793F2b62527A4b',
    ).unwrap(),
    providerUrl: 'ws://localhost:8746',
  }),
}

export class FheConfigFactory {
  static getFheConfig(chainId: ChainId): FheConfig | null {
    const config = configs[chainId.value]
    return config ? config() : null
  }
}
