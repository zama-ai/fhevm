import { User } from '#users/domain/entities/user.js'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import {
  USER_REPOSITORY,
  UserRepository,
} from '#users/domain/repositories/user.repository.js'
import { Inject, Injectable } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

type Input = {
  name: string
  email: string
  password: ValidatedPassword
}

@Injectable()
export class CreateUser implements UseCase<Input, User> {
  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
  ) {}

  execute = (
    input: Input,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<User, AppError> => {
    return User.create(input).asyncChain(this.userRepository.create)
  }
}
