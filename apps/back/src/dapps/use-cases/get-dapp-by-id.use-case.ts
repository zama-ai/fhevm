import { UNIT_OF_WORK } from '#constants.js'
import { Inject } from '@nestjs/common'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '../domain/repositories/dapp.repository.js'
import { DAppId } from '../domain/entities/value-objects.js'
import type { DAppProps } from '../domain/entities/dapp.js'
import { UserId } from '#users/domain/entities/value-objects.js'

// TODO: move user to context
// TODO: change `dappId` to string
type Input = { dappId: DAppId; userId: UserId }

export class GetDappById implements UseCase<Input, DAppProps> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(DAPP_REPOSITORY) private readonly repo: DAppRepository,
  ) {}

  /**
   * Returns a DApp by its id.
   *
   * @param input.id - The id of the DApp to retrieve.
   * @param context - Optional context object.
   *
   * @returns A Task that resolves with the DApp if found, otherwise rejects with an AppError.
   */
  execute = ({ dappId, userId }: Input): Task<DAppProps, AppError> => {
    return this.uow
      .exec(this.repo.findOneByIdAndUserId(dappId, userId))
      .map(dapp => dapp.toJSON())
  }
}
