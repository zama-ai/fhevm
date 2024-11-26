import { UserProps, User } from 'src/users/domain/entities/user'
import { UserRepository } from 'src/users/domain/repositories/user.repository'
import { PrismaService } from '../prisma.service'
import { Injectable } from '@nestjs/common'
import { Task } from 'src/utils/task'
import { AppError, notFound, unknown } from 'src/utils/app-error'

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
        .catch(err => reject(unknown(String(err))))
    }).chain(props => User.parse(props).asyncMap<User>(user => user))
  }

  findById(id: string): Task<User, AppError> {
    return new Task<UserProps, AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ where: { id } })
        .then(data =>
          data ? resolve(data) : reject(notFound('User not found')),
        )
        .catch(err => reject(unknown(String(err))))
    }).chain(props => User.parse(props).asyncMap<User>(user => user))
  }

  findByEmail(email: string): Task<User, AppError> {
    return new Task<UserProps, AppError>((resolve, reject) => {
      this.db.user
        .findFirst({ where: { email } })
        .then(data =>
          data ? resolve(data) : reject(notFound('User not found')),
        )
        .catch(err => reject(unknown(String(err))))
    }).chain(props => User.parse(props).asyncMap<User>(user => user))
  }
}
