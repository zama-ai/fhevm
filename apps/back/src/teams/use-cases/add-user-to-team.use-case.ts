import { Team } from '#teams/domain/entities/team.js'
import { TeamId } from '#teams/domain/entities/value-objects.js'
import {
  TEAM_REPOSITORY,
  TeamRepository,
} from '#teams/domain/repositories/team.repository.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { Inject, Injectable } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

type Input = {
  userId: UserId
  teamId: TeamId
}

@Injectable()
export class AddUserToTeam implements UseCase<Input, Team> {
  constructor(@Inject(TEAM_REPOSITORY) private readonly repo: TeamRepository) {}
  execute = (
    input: Input,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<Team, AppError> => {
    return this.repo.addUser(input.teamId, input.userId)
  }
}
