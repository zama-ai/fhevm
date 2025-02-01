import { Injectable, Logger } from '@nestjs/common'
import type { AppError } from 'utils'
import { every, notFoundError, Task, unknownError } from 'utils'

import { DApp, DAppProps } from '#dapps/domain/entities/dapp.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'

import { PrismaService } from '../prisma.service.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { DAppStat } from '#dapps/domain/entities/dapp-stat.js'

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
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }

  update = (
    id: DAppId,
    data: Partial<Omit<DAppProps, 'id'>>,
  ): Task<DApp, AppError> => {
    this.logger.debug(`update: ${id} ${JSON.stringify(data)}`)
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .update({ where: { id: id.value }, data })
        .then(data => {
          this.logger.verbose(`updated: ${JSON.stringify(data)}`)
          resolve(data)
        })
        .catch((err: unknown) => {
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
        .catch((err: unknown) => reject(unknownError(String(err))))
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
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }
  findAllByTeamId = (teamId: string): Task<DApp[], AppError> => {
    return new Task<unknown[], AppError>((resolve, reject) => {
      this.db.dapp
        .findMany({
          where: { teamId },
          orderBy: { createdAt: 'desc' },
        })
        .then(resolve)
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(dapps => every(dapps.map(DApp.parse)).async())
  }

  findAllStats = (id: DAppId): Task<DAppStat[], AppError> => {
    return new Task<unknown[], AppError>((resolve, reject) => {
      this.db.dappStat
        .findMany({ where: { dappId: id.value } })
        .then(resolve)
        .catch((err: unknown) => {
          this.logger.warn(`failed to run findAllStats for ${id.value}: ${err}`)
          return reject(unknownError(String(err)))
        })
    }).chain(dappStats => every(dappStats.map(DAppStat.parse)).async())
  }
}
