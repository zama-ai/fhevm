import { DApp } from '../entities/dapp'
import type { AppError } from 'utils'
import { Task } from 'utils'

export abstract class DAppRepository {
  abstract create(data: DApp): Task<DApp, AppError>
  abstract update(data: DApp): Task<DApp, AppError>
  abstract findOneByIdAndUserId(
    id: string,
    userId: string,
  ): Task<DApp, AppError>
  abstract findAllByTeamId(teamId: string): Task<DApp[], AppError>
}
