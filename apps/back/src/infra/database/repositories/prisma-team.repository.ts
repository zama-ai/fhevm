import { TeamProps, Team } from '@/users/domain/entities/team'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { PrismaService } from '../prisma.service'
import { Injectable } from '@nestjs/common'
import { Task } from '@/utils/task'
import { AppError, notFoundError, unknownError } from '@/utils/app-error'
import { TeamId, UserId } from '@/users/domain/entities/value-objects'

@Injectable()
export class PrismaTeamRepository extends TeamRepository {
  constructor(private readonly db: PrismaService) {
    super()
  }

  findById(id: TeamId): Task<Team, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.team
        .findFirst({ where: { id: id.value } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('User not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).asyncMap(team => team))
  }

  findManyByUserId(userId: UserId): Task<Team[], AppError> {
    return new Task<unknown[], AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ select: { teams: true }, where: { id: userId.value } })
        .then(data =>
          data ? resolve(data.teams) : reject(notFoundError('User not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Team.parseArray(props).asyncMap(teams => teams))
  }
}
