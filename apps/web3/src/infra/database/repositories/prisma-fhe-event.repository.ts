import { FheEventRepository } from '#domain/services/fhe-event.repository.js'
import { Logger } from '@nestjs/common'
import { Task, AppError, unknownError } from 'utils'
import { PrismaService } from '../prisma.service.js'
import { FheEvent } from '#domain/entities/fhe-event.js'
import { ChainId } from '#domain/entities/value-objects.js'

export class PrismaFheEventRepository implements FheEventRepository {
  logger = new Logger(PrismaFheEventRepository.name)

  constructor(private readonly db: PrismaService) {}

  getLastBlockNumber = (chainId: ChainId): Task<number, AppError> => {
    return new Task<number, AppError>((resolve, reject) => {
      this.db.fheEvent
        .aggregate({
          _max: { blockNumber: true },
          where: { chainId: chainId.value },
        })
        .then(value => resolve(value._max.blockNumber ?? 0))
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
  }

  create = (data: FheEvent): Task<FheEvent, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.fheEvent
        .create({
          data: {
            chainId: data.chainId.value,
            id: data.id.value,
            name: data.name,
            callerAddress: data.callerAddress.value,
            blockNumber: data.blockNumber,
            args: data.args,
            timestamp: data.timestamp,
          },
        })
        .then(resolve)
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => FheEvent.parse(props).async())
  }
}
