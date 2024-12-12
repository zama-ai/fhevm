import { Injectable } from '@nestjs/common'
import type { AppError, UseCase } from 'utils'
import { Task } from 'utils'
import { DApp } from '../domain/entities/dapp'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { User } from '@/users/domain/entities/user'

interface Input {
  id: string
  name?: string
  address?: string
}

@Injectable()
export class UpdateDapp implements UseCase<Input, DApp> {
  constructor(
    private readonly dappRepository: DAppRepository,
    private readonly teamRepository: TeamRepository,
  ) {}
  execute(input: Input, ctx: { user: User }): Task<DApp, AppError> {
    return this.dappRepository
      .findOneByIdAndUserId(input.id, String(ctx.user.id))
      .chain(dapp =>
        this.dappRepository.update(
          DApp.parse({
            ...dapp.toJSON(),
            ...(input.name ? { name: input.name } : {}),
            ...(input.address ? { address: input.address } : {}),
          }).unwrap(),
        ),
      )
  }
}
