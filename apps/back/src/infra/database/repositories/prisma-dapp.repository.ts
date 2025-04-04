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
  type DailyStats,
} from '#dapps/domain/repositories/dapp.repository.js'

import { PrismaService } from '../prisma.service.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { DAppStat, DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { Computation } from '#dapps/domain/utilities/computation.js'
import { StatsType } from '#prisma/client/index.js'

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

  findDailyStats = (id: DAppId): Task<DailyStats, AppError> => {
    const LIMIT_BYDAY_AGO = 30 // default 30 days

    return new Task<DailyStats, AppError>((resolve, reject) => {
      const daysago = new Date()
      daysago.setUTCHours(0, 0, 0, 0) // full days in UTC, partial day stats are ugly
      daysago.setUTCDate(daysago.getUTCDate() - LIMIT_BYDAY_AGO)

      this.db.dappStat
        .groupBy({
          by: ['type', 'day', 'year'],
          where: {
            dappId: id.value,
            timestamp: {
              gte: daysago.toISOString(),
            },
          },
          _count: {
            _all: true,
          },
        })
        .then(stats => {
          const dailyStatsMap = new Map<string, DailyStats[0]>()

          stats.forEach(stat => {
            const date = new Date(stat.year, 0, 1)
            // reminder: stat.day is 1-366
            date.setUTCDate(date.getUTCDate() + stat.day - 1)
            const formattedDay = date.toISOString().split('T')[0]
            const dayId = `day_${formattedDay.replace(/-/g, '')}`

            if (!dailyStatsMap.has(dayId)) {
              dailyStatsMap.set(dayId, {
                id: dayId,
                day: formattedDay,
                total: 0,
                computation: 0,
                encryption: 0,
              })
            }

            const dayStats = dailyStatsMap.get(dayId)!
            if (stat.type === StatsType.COMPUTATION) {
              dayStats.computation = stat._count._all
            } else {
              dayStats.encryption = stat._count._all
            }
            dayStats.total += stat._count._all
          })

          resolve(Array.from(dailyStatsMap.values()))
        })
        .catch((err: unknown) => {
          this.logger.warn(
            `failed to run findDailyStats for ${id.value}: ${err}`,
          )
          reject(unknownError(String(err)))
        })
    })
  }
}
