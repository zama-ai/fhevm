import { User } from '#users/domain/entities/user.js'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import {
  USER_REPOSITORY,
  UserRepository,
} from '#users/domain/repositories/user.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

type Input = {
  name: string
  email: string
  password: ValidatedPassword
}

@Injectable()
export class CreateUser implements UseCase<Input, User> {
  private readonly logger = new Logger(CreateUser.name)

  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
  ) {}

  execute = (
    input: Input,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<User, AppError> => {
    this.logger.log(`creating user ${input.name} with email ${input.email}`)

    return User.create(input).asyncChain(this.userRepository.create)
  }
}
