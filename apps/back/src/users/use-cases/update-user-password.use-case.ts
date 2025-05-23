import { User } from '#users/domain/entities/user.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import {
  USER_REPOSITORY,
  UserRepository,
} from '#users/domain/repositories/user.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import {
  AppError,
  forbiddenError,
  Task,
  unauthorizedError,
  UseCase,
} from 'utils'

export type UpdateUserPasswordInput = {
  userId: string
  password: string
}

export type UpdateUserPasswordOutput = {
  user: User
}

export type IUpdateUserPassword = UseCase<
  UpdateUserPasswordInput,
  UpdateUserPasswordOutput
>
export const UPDATE_USER_PASSWORD = 'UPDATE_USER_PASSWORD'

@Injectable()
export class UpdateUserPassword implements IUpdateUserPassword {
  private readonly logger = new Logger(UpdateUserPassword.name)

  constructor(@Inject(USER_REPOSITORY) private readonly repo: UserRepository) {}
  execute = (
    { userId, password }: UpdateUserPasswordInput,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<UpdateUserPasswordOutput, AppError> => {
    this.logger.debug(`updating password for user ${userId}`)

    return UserId.from(userId)
      .asyncChain(this.repo.findById)
      .chain(user => {
        this.logger.debug(`updating password for user ${userId}/${user.email}`)
        return this.repo.update(user.id, {
          password: user.hashPassword(password).unwrap().value,
        })
      })
      .map(user => ({ user }))
  }
}

@Injectable()
export class UpdateUserPasswordWithAuthorization
  implements IUpdateUserPassword
{
  private readonly logger = new Logger(UpdateUserPasswordWithAuthorization.name)

  constructor(private readonly updateUserPassword: UpdateUserPassword) {}
  execute = (
    { userId, password }: UpdateUserPasswordInput,
    context?: Record<string, unknown>,
  ): Task<UpdateUserPasswordOutput, AppError> => {
    this.logger.debug(`check authorization for user ${userId}`)
    if (!(context?.user instanceof User)) {
      this.logger.warn(`no user in context`)
      return Task.reject(unauthorizedError())
    }

    if (context.user.id.value !== userId) {
      this.logger.warn(`wrong user in context`)
      return Task.reject(forbiddenError())
    }

    return this.updateUserPassword.execute({ userId, password }, context)
  }
}
