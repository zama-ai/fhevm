import { Inject, Injectable } from '@nestjs/common'
import { Task } from 'utils'
import type { AppError, UseCase } from 'utils'

import {
  TEAM_REPOSITORY,
  TeamRepository,
} from '../domain/repositories/team.repository.js'
import { Team } from '../domain/entities/team.js'
import { TeamId } from '../domain/entities/value-objects.js'

@Injectable()
export class GetTeamById implements UseCase<string, Team> {
  constructor(
    @Inject(TEAM_REPOSITORY) private readonly teamRepository: TeamRepository,
  ) {}

  execute = (id: string): Task<Team, AppError> => {
    return TeamId.from(id).asyncChain(this.teamRepository.findOneById)
  }
}
