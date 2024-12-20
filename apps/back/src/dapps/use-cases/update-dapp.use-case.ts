import { Inject, Injectable } from '@nestjs/common'
import type { AppError, UnitOfWork, UseCase } from 'utils'
import { Task } from 'utils'
import { DApp, DAppProps } from '../domain/entities/dapp'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { User } from '@/users/domain/entities/user'
import { forbiddenError } from 'utils/dist/app-error'
import { UNIT_OF_WORK } from '@/constants'
import { DAppId } from '../domain/entities/value-objects'

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
  ) {}
  execute(input: Input): Task<DApp, AppError> {
    return this.uow.exec(
      this.dappRepository
        .findOneByIdAndUserId(input.dapp.id, input.user.id)
        .mapError<AppError>(err =>
          err._tag === 'NotFoundError' ? forbiddenError() : err,
        )
        .chain(() => this.dappRepository.update(input.dapp)),
    )
  }
}
