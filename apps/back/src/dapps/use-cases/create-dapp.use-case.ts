import { Inject, Injectable } from '@nestjs/common'
import type { UseCase, AppError } from 'utils'
import { Task } from 'utils'

import { DApp, DAppProps } from '../domain/entities/dapp.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '../domain/repositories/dapp.repository.js'
import { User } from '#users/domain/entities/user.js'
import { GetTeamByIdAndUser } from '#teams/use-cases/get-team-by-id-and-user.use-case.js'

interface Input {
  dapp: {
    teamId: string
    name: string
    chainId?: number
    address?: string
  }
  user: User
}

@Injectable()
export class CreateDapp implements UseCase<Input, DAppProps> {
  constructor(
    @Inject(DAPP_REPOSITORY) private readonly dappRepository: DAppRepository,
    private readonly getTeamByIdAndUserUC: GetTeamByIdAndUser,
  ) {}
  execute = (input: Input): Task<DAppProps, AppError> => {
    return this.getTeamByIdAndUserUC
      .execute({ id: input.dapp.teamId, userId: input.user.id.value })
      .chain(team =>
        DApp.create({
          name: input.dapp.name,
          teamId: team.id.value,
          // TODO: I need to check chainId exists
          chainId: input.dapp.chainId,
          address: input.dapp.address,
        })
          .asyncChain(this.dappRepository.create)
          .map(dapp => dapp.toJSON()),
      )
  }
}
