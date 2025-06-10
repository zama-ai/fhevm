import { ChainId, Web3Address } from '#domain/entities/value-objects.js'
import { Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { z } from 'zod'

const schema = z.object({
  chainId: ChainId.schema,
  contractAddress: Web3Address.schema,
  providerUrl: z.string(),
})

export type FheConfig = z.infer<typeof schema>

export class FheConfigFactory {
  private static logger = new Logger(FheConfigFactory.name)
  static getFheConfig(config: ConfigService): FheConfig[] {
    const configs = config.get<unknown[]>('fhe', [])
    if (!Array.isArray(configs)) {
      this.logger.warn(
        `fhe config is not an array, config: ${JSON.stringify(configs)}`,
      )
      return []
    }

    return configs
      .map(c => schema.safeParse(c))
      .filter(c => c.success)
      .map(c => c.data)
  }
}
