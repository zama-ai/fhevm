import { UserProps, User } from '@/users/domain/entities/user'
import { UserRepository } from '@/users/domain/repositories/user.repository'
import { PrismaService } from '../prisma.service'
import { Injectable } from '@nestjs/common'
import { Task } from '@/utils/task'
import { AppError, notFoundError, unknownError } from '@/utils/app-error'

@Injectable()
export class PrismaUserRepository extends UserRepository {
  constructor(private readonly db: PrismaService) {
    super()
  }

  create(data: UserProps): Task<User, AppError> {
    return new Task<UserProps, AppError>((resolve, reject) => {
      this.db.user
        .create({ data })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).asyncMap<User>(user => user))
  }

  findById(id: string): Task<User, AppError> {
    return new Task<UserProps, AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ where: { id } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('User not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).asyncMap<User>(user => user))
  }

  findByEmail(email: string): Task<User, AppError> {
    return new Task<UserProps, AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ where: { email } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('User not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => User.parse(props).asyncMap<User>(user => user))
  }
}
