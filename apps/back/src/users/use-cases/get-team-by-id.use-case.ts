import { Injectable } from '@nestjs/common'
import { Task } from 'utils'
import type { AppError, UseCase } from 'utils'

import { Team } from '#users/domain/entities/team.js'
import { TeamRepository } from '../domain/repositories/team.repository.js'
import { TeamId } from '../domain/entities/value-objects.js'

@Injectable()
export class GetTeamById implements UseCase<TeamId, Team> {
  constructor(private readonly teamRepository: TeamRepository) {}

  execute(id: TeamId): Task<Team, AppError> {
    return this.teamRepository.findOneById(id)
  }
}
