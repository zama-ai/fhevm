import { DApp } from '../entities/dapp.js'
import type { AppError } from 'utils'
import { Task } from 'utils'
import { DAppId } from '../entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'

export abstract class DAppRepository {
  abstract create(data: DApp): Task<DApp, AppError>
  abstract update(data: DApp): Task<DApp, AppError>
  abstract findById(id: DAppId): Task<DApp, AppError>
  abstract findOneByIdAndUserId(
    id: DAppId,
    userId: UserId,
  ): Task<DApp, AppError>
  abstract findAllByTeamId(teamId: string): Task<DApp[], AppError>
}
