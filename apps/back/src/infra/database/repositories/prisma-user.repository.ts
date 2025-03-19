import { User } from '#users/domain/entities/user.js'
import { UserRepository } from '#users/domain/repositories/user.repository.js'
import { PrismaService } from '../prisma.service.js'
import { Injectable, Logger } from '@nestjs/common'
import { AppError, isAppError } from 'utils'
import { notFoundError, Task, unknownError } from 'utils'
import { UserId } from '#users/domain/entities/value-objects.js'

@Injectable()
export class PrismaUserRepository extends UserRepository {
  private readonly logger = new Logger(PrismaUserRepository.name)
  constructor(private readonly db: PrismaService) {
    super()
  }

  create = (data: User): Task<User, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.user
        .create({ data: data.toJSON() })
        .then(resolve)
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).async())
  }

  findById = (id: UserId): Task<User, AppError> => {
    if (!id) {
      return Task.reject(notFoundError('User not found'))
    }
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ where: { id: id.value, deletedAt: null } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('User not found')),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).async())
  }

  findByEmail = (email: string): Task<User, AppError> => {
    if (!email) {
      return Task.reject(notFoundError('User not found'))
    }
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ where: { email, deletedAt: null } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('User not found')),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).async())
  }

  update(id: UserId, data: Partial<Omit<User, 'id'>>): Task<User, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.user
        .update({
          where: { id: id.value },
          data,
        })
        .then(resolve)
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).async())
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
