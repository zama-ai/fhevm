import { Chain } from '#chains/domain/entities/chain.js'
import { ChainId } from '#chains/domain/entities/value-objects.js'
import { ChainsRepository } from '#chains/domain/repositories/chains.repository.js'
import { PrismaService } from '#infra/database/prisma.service.js'
import { Injectable, Logger } from '@nestjs/common'
import { Task, AppError, every, notFoundError } from 'utils'

@Injectable()
export class PrismaChainsRepository implements ChainsRepository {
  private readonly logger = new Logger(PrismaChainsRepository.name)

  constructor(private readonly db: PrismaService) {}
  getChainById = (id: ChainId): Task<Chain, AppError> => {
    this.logger.verbose(`getting chain ${id.value}`)
    return Task.fromPromise<unknown, AppError>(
      this.db.chain.findUnique({ where: { id: id.value, enabled: true } }),
    ).chain(chain =>
      chain
        ? Chain.parse(chain).async()
        : Task.reject(notFoundError('Chain not found')),
    )
  }

  getChains = (): Task<Chain[], AppError> => {
    this.logger.verbose('getting all chains')
    return Task.fromPromise<unknown[], AppError>(
      this.db.chain.findMany({ where: { enabled: true } }),
    ).chain(chains => every(chains.map(Chain.parse)).async())
  }
}
