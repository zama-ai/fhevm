import { Inject, Injectable } from '@nestjs/common'
import { type TeamProps } from '#teams/domain/entities/team.js'
import type { AppError, Task, UseCase } from 'utils'
import {
  TEAM_REPOSITORY,
  TeamRepository,
} from '../domain/repositories/team.repository.js'
import { UserId } from '#users/domain/entities/value-objects.js'

@Injectable()
export class GetTeamsByUserId implements UseCase<UserId, TeamProps[]> {
  constructor(
    @Inject(TEAM_REPOSITORY) private readonly teamRepository: TeamRepository,
  ) {}

  execute = (userId: UserId): Task<TeamProps[], AppError> => {
    return this.teamRepository
      .findManyByUserId(userId)
      .map(teams => teams.map(team => team.toJSON()))
  }
}
