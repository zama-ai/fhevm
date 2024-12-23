import { Team } from '@/users/domain/entities/team'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { PrismaService } from '../prisma.service'
import { Injectable } from '@nestjs/common'
import type { AppError, Result } from 'utils'
import { notFoundError, unknownError, Task, ok, fail } from 'utils'
import { TeamId, UserId } from '@/users/domain/entities/value-objects'

@Injectable()
export class PrismaTeamRepository extends TeamRepository {
  constructor(private readonly db: PrismaService) {
    super()
  }

  findOneById = (id: TeamId): Task<Team, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.team
        .findFirst({ where: { id: id.value } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Team not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).async())
  }

  findManyByUserId = (userId: UserId): Task<Team[], AppError> => {
    return new Task<unknown[], AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ select: { teams: true }, where: { id: userId.value } })
        .then(data =>
          data
            ? resolve(data.teams)
            : reject(notFoundError(`User ${userId.value} not found`)),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props =>
      props
        .map(Team.parse)
        .reduce(
          (acc, team) => {
            if (acc.isFail()) return acc
            return team.isOk()
              ? ok([...acc.value, team.value])
              : (fail(team.error) as Result<Team[], AppError>)
          },
          ok([]) as Result<Team[], AppError>,
        )
        .async(),
    )
  }

  addUser = (id: TeamId, userId: UserId): Task<Team, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.team
        .update({
          where: { id: id.value },
          data: {
            users: {
              connect: { id: userId.value },
            },
          },
        })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).async())
  }

  create = (team: Team): Task<Team, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.team
        .create({
          data: {
            id: team.id.value,
            name: team.name,
          },
        })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).async())
  }

  findOneByIdAndUserId = (id: TeamId, userId: UserId): Task<Team, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.team
        .findFirst({
          where: { id: id.value, users: { some: { id: userId.value } } },
        })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Team not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).async())
  }
}
