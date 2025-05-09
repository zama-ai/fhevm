import { Injectable } from '@nestjs/common'
import { Task } from 'utils'
import type { AppError, UseCase } from 'utils'

import type { TeamProps } from '#users/domain/entities/team.js'
import { TeamRepository } from '../domain/repositories/team.repository.js'
import { TeamId } from '../domain/entities/value-objects.js'

@Injectable()
export class GetTeamById implements UseCase<TeamId, TeamProps> {
  constructor(private readonly teamRepository: TeamRepository) {}

  execute = (id: TeamId): Task<TeamProps, AppError> => {
    return this.teamRepository.findOneById(id).map(team => team.toJSON())
  }
}
