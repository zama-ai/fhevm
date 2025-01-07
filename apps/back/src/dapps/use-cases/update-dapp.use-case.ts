import { Inject, Injectable } from '@nestjs/common'
import type { AppError, UnitOfWork, UseCase } from 'utils'
import { Task } from 'utils'
import { DApp, DAppProps } from '../domain/entities/dapp.js'
import { DAppRepository } from '../domain/repositories/dapp.repository.js'
import { User } from '#users/domain/entities/user.js'
import { forbiddenError } from 'utils/dist/src/app-error.js'
import { UNIT_OF_WORK } from '#constants.js'
import { DAppId } from '../domain/entities/value-objects.js'

interface Input {
  dapp: {
    id: DAppId
  } & Partial<Omit<DAppProps, 'id'>>
  user: User
}

@Injectable()
export class UpdateDapp implements UseCase<Input, DApp> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly dappRepository: DAppRepository,
  ) { }
  execute({ dapp: { id, ...data }, user }: Input): Task<DApp, AppError> {
    return this.uow.exec(
      this.dappRepository
        .findOneByIdAndUserId(id, user.id)
        .mapError<AppError>(err =>
          err._tag === 'NotFoundError' ? forbiddenError() : err,
        )
        .chain(() => this.dappRepository.update(id, data)),
    )
  }
}
