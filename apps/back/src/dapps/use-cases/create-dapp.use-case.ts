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
  dapp: {
    teamId: string
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
      .chain(() =>
        DApp.parse({
          id: randomUUID(),
          name: input.dapp.name,
          status: 'DRAFT',
          teamId: input.dapp.teamId,
          address: input.dapp.address,
        }).asyncChain(this.dappRepository.create),
      )
  }
}
