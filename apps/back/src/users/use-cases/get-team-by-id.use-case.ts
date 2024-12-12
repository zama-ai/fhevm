import { Injectable } from '@nestjs/common'
import { Task } from 'utils'
import type { AppError, UseCase } from 'utils'

import { Team } from '@/users/domain/entities/team'
import { TeamRepository } from '../domain/repositories/team.repository'
import { TeamId } from '../domain/entities/value-objects'

@Injectable()
export class GetTeamById implements UseCase<TeamId, Team> {
  constructor(private readonly teamRepository: TeamRepository) {}

  execute(id: TeamId): Task<Team, AppError> {
    return this.teamRepository.findOneById(id)
  }
}
