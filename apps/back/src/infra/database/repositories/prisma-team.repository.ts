import { Injectable } from '@nestjs/common'

import { Team } from '@/users/domain/entities/team'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import type { AppError } from 'utils'
import { notFoundError, unknownError, Task } from 'utils'
import { TeamId, UserId } from '@/users/domain/entities/value-objects'
import { PrismaService } from '../prisma.service'

@Injectable()
export class PrismaTeamRepository extends TeamRepository {
  constructor(private readonly db: PrismaService) {
    super()
  }

  findOneById(id: TeamId): Task<Team, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.team
        .findFirst({ where: { id: id.value } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('User not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).async())
  }

  findManyByUserId(userId: UserId): Task<Team[], AppError> {
    return new Task<unknown[], AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ select: { teams: true }, where: { id: userId.value } })
        .then(data =>
          data ? resolve(data.teams) : reject(notFoundError('User not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Team.parseArray(props).async())
  }

  addUser(id: TeamId, userId: UserId): Task<Team, AppError> {
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
  create(id: TeamId, name: string): Task<Team, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.team
        .create({
          data: {
            id: id.value,
            name,
          },
        })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).async())
  }
  findOneByIdAndUserId(id: TeamId, userId: UserId): Task<Team, AppError> {
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
