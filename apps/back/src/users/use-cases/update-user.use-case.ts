import { Inject, Injectable } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import { type UseCase, AppError, ok, Task, unknownError } from 'utils'
import {
  USER_REPOSITORY,
  UserRepository,
} from '../domain/repositories/user.repository.js'
import { UserId } from '../domain/entities/value-objects.js'

type UpdateUserInput = {
  newUser: {
    id: string | UserId
    name?: string
    confirmedAt?: Date
  }
  // TODO: move user into context
  user: User
}

// TODO:
// - rename the file to `update-user.use-case.ts`
// - move user from input to context
@Injectable()
export class UpdateUser implements UseCase<UpdateUserInput, User> {
  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
  ) {}

  execute = ({ newUser, user }: UpdateUserInput): Task<User, AppError> => {
    return (
      typeof newUser.id === 'string'
        ? UserId.from(newUser.id)
        : ok<UserId, AppError>(user.id)
    )
      .asyncChain(newUserId =>
        this.userRepository
          .findById(user.id)
          .chain<User>(user =>
            user.isSome()
              ? Task.of(user.unwrap())
              : Task.reject(unknownError('User not found')),
          )
          .chain(user => {
            if (newUserId.equals(user.id)) {
              return Task.of(user)
            } else {
              return Task.reject<never, AppError>(
                unknownError('User not found'),
              )
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
