import { User } from '#users/domain/entities/user.js'
import { UserRepository } from '#users/domain/repositories/user.repository.js'
import { PrismaService } from '../prisma.service.js'
import { Injectable } from '@nestjs/common'
import type { AppError } from 'utils'
import { notFoundError, Task, unknownError } from 'utils'
import { UserId } from '#users/domain/entities/value-objects.js'

@Injectable()
export class PrismaUserRepository extends UserRepository {
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
        .findFirst({ where: { id: id.value } })
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
        .findFirst({ where: { email } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('User not found')),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).async())
  }

  update(user: User): Task<User, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.user
        .update({
          where: { id: user.id.value },
          data: { ...user, id: undefined },
        })
        .then(resolve)
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).async())
  }
}
