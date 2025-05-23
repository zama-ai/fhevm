import { Inject, Injectable } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import { type AppError, type UseCase, Task, unknownError } from 'utils'
import {
  USER_REPOSITORY,
  UserRepository,
} from '../domain/repositories/user.repository.js'
import { UserId } from '../domain/entities/value-objects.js'

interface Input {
  newUser: {
    name: string
    id: string
  }
  user: User
}

// TODO:
// - rename the file to `update-user.use-case.ts`
// - move user from input to context
@Injectable()
export class UpdateUser implements UseCase<Input, User> {
  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
  ) {}

  execute = ({ newUser, user }: Input): Task<User, AppError> => {
    return UserId.from(newUser.id)
      .asyncChain(newUserId =>
        this.userRepository.findById(user.id).chain(user => {
          if (newUserId.equals(user.id)) {
            return Task.of(user)
          } else {
            return Task.reject<never, AppError>(unknownError('User not found'))
          }
        }),
      )
      .chain(() => {
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        const { id, ...userProps } = user.toJSON()
        const { name } = newUser
        return this.userRepository.update(user.id, {
          ...userProps,
          name,
        })
      })
  }
}
