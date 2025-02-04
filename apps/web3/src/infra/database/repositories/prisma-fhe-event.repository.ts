import { FheEventRepository } from '#domain/services/fhe-event.repository.js'
import { Injectable, Logger } from '@nestjs/common'
import { Task, AppError, unknownError } from 'utils'
import { PrismaService } from '../prisma.service.js'
import { FheEvent } from '#domain/entities/fhe-event.js'
import { ChainId } from '#domain/entities/value-objects.js'

@Injectable()
export class PrismaFheEventRepository implements FheEventRepository {
  logger = new Logger(PrismaFheEventRepository.name)

  constructor(private readonly db: PrismaService) {
    this.logger.debug('new', db)
  }

  getLastBlockNumber = (chainId: ChainId): Task<number, AppError> => {
    this.logger.debug(`getting last block for chain ${chainId.value}`)
    return new Task<number, AppError>((resolve, reject) => {
      this.db.fheEvent
        .aggregate({
          _max: { blockNumber: true },
          where: { chainId: chainId.value },
        })
        .then(value => {
          this.logger.debug(`found ${value._max.blockNumber}`)
          return value
        })
        .then(value => resolve(value._max.blockNumber ?? 0))
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
  }

  create = (data: FheEvent): Task<FheEvent, AppError> => {
    this.logger.log(`creating fhe event ${JSON.stringify(data)}`)
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.fheEvent
        .upsert({
          create: {
            chainId: data.chainId.value,
            id: data.id.value,
            name: data.name,
            callerAddress: data.callerAddress.value,
            blockNumber: data.blockNumber,
            args: data.args,
            timestamp: data.timestamp,
          },
          update: {},
          where: {
            id: data.id.value,
          },
        })
        .then(resolve)
        .catch((err: unknown) => {
          this.logger.warn(`Failed to store Fhe Event: ${err}`)
          reject(unknownError(String(err)))
        })
    }).chain(props => FheEvent.parse(props).async())
  }
}
