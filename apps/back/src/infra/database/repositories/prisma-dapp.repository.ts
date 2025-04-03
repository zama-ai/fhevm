import { Injectable, Logger } from '@nestjs/common'
import { AppError } from 'utils'
import {
  duplicatedError,
  every,
  notFoundError,
  Task,
  unknownError,
} from 'utils'

import { DApp, DAppProps } from '#dapps/domain/entities/dapp.js'
import {
  DAppRepository,
  type Operation,
  type CumulativeStats,
} from '#dapps/domain/repositories/dapp.repository.js'

import { PrismaService } from '../prisma.service.js'
import {
  ApiKeyId,
  DAppId,
  Token,
} from '#dapps/domain/entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { DAppStat, DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { ApiKey } from '#dapps/domain/entities/api-key.js'
import { Computation } from '#dapps/domain/utilities/computation.js'

@Injectable()
export class PrismaDAppRepository implements DAppRepository {
  logger = new Logger(PrismaDAppRepository.name)
  constructor(private readonly db: PrismaService) {}

  create = (data: DApp): Task<DApp, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .create({ data: data.toJSON() })
        .then(resolve)
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }

  delete = (id: DAppId): Task<void, AppError> => {
    return new Task((resolve, reject) => {
      this.db.dapp
        .findUnique({
          where: { id: id.value, deletedAt: null },
        })
        .then(dapp => {
          if (!dapp) {
            reject(notFoundError('dapp not found'))
          } else {
            return this.db.dapp.update({
              data: { deletedAt: new Date() },
              where: { id: id.value },
            })
          }
        })
        .then(() => {
          resolve(void 0)
        })
        .catch(error => {
          this.logger.warn(`failed to delete dapp ${id.value}: ${error}`)
          reject(unknownError(String(error)))
        })
    })
  }

  update = (
    id: DAppId,
    data: Partial<Omit<DAppProps, 'id'>>,
  ): Task<DApp, AppError> => {
    this.logger.debug(`update: ${id} ${JSON.stringify(data)}`)
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .findUnique({ where: { id: id.value, deletedAt: null } })
        .then(dapp => {
          if (!dapp) {
            reject(notFoundError(`dapp not found`))
          } else {
            return this.db.dapp.update({
              where: { id: id.value },
              data,
            })
          }
        })
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
        .findUnique({ where: { id: id.value, deletedAt: null } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('DApp not found')),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => DApp.parse(props).async())
  }

  findByAddress = (
    chainId: string | number,
    address: string,
  ): Task<DApp, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .findFirst({ where: { address, deletedAt: null } })
        .then(data =>
          data
            ? resolve(data)
            : reject(notFoundError(`No DApp found for ${chainId}/${address}`)),
        )
        .catch((error: unknown) => reject(unknownError(String(error))))
    }).chain(props => DApp.parse(props).async())
  }

  findOneByIdAndUserId = (id: DAppId, userId: UserId): Task<DApp, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.dapp
        .findUnique({
          where: {
            id: id.value,
            deletedAt: null,
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
          where: { teamId, deletedAt: null },
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

  createStat = (id: DAppId, props: DAppStatProps): Task<DAppStat, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.logger.verbose(`searching for existing stat: ${props.externalRef}`)
      this.db.dappStat
        .findFirst({ where: { externalRef: props.externalRef } })
        .then(stat => {
          this.logger.verbose(`stat found: ${JSON.stringify(stat)}`)
          return stat
            ? reject(duplicatedError(`${props.externalRef} already exists`))
            : resolve(null)
        })
        .catch((err: unknown) => reject(unknownError(String(err))))
    })
      .chain(
        () =>
          new Task((resolve, reject) => {
            this.logger.verbose(`creating stat: ${JSON.stringify(props)}`)
            this.db.dappStat
              .create({
                data: {
                  ...props,
                  dappId: id.value,
                },
              })
              .then(resolve)
              .catch((err: unknown) => reject(unknownError(String(err))))
          }),
      )
      .chain(props => {
        this.logger.verbose(`parsing stat: ${JSON.stringify(props)}`)
        return DAppStat.parse(props).async()
      })
  }

  findCumulativeStats = (id: DAppId): Task<CumulativeStats, AppError> => {
    return new Task<CumulativeStats, AppError>((resolve, reject) => {
      this.db.dappStat
        .groupBy({
          by: ['name'],
          where: { dappId: id.value },
          _count: {
            name: true,
          },
        })
        .then(stats => {
          this.logger.debug(`stats: ${JSON.stringify(stats)}`)
          const operations = stats.reduce(
            (acc, stat) => {
              acc[stat.name as Operation] = stat._count.name
              return acc
            },
            {} as Record<Operation, number>,
          )
          const computation = new Computation(operations)
          resolve(computation)
        })
        .catch((err: unknown) => {
          this.logger.warn(
            `failed to run findCumulativeStats for ${id.value}: ${err}`,
          )
          reject(unknownError(String(err)))
        })
    })
  }

  /* Api Keys */
  createApiKey = (apiKey: ApiKey): Task<ApiKey, AppError> => {
    this.logger.verbose(`creating API key for dApp ${apiKey.dappId.value}`)
    return this.findById(apiKey.dappId)
      .chain(() => {
        return new Task<unknown, AppError>((resolve, reject) => {
          this.logger.debug(
            `creating ${apiKey.id.value} for dApp ${apiKey.dappId.value}`,
          )

          this.db.apiKey
            .create({
              data: {
                id: apiKey.id.value,
                token: apiKey.token.value,
                dappId: apiKey.dappId.value,
                name: apiKey.name,
                description: apiKey.description,
              },
            })
            .then(resolve)
            .catch(err => {
              this.logger.warn(`failed to create api key: ${err}`)
              reject(unknownError(String(err)))
            })
        })
      })
      .chain(props => ApiKey.parse(props).async())
  }

  findAllApiKeys = (id: DAppId): Task<ApiKey[], AppError> => {
    this.logger.verbose(`finding all api keys for ${id.value}`)
    return new Task<unknown[], AppError>((resolve, reject) =>
      this.db.apiKey
        .findMany({ where: { dappId: id.value, deletedAt: null } })
        .then(resolve)
        .catch(err => {
          this.logger.warn(
            `failed to run findAllApiKeys for ${id.value}: ${err}`,
          )
          reject(unknownError(String(err)))
        }),
    ).chain(props => every(props.map(ApiKey.parse)).async())
  }

  findApiKey = (id: ApiKeyId): Task<ApiKey, AppError> => {
    this.logger.verbose(`finding api key ${id.value}`)
    return new Task<unknown, AppError>((resolve, reject) =>
      this.db.apiKey
        .findUnique({ where: { id: id.value, deletedAt: null } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('API key not found')),
        )
        .catch(err => {
          this.logger.warn(`failed to run findApiKey for ${id.value}: ${err}`)
          reject(unknownError(String(err)))
        }),
    ).chain(props => ApiKey.parse(props).async())
  }

  findApiKeyByToken = (token: Token): Task<ApiKey, AppError> => {
    this.logger.verbose(`finding api key by token ${token.value}`)
    return new Task<unknown, AppError>((resolve, reject) =>
      this.db.apiKey
        .findFirst({ where: { token: token.value, deletedAt: null } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('API key not found')),
        )
        .catch(err => {
          this.logger.warn(
            `failed to run findApiKeyByToken for ${token.value}: ${err}`,
          )
          reject(unknownError(String(err)))
        }),
    ).chain(props => ApiKey.parse(props).async())
  }

  updateApiKey = (apiKey: ApiKey): Task<ApiKey, AppError> => {
    this.logger.verbose(`updating api key ${apiKey.id.value}`)
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.apiKey
        .update({
          where: { id: apiKey.id.value },
          data: {
            name: apiKey.name,
            description: apiKey.description,
          },
        })
        .then(resolve)
        .catch(err => {
          this.logger.warn(`failed to update api key: ${err}`)
          reject(unknownError(String(err)))
        })
    }).chain(props => ApiKey.parse(props).async())
  }

  deleteApiKey = (id: ApiKeyId): Task<void, AppError> => {
    this.logger.verbose(`deleting api key ${id.value}`)
    return new Task<unknown, AppError>((resolve, reject) =>
      this.db.apiKey
        .update({ where: { id: id.value }, data: { deletedAt: new Date() } })
        .then(resolve)
        .catch(err => {
          this.logger.warn(`failed to run deleteApiKey for ${id.value}: ${err}`)
          reject(unknownError(String(err)))
        }),
    ).map(() => void 0)
  }
}
