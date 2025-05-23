import { Team } from '#teams/domain/entities/team.js'
import {
  TEAM_REPOSITORY,
  TeamRepository,
} from '#teams/domain/repositories/team.repository.js'
import { Inject, Injectable } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

@Injectable()
export class CreateTeam implements UseCase<{ name: string }, Team> {
  constructor(@Inject(TEAM_REPOSITORY) private readonly repo: TeamRepository) {}

  execute = (
    input: { name: string },
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<Team, AppError> => {
    return Team.create(input).asyncChain(this.repo.create)
  }
}
