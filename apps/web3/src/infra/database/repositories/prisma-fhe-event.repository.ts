import { FheEventRepository } from '#src/domain/services/fhe-event.repository.js'
import { Logger } from '@nestjs/common'
import { Task, AppError, unknownError } from 'utils'
import { PrismaService } from '../prisma.service.js'
import { FheEvent } from '#src/domain/entities/fhe-event.js'

export class PrismaFheEventRepository implements FheEventRepository {
  logger = new Logger(PrismaFheEventRepository.name)

  constructor(private readonly db: PrismaService) {}

  getLastBlockNumber = (chainId: string): Task<number, AppError> => {
    return new Task<number, AppError>((resolve, reject) => {
      this.db.fheEvent
        .aggregate({
          _max: { blockNumber: true },
          where: { chainId },
        })
        .then(value => resolve(value._max.blockNumber ?? 0))
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
  }

  create = (data: FheEvent): Task<FheEvent, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.fheEvent
        .create({
          data: data.toJSON(),
        })
        .then(resolve)
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => FheEvent.parse(props).async())
  }
}
