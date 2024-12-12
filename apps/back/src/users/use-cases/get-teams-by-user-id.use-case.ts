import { Injectable } from '@nestjs/common'
import { Team } from '@/users/domain/entities/team'
import type { AppError, Task, UseCase } from 'utils'
import { TeamRepository } from '../domain/repositories/team.repository'
import { UserId } from '../domain/entities/value-objects'

@Injectable()
export class GetTeamsByUserId implements UseCase<UserId, Team[]> {
  constructor(private readonly teamRepository: TeamRepository) {}

  execute(userId: UserId): Task<Team[], AppError> {
    return this.teamRepository.findManyByUserId(userId)
  }
}
