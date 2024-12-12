import { DApp } from '../entities/dapp'
import { AppError } from '@/utils/app-error'
import { Task } from '@/utils/task'

export abstract class DAppRepository {
  abstract create(data: DApp): Task<DApp, AppError>
  abstract update(data: DApp): Task<DApp, AppError>
  abstract findOneByIdAndUserId(
    id: string,
    userId: string,
  ): Task<DApp, AppError>
}
