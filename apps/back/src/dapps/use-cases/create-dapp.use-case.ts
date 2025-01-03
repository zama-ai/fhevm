import { Injectable } from '@nestjs/common'
import type { UseCase, AppError } from 'utils'
import { Task } from 'utils'

import { DApp } from '../domain/entities/dapp'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { User } from '@/users/domain/entities/user'
import { TeamId } from '@/users/domain/entities/value-objects'

interface Input {
  dapp: {
    teamId: `t_${string}`
    name: string
    address?: string
  }
  user: User
}

@Injectable()
export class CreateDapp implements UseCase<Input, DApp> {
  constructor(
    private readonly dappRepository: DAppRepository,
    private readonly teamRepository: TeamRepository,
  ) {}
  execute(input: Input): Task<DApp, AppError> {
    return this.teamRepository
      .findOneByIdAndUserId(new TeamId(input.dapp.teamId), input.user.id) // this can throw with a "Team not found" error, it should throw an unthorized error
      .chain(team =>
        DApp.create({
          name: input.dapp.name,
          teamId: team.id.value,
          address: input.dapp.address,
        }).asyncChain(this.dappRepository.create),
      )
  }
}
