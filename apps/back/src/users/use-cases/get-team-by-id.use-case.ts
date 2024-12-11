import { Injectable } from '@nestjs/common'
import { Team } from '@/users/domain/entities/team'
import { UseCase } from '@/utils/use-case'
import { TeamRepository } from '../domain/repositories/team.repository'
import { Task } from '@/utils/task'
import { AppError } from '@/utils/app-error'
import { TeamId } from '../domain/entities/value-objects'

@Injectable()
export class GetTeamById implements UseCase<TeamId, Team> {
  constructor(private readonly teamRepository: TeamRepository) {}

  execute(id: TeamId): Task<Team, AppError> {
    return this.teamRepository.findOneById(id)
  }
}
