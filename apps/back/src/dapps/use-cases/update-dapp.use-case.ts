import { Inject, Injectable } from '@nestjs/common'
import type { AppError, UnitOfWork, UseCase } from 'utils'
import { Task } from 'utils'
import { DApp } from '../domain/entities/dapp'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { User } from '@/users/domain/entities/user'
import { forbiddenError } from 'utils/dist/app-error'
import { UNIT_OF_WORK } from '@/constants'

interface Input {
  dapp: {
    id: string
    name?: string
    address?: string
  }
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
        .findOneByIdAndUserId(input.dapp.id, String(input.user.id))
        .mapError<AppError>(err =>
          err._tag === 'NotFoundError' ? forbiddenError() : err,
        )
        .chain(dapp =>
          DApp.parse(Object.assign({}, dapp.toJSON(), input.dapp)).asyncChain(
            this.dappRepository.update,
          ),
        ),
    )
  }
}
