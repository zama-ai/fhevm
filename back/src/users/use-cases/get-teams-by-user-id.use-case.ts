import { Injectable } from '@nestjs/common'
import { Team } from '@/users/domain/entities/team'
import { UseCase } from '@/utils/use-case'
import { TeamRepository } from '../domain/repositories/team.repository'
import { Task } from '@/utils/task'
import { AppError } from '@/utils/app-error'

@Injectable()
export class GetTeamsByUserId implements UseCase<string, Team[]> {
  constructor(private readonly teamRepository: TeamRepository) {}

  execute(userId: string): Task<Team[], AppError> {
    return this.teamRepository.findManyByUserId(userId)
  }
}
