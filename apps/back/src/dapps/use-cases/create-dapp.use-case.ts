import { Injectable } from '@nestjs/common'
import { randomUUID } from 'crypto'

import { Dapp } from '../domain/entities/dapp'
import { UseCase } from '@/utils/use-case'
import { Task } from '@/utils/task'
import { AppError } from '@/utils/app-error'
import { DappRepository } from '../domain/repositories/dapp.repository'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { User } from '@/users/domain/entities/user'
import { TeamId } from '@/users/domain/entities/value-objects'

interface Input {
  teamId: string
  name: string
  address?: string
}

@Injectable()
export class CreateDapp implements UseCase<Input, Dapp> {
  constructor(
    private readonly dappRepository: DappRepository,
    private readonly teamRepository: TeamRepository,
  ) {}
  execute(input: Input, ctx: { user: User }): Task<Dapp, AppError> {
    console.log(ctx.user.toJSON())
    return this.teamRepository
      .findOneByIdAndUserId(new TeamId(input.teamId), ctx.user.id) // this can throw with a "Team not found" error, it should throw an unthorized error
      .chain(() =>
        this.dappRepository.create(
          Dapp.parse({
            id: randomUUID(),
            name: input.name,
            status: 'DRAFT',
            teamId: input.teamId,
            address: input.address,
          }).unwrap(),
        ),
      )
  }
}
