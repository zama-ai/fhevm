import { DApp, DAppProps } from '../entities/dapp.js'
import type { AppError } from 'utils'
import { Task } from 'utils'
import { DAppId } from '../entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { DAppStat, DAppStatProps } from '../entities/dapp-stat.js'

export abstract class DAppRepository {
  abstract create(data: DApp): Task<DApp, AppError>
  abstract update(
    id: DAppId,
    data: Partial<Omit<DAppProps, 'id'>>,
  ): Task<DApp, AppError>

  abstract delete(id: DAppId): Task<void, AppError>
  abstract findById(id: DAppId): Task<DApp, AppError>
  abstract findByAddress(chainId: string, address: string): Task<DApp, AppError>
  abstract findOneByIdAndUserId(
    id: DAppId,
    userId: UserId,
  ): Task<DApp, AppError>
  abstract findAllByTeamId(teamId: string): Task<DApp[], AppError>

  abstract createStat(
    id: DAppId,
    props: DAppStatProps,
  ): Task<DAppStat, AppError>
  abstract findAllStats(id: DAppId): Task<DAppStat[], AppError>
}
