import { Injectable, Logger } from '@nestjs/common'
import type { AppError, Result } from 'utils'
import { notFoundError, unknownError, Task, ok, fail, isAppError } from 'utils'

import { Team } from '#teams/domain/entities/team.js'
import { TeamRepository } from '#teams/domain/repositories/team.repository.js'
import { TeamId } from '#teams/domain/entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { PrismaService } from '#infra/database/prisma.service.js'

@Injectable()
export class PrismaTeamRepository implements TeamRepository {
  private readonly logger = new Logger(PrismaTeamRepository.name)
  constructor(private readonly db: PrismaService) {}

  findOneById = (id: TeamId): Task<Team, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.team
        .findUnique({ where: { id: id.value, deletedAt: null } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Team not found')),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).async())
  }

  findManyByUserId = (userId: UserId): Task<Team[], AppError> => {
    return new Task<unknown[], AppError>((resolve, reject) => {
      this.db.user
        .findFirst({
          select: { teams: { where: { deletedAt: null } } },
          where: { id: userId.value, deletedAt: null },
        })
        .then(data =>
          data
            ? resolve(data.teams)
            : reject(notFoundError(`User ${userId.value} not found`)),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
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
        .findUnique({ where: { id: id.value, deletedAt: null } })
        .then(team => {
          if (team) {
            return this.db.team.update({
              where: { id: id.value },
              data: {
                users: {
                  connect: { id: userId.value },
                },
              },
            })
          } else {
            reject(notFoundError(`team not found`))
          }
        })
        .then(resolve)
        .catch((err: unknown) => {
          if (isAppError(err)) {
            this.logger.warn(`failed to add user: ${err._tag}/${err.message}`)
            reject(err)
          } else {
            this.logger.warn(`failed to add user: ${err}`)
            reject(unknownError(String(err)))
          }
        })
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
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).async())
  }

  findOneByIdAndUserId = (id: TeamId, userId: UserId): Task<Team, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.team
        .findFirst({
          where: {
            id: id.value,
            deletedAt: null,
            users: { some: { id: userId.value } },
          },
        })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Team not found')),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => Team.parse(props).async())
  }

  delete = (id: TeamId): Task<void, AppError> => {
    return new Task((resolve, reject) => {
      this.db.team
        .findUnique({
          where: { id: id.value, deletedAt: null },
        })
        .then(team => {
          if (team) {
            reject(notFoundError(`team not found`))
          } else {
            return this.db.team.update({
              data: { deletedAt: new Date() },
              where: { id: id.value },
            })
          }
        })
        .then(() => {
          this.logger.debug(`team deleted`)
          resolve(void 0)
        })
        .catch(error => {
          if (isAppError(error)) {
            this.logger.warn(
              `failed to delete team ${id.value}: ${error._tag}/${error.message}`,
            )
            return reject(error)
          } else {
            this.logger.warn(`failed to delete team ${id.value}: ${error}`)
            return reject(unknownError(String(error)))
          }
        })
    })
  }
}
