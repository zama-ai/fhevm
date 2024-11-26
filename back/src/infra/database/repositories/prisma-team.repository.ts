import { TeamProps, Team } from '@/users/domain/entities/team'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { PrismaService } from '../prisma.service'
import { Injectable } from '@nestjs/common'
import { Task } from '@/utils/task'
import { AppError, notFound, unknown } from '@/utils/app-error'

@Injectable()
export class PrismaTeamRepository extends TeamRepository {
  constructor(private readonly db: PrismaService) {
    super()
  }

  findById(id: string): Task<Team, AppError> {
    return new Task<TeamProps, AppError>((resolve, reject) => {
      this.db.team
        .findFirst({ where: { id } })
        .then(data =>
          data ? resolve(data) : reject(notFound('User not found')),
        )
        .catch(err => reject(unknown(String(err))))
    }).chain(props => Team.parse(props).asyncMap(team => team))
  }

  findManyByUserId(userId: string): Task<Team[], AppError> {
    return new Task<TeamProps[], AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ select: { teams: true }, where: { id: userId } })
        .then(data =>
          data ? resolve(data.teams) : reject(notFound('User not found')),
        )
        .catch(err => reject(unknown(String(err))))
    }).chain(props => Team.parseArray(props).asyncMap(teams => teams))
  }
}
