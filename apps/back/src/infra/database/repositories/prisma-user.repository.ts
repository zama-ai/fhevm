import { User, UserProps } from '#users/domain/entities/user.js'
import { UserRepository } from '#users/domain/repositories/user.repository.js'
import { PrismaService } from '../prisma.service.js'
import { Injectable, Logger } from '@nestjs/common'
import { AppError, isAppError, none, Option, shortString, some } from 'utils'
import { notFoundError, Task, unknownError } from 'utils'
import { UserId } from '#users/domain/entities/value-objects.js'
import { Email } from '#shared/entities/value-objects/email.js'

@Injectable()
export class PrismaUserRepository implements UserRepository {
  private readonly logger = new Logger(PrismaUserRepository.name)
  constructor(private readonly db: PrismaService) {}

  create = (data: User): Task<User, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.user
        .create({ data: data.toJSON() })
        .then(resolve)
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).async())
  }

  findById = (id: UserId): Task<Option<User>, AppError> => {
    if (!id) {
      return Task.reject(notFoundError('User not found'))
    }
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.user
        .findUnique({ where: { id: id.value, deletedAt: null } })
        .then(data => {
          this.logger.verbose(
            `found user by id: ${JSON.stringify(data, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
          )
          return resolve(data)
        })
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props =>
      props
        ? User.parse(props).map<Option<User>>(some).async()
        : Task.of(none()),
    )
  }

  findByEmail = (email: Email): Task<Option<User>, AppError> => {
    if (!email) {
      return Task.reject(notFoundError('User not found'))
    }
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ where: { email: email.value, deletedAt: null } })
        .then(data => {
          if (data) {
            this.logger.verbose(
              `found user by email: ${JSON.stringify(data, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
            )
          } else {
            this.logger.verbose(`user ${email} not found`)
          }
          return resolve(data)
        })
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props =>
      props
        ? User.parse(props).map<Option<User>>(some).async()
        : Task.of(none()),
    )
  }

  update = (
    id: UserId,
    data: Partial<Omit<UserProps, 'id'>>,
  ): Task<User, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.logger.debug(
        `updating user ${id} with ${JSON.stringify(data, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
      )
      this.db.user
        .update({
          where: { id: id.value },
          data,
        })
        .then(resolve)
        .catch((err: unknown) => {
          this.logger.warn(`failed to update: ${err}`)
          reject(unknownError(String(err)))
        })
    }).chain(props => {
      this.logger.verbose(
        `parsing user: ${JSON.stringify(props, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
      )
      return User.parse(props).async()
    })
  }

  delete = (id: UserId): Task<void, AppError> => {
    return new Task<void, AppError>((resolve, reject) => {
      this.db.user
        .findUnique({
          where: { id: id.value, deletedAt: null },
        })
        .then(user => {
          if (!user) {
            reject(notFoundError(`user not found`))
          } else {
            return this.db.user.update({
              where: { id: id.value, deletedAt: null },
              data: {
                deletedAt: new Date().toISOString(),
              },
            })
          }
        })
        .then(() => {
          resolve(void 0)
        })
        .catch((err: unknown) => {
          if (isAppError(err)) {
            this.logger.warn(`failed to delete: ${err._tag}/${err.message}`)
            reject(err)
          } else {
            this.logger.warn(`failed to delete: ${err}`)
            reject(unknownError(String(err)))
          }
        })
    }).tap(() => {
      this.logger.debug(`user ${id.value} deleted`)
    })
  }
}
