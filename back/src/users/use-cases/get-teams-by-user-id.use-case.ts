import { Injectable } from '@nestjs/common'
import { Team } from 'src/users/domain/entities/team'
import { UseCase } from 'src/utils/use-case'
import { TeamRepository } from '../domain/repositories/team.repository'
import { Task } from 'src/utils/task'
import { AppError } from 'src/utils/app-error'

@Injectable()
export class GetTeamsByUserId implements UseCase<string, Team[]> {
  constructor(private readonly teamRepository: TeamRepository) {}

  execute(userId: string): Task<Team[], AppError> {
    return this.teamRepository.findManyByUserId(userId)
  }
}
