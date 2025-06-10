import { ChainId } from '#domain/entities/value-objects.js'
import { Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { z } from 'zod'

export type EtherProvider = 'Etherscan' | 'Ethers'

const schema = z.object({
  chainId: ChainId.schema,
  provider: z.enum(['Etherscan', 'Ethers']),
  apiEndpoint: z.string().url(),
  rpcEndpoint: z.string().url(),
  apiKey: z.string().optional(),
})

export type EtherConfig = z.infer<typeof schema>

export class EtherConfigFactory {
  private static logger = new Logger(EtherConfigFactory.name)

  static getEtherConfig(config: ConfigService): EtherConfig[] {
    const configs = config.get<unknown[]>('ethers', [])

    if (!Array.isArray(configs)) {
      this.logger.warn(
        `ethers config is not an array, config: ${JSON.stringify(configs)}`,
      )
      return []
    }

    return configs
      .map(c => schema.safeParse(c))
      .filter(c => c.success)
      .map(c => c.data)
  }
}
