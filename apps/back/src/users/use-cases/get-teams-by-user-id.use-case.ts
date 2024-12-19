import { Inject, Injectable } from '@nestjs/common'
import { Team } from '@/users/domain/entities/team'
import type { AppError, Task, UnitOfWork, UseCase } from 'utils'
import { TeamRepository } from '../domain/repositories/team.repository'
import { UserId } from '../domain/entities/value-objects'
import { UNIT_OF_WORK } from '@/constants'

@Injectable()
export class GetTeamsByUserId implements UseCase<UserId, Team[]> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly teamRepository: TeamRepository,
  ) {}

  execute(userId: UserId): Task<Team[], AppError> {
    return this.uow.exec(this.teamRepository.findManyByUserId(userId))
  }
}
