import { User } from '#users/domain/entities/user.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import {
  USER_REPOSITORY,
  UserRepository,
} from '#users/domain/repositories/user.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, ok, Task, UseCase } from 'utils'

type Input = {
  id: UserId | string
}

type Output = User

@Injectable()
export class ConfirmUser implements UseCase<Input, Output> {
  private readonly logger = new Logger(ConfirmUser.name)

  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
  ) {}

  execute(
    input: Input,
    context?: Record<string, unknown>,
  ): Task<User, AppError> {
    this.logger.debug(`confirming user ${input.id}`)
    return (
      typeof input.id === 'string'
        ? UserId.from(input.id)
        : ok<UserId, AppError>(input.id)
    ).asyncChain(id => {
      this.logger.verbose(`updating user ${id} confirmedAt`)
      return this.userRepository.update(id, { confirmedAt: new Date() })
    })
  }
}
