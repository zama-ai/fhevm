import { Inject, Injectable } from '@nestjs/common'
import type { UseCase, AppError, UnitOfWork } from 'utils'
import { Task } from 'utils'

import { DApp, DAppProps } from '../domain/entities/dapp.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '../domain/repositories/dapp.repository.js'
import { TeamRepository } from '#users/domain/repositories/team.repository.js'
import { User } from '#users/domain/entities/user.js'
import { TeamId } from '#users/domain/entities/value-objects.js'
import { UNIT_OF_WORK } from '#constants.js'

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
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(DAPP_REPOSITORY) private readonly dappRepository: DAppRepository,
    private readonly teamRepository: TeamRepository,
  ) {}
  execute = (input: Input): Task<DAppProps, AppError> => {
    return this.uow.exec(
      TeamId.from(input.dapp.teamId)
        .asyncChain(teamId =>
          this.teamRepository.findOneByIdAndUserId(teamId, input.user.id),
        ) // this can throw with a "Team not found" error, it should throw an unthorized error
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
        ),
    )
  }
}
