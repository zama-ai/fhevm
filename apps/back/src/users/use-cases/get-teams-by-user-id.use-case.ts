import { Inject, Injectable } from '@nestjs/common'
import { type TeamProps } from '#users/domain/entities/team.js'
import type { AppError, Task, UnitOfWork, UseCase } from 'utils'
import { TeamRepository } from '../domain/repositories/team.repository.js'
import { UserId } from '../domain/entities/value-objects.js'
import { UNIT_OF_WORK } from '#constants.js'

@Injectable()
export class GetTeamsByUserId implements UseCase<UserId, TeamProps[]> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly teamRepository: TeamRepository,
  ) {}

  execute = (userId: UserId): Task<TeamProps[], AppError> => {
    return this.uow.exec(
      this.teamRepository
        .findManyByUserId(userId)
        .map(teams => teams.map(team => team.toJSON())),
    )
  }
}
