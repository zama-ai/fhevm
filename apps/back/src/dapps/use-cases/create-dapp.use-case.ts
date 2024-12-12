import { Injectable } from '@nestjs/common'
import { randomUUID } from 'crypto'
import type { UseCase, AppError } from 'utils'
import { Task } from 'utils'

import { DApp } from '../domain/entities/dapp'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { User } from '@/users/domain/entities/user'
import { TeamId } from '@/users/domain/entities/value-objects'

interface Input {
  teamId: string
  name: string
  address?: string
}

@Injectable()
export class CreateDapp implements UseCase<Input, DApp> {
  constructor(
    private readonly dappRepository: DAppRepository,
    private readonly teamRepository: TeamRepository,
  ) {}
  execute(input: Input, ctx: { user: User }): Task<DApp, AppError> {
    return this.teamRepository
      .findOneByIdAndUserId(new TeamId(input.teamId), ctx.user.id) // this can throw with a "Team not found" error, it should throw an unthorized error
      .chain(() =>
        this.dappRepository.create(
          DApp.parse({
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
