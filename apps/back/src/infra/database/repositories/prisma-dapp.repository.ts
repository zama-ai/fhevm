import { Injectable, Logger } from '@nestjs/common'
import type { AppError } from 'utils'
import { notFoundError, Task, unknownError } from 'utils'

import { DApp, DAppProps } from '@/dapps/domain/entities/dapp'
import { DAppRepository } from '@/dapps/domain/repositories/dapp.repository'

import { PrismaService } from '../prisma.service'
import { DAppId } from '@/dapps/domain/entities/value-objects'
import { UserId } from '@/users/domain/entities/value-objects'

@Injectable()
export class PrismaDAppRepository extends DAppRepository {
  logger = new Logger(PrismaDAppRepository.name)
  constructor(private readonly db: PrismaService) {
    super()
  }

  create = (data: DApp): Task<DApp, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .create({ data: data.toJSON() })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }

  update = ({
    id,
    ...data
  }: { id: DAppId } & Partial<Omit<DAppProps, 'id'>>): Task<DApp, AppError> => {
    this.logger.debug(`update: ${id} ${data}`)
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .update({ where: { id: id.value }, data })
        .then(data => {
          this.logger.verbose(`updated: ${JSON.stringify(data)}`)
          resolve(data)
        })
        .catch(err => {
          this.logger.warn(`failed: ${err}`)
          reject(unknownError(String(err)))
        })
    }).chain(props => DApp.parse(props).async())
  }

  findById = (id: DAppId): Task<DApp, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .findUnique({ where: { id: id.value } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('DApp not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }

  findOneByIdAndUserId = (id: DAppId, userId: UserId): Task<DApp, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .findUnique({
          where: {
            id: id.value,
            team: {
              users: {
                some: { id: { equals: userId.value } },
              },
            },
          },
        })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('DApp not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }
  findAllByTeamId = (teamId: string): Task<DApp[], AppError> => {
    return new Task<unknown[], AppError>((resolve, reject) => {
      this.db.dapp
        .findMany({
          where: { teamId },
        })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(dapps => DApp.parseArray(dapps).async())
  }
}
