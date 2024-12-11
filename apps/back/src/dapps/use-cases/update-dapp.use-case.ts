import { Injectable } from '@nestjs/common'

import { Dapp } from '../domain/entities/dapp'
import { UseCase } from '@/utils/use-case'
import { Task } from '@/utils/task'
import { AppError } from '@/utils/app-error'
import { DappRepository } from '../domain/repositories/dapp.repository'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { User } from '@/users/domain/entities/user'

interface Input {
  id: string
  name?: string
  address?: string
}

@Injectable()
export class UpdateDapp implements UseCase<Input, Dapp> {
  constructor(
    private readonly dappRepository: DappRepository,
    private readonly teamRepository: TeamRepository,
  ) {}
  execute(input: Input, ctx: { user: User }): Task<Dapp, AppError> {
    return this.dappRepository
      .findOneByIdAndUserId(input.id, String(ctx.user.id))
      .chain(dapp =>
        this.dappRepository.update(
          Dapp.parse({
            ...dapp.toJSON(),
            ...(input.name ? { name: input.name } : {}),
            ...(input.address ? { address: input.address } : {}),
          }).unwrap(),
        ),
      )
  }
}
