import { Inject, Injectable } from '@nestjs/common'
import { every, Task } from 'utils'
import type { AppError, UseCase } from 'utils'

import {
  TEAM_REPOSITORY,
  TeamRepository,
} from '../domain/repositories/team.repository.js'
import { Team } from '../domain/entities/team.js'
import { TeamId } from '../domain/entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'

type Input = { id: string; userId: string }
@Injectable()
export class GetTeamByIdAndUser implements UseCase<Input, Team> {
  constructor(
    @Inject(TEAM_REPOSITORY) private readonly teamRepository: TeamRepository,
  ) {}

  execute = (
    { id, userId }: Input,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<Team, AppError> => {
    return every([TeamId.from(id), UserId.from(userId)]).asyncChain(
      ([teamId, userId]) =>
        this.teamRepository.findOneByIdAndUserId(teamId, userId),
    )
  }
}
