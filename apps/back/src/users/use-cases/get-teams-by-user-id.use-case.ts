import { Injectable } from '@nestjs/common'
import { Team } from '@/users/domain/entities/team'
import { UseCase } from '@/utils/use-case'
import { TeamRepository } from '../domain/repositories/team.repository'
import { Task } from '@/utils/task'
import { AppError } from '@/utils/app-error'
import { UserId } from '../domain/entities/value-objects'

@Injectable()
export class GetTeamsByUserId implements UseCase<UserId, Team[]> {
  constructor(private readonly teamRepository: TeamRepository) {}

  execute(userId: UserId): Task<Team[], AppError> {
    return this.teamRepository.findManyByUserId(userId)
  }
}
