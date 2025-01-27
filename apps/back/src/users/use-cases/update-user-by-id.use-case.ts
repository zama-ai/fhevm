import { Inject, Injectable } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import {
  type AppError,
  type UseCase,
  type UnitOfWork,
  Task,
  unknownError,
} from 'utils'
import { UserRepository } from '../domain/repositories/user.repository.js'
import { UNIT_OF_WORK } from '#constants.js'
import { UserId } from '../domain/entities/value-objects.js'

interface Input {
  newUser: {
    name: string
    id: `user_${string}`
  }
  user: User
}

@Injectable()
export class UpdateUser implements UseCase<Input, User> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly userRepository: UserRepository,
  ) {}

  execute({ newUser, user }: Input): Task<User, AppError> {
    return this.uow.exec(
      this.userRepository
        .findById(user.id)
        .chain(user => {
          if (user && user.id === UserId.from(newUser.id)) {
            return Task.of(user)
          } else {
            return Task.reject<never, AppError>(unknownError('User not found'))
          }
        })
        .chain(() => {
          const { id, ...userProps } = user
          const { name } = newUser
          return this.userRepository.update(id, {
            ...userProps,
            name,
          })
        }),
    )
  }
}
