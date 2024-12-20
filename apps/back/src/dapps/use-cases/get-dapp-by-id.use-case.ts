import { UNIT_OF_WORK } from '@/constants'
import { Inject } from '@nestjs/common'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { DAppId } from '../domain/entities/value-objects'
import { DApp } from '../domain/entities/dapp'

export class GetDappById implements UseCase<DAppId, DApp> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly repo: DAppRepository,
  ) {}

  /**
   * Returns a DApp by its id.
   *
   * @param input.id - The id of the DApp to retrieve.
   * @param context - Optional context object.
   *
   * @returns A Task that resolves with the DApp if found, otherwise rejects with an AppError.
   */
  execute(id: DAppId): Task<DApp, AppError> {
    return this.uow.exec(this.repo.findById(id))
  }
}
