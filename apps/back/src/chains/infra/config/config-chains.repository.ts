import { Chain } from '#chains/domain/entities/chain.js'
import { ChainId } from '#chains/domain/entities/value-objects.js'
import { ChainsRepository } from '#chains/domain/repositories/chains.repository.js'
import { Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { Task, AppError, every, notFoundError } from 'utils'

@Injectable()
export class ConfigChainsRepository implements ChainsRepository {
  private readonly logger = new Logger(ConfigChainsRepository.name)
  private _chains: Chain[]
  constructor(private readonly config: ConfigService) {}

  private get chains() {
    if (!this._chains) {
      const chains: unknown[] = this.config.get('chains') ?? []
      this.logger.verbose(`chains from config: ${JSON.stringify(chains)}`)
      this._chains = chains
        .map(Chain.parse)
        .filter(c => c.isOk())
        .map(c => c.unwrap())
    }
    return this._chains
  }

  getChainById = (id: ChainId): Task<Chain, AppError> => {
    this.logger.verbose(`getting chain ${id.value}`)
    const chain = this.chains.find(c => c.id.equals(id))
    return chain
      ? Task.of(chain)
      : Task.reject(notFoundError('Chain not found'))
  }

  getChains = (): Task<Chain[], AppError> => {
    this.logger.verbose('getting all chains')
    return Task.of<Chain[], AppError>(this.chains)
  }
}
