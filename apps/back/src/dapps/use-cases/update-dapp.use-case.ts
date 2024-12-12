import { Injectable } from '@nestjs/common'
import type { AppError, UseCase } from 'utils'
import { Task } from 'utils'
import { DApp } from '../domain/entities/dapp'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { User } from '@/users/domain/entities/user'

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
  constructor(private readonly dappRepository: DAppRepository) {}
  execute(input: Input): Task<DApp, AppError> {
    return this.dappRepository
      .findOneByIdAndUserId(input.dapp.id, String(input.user.id))
      .chain(dapp =>
        DApp.parse(Object.assign({}, dapp.toJSON(), input.dapp)).asyncChain(
          this.dappRepository.update,
        ),
      )
  }
}
