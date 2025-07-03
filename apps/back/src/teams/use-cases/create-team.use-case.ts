import { Team } from '#teams/domain/entities/team.js'
import {
  TEAM_REPOSITORY,
  TeamRepository,
} from '#teams/domain/repositories/team.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

type CreateTeamInput = { name: string }
type CreateTeamOutput = Team

@Injectable()
export class CreateTeam implements UseCase<CreateTeamInput, CreateTeamOutput> {
  private readonly logger = new Logger(CreateTeam.name)
  constructor(@Inject(TEAM_REPOSITORY) private readonly repo: TeamRepository) {}

  execute = (
    input: CreateTeamInput,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<Team, AppError> => {
    this.logger.debug(`creating team ${input.name}`)

    return Team.create(input).asyncChain(this.repo.create)
  }
}
