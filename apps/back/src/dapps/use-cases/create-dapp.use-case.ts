import { Injectable } from '@nestjs/common'
import type { UseCase, AppError } from 'utils'
import { Task } from 'utils'

import { DApp } from '../domain/entities/dapp.js'
import { DAppRepository } from '../domain/repositories/dapp.repository.js'
import { TeamRepository } from '#users/domain/repositories/team.repository.js'
import { type UserProps } from '#users/domain/entities/user.js'
import { TeamId, UserId } from '#users/domain/entities/value-objects.js'

interface Input {
  dapp: {
    teamId: `team_${string}`
    name: string
    address?: string
  }
  user: UserProps
}

@Injectable()
export class CreateDapp implements UseCase<Input, DApp> {
  constructor(
    private readonly dappRepository: DAppRepository,
    private readonly teamRepository: TeamRepository,
  ) {}
  execute(input: Input): Task<DApp, AppError> {
    return this.teamRepository
      .findOneByIdAndUserId(
        TeamId.from(input.dapp.teamId),
        UserId.from(input.user.id),
      ) // this can throw with a "Team not found" error, it should throw an unthorized error
      .chain(team =>
        DApp.create({
          name: input.dapp.name,
          teamId: team.id.value,
          address: input.dapp.address,
        }).asyncChain(this.dappRepository.create),
      )
  }
}
